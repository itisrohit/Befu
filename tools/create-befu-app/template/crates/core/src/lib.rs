pub use befu_bridge::{
    BridgeError, BridgeRequest, BridgeResponse, CommandHandler, CommandMetadata, CommandRegistry,
    RegisteredCommand, failure_response, success_response,
};
pub use befu_macros::command;

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use serde_json::Value;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock};

mod demo_commands;
mod hot_reload;

fn init_registry() -> CommandRegistry {
    let mut registry = CommandRegistry::new();

    // Register default commands
    registry.register(
        CommandMetadata { name: "ping", description: "Connectivity check" },
        ping_command,
    );
    registry.register(
        CommandMetadata { name: "app.info", description: "Get application metadata" },
        app_info_command,
    );

    registry.register(
        CommandMetadata { name: "befu.commands", description: "List all registered commands" },
        list_commands_command,
    );

    registry.register(
        CommandMetadata { name: "befu.reload", description: "Reload hot-reloadable components" },
        reload_commands_command,
    );

    // Register local demo commands (fallback)
    befu_macros::register_commands!(registry, demo_commands::hello);

    // Load external hot-reloadable commands
    #[cfg(debug_assertions)]
    hot_reload::load_external_commands(&mut registry);

    registry
}

static REGISTRY: OnceLock<Mutex<CommandRegistry>> = OnceLock::new();

fn get_registry_lock() -> &'static Mutex<CommandRegistry> {
    REGISTRY.get_or_init(|| Mutex::new(init_registry()))
}

/// Lightweight health command used for connectivity checks.
pub fn ping() -> &'static str {
    "pong"
}

/// Returns static runtime metadata exposed to bridge callers.
pub fn app_info() -> Value {
    serde_json::json!({
        "name": "Befu",
        "version": env!("CARGO_PKG_VERSION"),
        "runtime": "befu",
        "hot_reload": cfg!(debug_assertions)
    })
}

fn find_command(command: &str) -> Option<CommandHandler> {
    let reg = get_registry_lock().lock().unwrap_or_else(|e| e.into_inner());
    reg.find(command).map(|c| c.handler)
}

fn list_commands_command(request: &BridgeRequest) -> BridgeResponse {
    let reg = get_registry_lock().lock().unwrap_or_else(|e| e.into_inner());
    success_response(&request.id, serde_json::to_value(reg.list_metadata()).unwrap_or_default())
}

fn reload_commands_command(request: &BridgeRequest) -> BridgeResponse {
    let mut reg = get_registry_lock().lock().unwrap_or_else(|e| e.into_inner());
    *reg = init_registry();
    success_response(&request.id, Value::Bool(true))
}

pub fn handle_request(payload: &str) -> String {
    let req: BridgeRequest = match serde_json::from_str(payload) {
        Ok(r) => r,
        Err(e) => {
            return serde_json::to_string(&failure_response(
                "",
                "INVALID_JSON",
                e.to_string(),
                None,
            ))
            .unwrap_or_default();
        }
    };

    let response = if let Some(handler) = find_command(&req.command) {
        handler(&req)
    } else {
        failure_response(&req.id, "NOT_FOUND", format!("Command not found: {}", req.command), None)
    };

    serde_json::to_string(&response).unwrap_or_else(|e| {
        serde_json::to_string(&failure_response("", "SERIALIZATION_ERROR", e.to_string(), None))
            .unwrap_or_default()
    })
}

fn ping_command(req: &BridgeRequest) -> BridgeResponse {
    success_response(&req.id, serde_json::json!({ "pong": "pong" }))
}

fn app_info_command(req: &BridgeRequest) -> BridgeResponse {
    success_response(&req.id, app_info())
}

/// Invokes a bridge command using a raw JSON string payload.
///
/// # Safety
/// The `payload` must be a valid, null-terminated C string. The caller is responsible
/// for freeing the returned string using `befu_free_raw`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn befu_invoke_raw(payload: *const c_char) -> *mut c_char {
    if payload.is_null() {
        return std::ptr::null_mut();
    }

    let bus_payload = match unsafe { CStr::from_ptr(payload).to_str() } {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let out = handle_request(bus_payload);
    match CString::new(out) {
        Ok(cs) => cs.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Frees a string returned by `befu_invoke_raw`.
///
/// # Safety
/// The `ptr` must be a pointer that was previously returned by `befu_invoke_raw`
/// and has not been freed yet.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn befu_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = unsafe { CString::from_raw(ptr) };
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_befu_app_BefuNativeBridge_invokeRawNative(
    mut env: JNIEnv,
    _class: JClass,
    payload: JString,
) -> jstring {
    let input: String = match env.get_string(&payload) {
        Ok(s) => s.into(),
        Err(_) => return std::ptr::null_mut(),
    };

    let output = handle_request(&input);

    match env.new_string(output) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}
