pub use befu_macros::command;
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod demo_commands;

type CommandHandler = fn(&BridgeRequest) -> BridgeResponse;

#[derive(Debug, Clone, Serialize)]
pub struct CommandMetadata {
    pub name: &'static str,
    pub description: &'static str,
}

pub struct RegisteredCommand {
    pub metadata: CommandMetadata,
    pub handler: CommandHandler,
}

pub struct CommandRegistry {
    commands: std::collections::HashMap<String, RegisteredCommand>,
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut registry = Self { commands: std::collections::HashMap::new() };

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

        // Register demo commands
        befu_macros::register_commands!(registry, demo_commands::hello);

        registry
    }

    pub fn register(&mut self, metadata: CommandMetadata, handler: CommandHandler) {
        self.commands.insert(metadata.name.to_string(), RegisteredCommand { metadata, handler });
    }

    pub fn find(&self, name: &str) -> Option<&RegisteredCommand> {
        self.commands.get(name)
    }

    pub fn list_metadata(&self) -> Vec<CommandMetadata> {
        self.commands.values().map(|c| c.metadata.clone()).collect()
    }
}

use std::sync::OnceLock;
static REGISTRY: OnceLock<CommandRegistry> = OnceLock::new();

fn get_registry() -> &'static CommandRegistry {
    REGISTRY.get_or_init(CommandRegistry::new)
}

#[derive(Debug, Deserialize)]
/// Incoming bridge request payload from web/native shells.
pub struct BridgeRequest {
    pub id: String,
    pub command: String,
    #[serde(default)]
    pub args: Option<Value>,
}

#[derive(Debug, Serialize)]
/// Structured bridge error payload.
pub struct BridgeError {
    pub code: &'static str,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
/// Envelope for command success/failure responses.
pub enum BridgeResponse {
    Success { id: String, ok: bool, result: Value },
    Failure { id: String, ok: bool, error: BridgeError },
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
        "runtime": "befu"
    })
}

fn find_command(command: &str) -> Option<CommandHandler> {
    get_registry().find(command).map(|c| c.handler)
}

fn list_commands_command(request: &BridgeRequest) -> BridgeResponse {
    success_response(
        &request.id,
        serde_json::to_value(get_registry().list_metadata()).unwrap_or_default(),
    )
}

pub(crate) fn success_response(id: &str, result: Value) -> BridgeResponse {
    BridgeResponse::Success { id: id.to_owned(), ok: true, result }
}

pub(crate) fn failure_response(
    id: &str,
    code: &'static str,
    message: impl Into<String>,
    details: Option<Value>,
) -> BridgeResponse {
    BridgeResponse::Failure {
        id: id.to_owned(),
        ok: false,
        error: BridgeError { code, message: message.into(), details },
    }
}

fn ping_command(request: &BridgeRequest) -> BridgeResponse {
    success_response(&request.id, serde_json::json!({ "pong": ping() }))
}

fn app_info_command(request: &BridgeRequest) -> BridgeResponse {
    success_response(&request.id, app_info())
}

/// Deserializes, dispatches, and serializes a bridge command request.
pub fn handle_request(request_json: &str) -> Result<String, serde_json::Error> {
    let request: BridgeRequest = serde_json::from_str(request_json)?;

    let response = match find_command(&request.command) {
        Some(handler) => handler(&request),
        None => failure_response(
            &request.id,
            "UNKNOWN_COMMAND",
            format!("Unknown command: {}", request.command),
            request.args.clone(),
        ),
    };

    serde_json::to_string(&response)
}

#[unsafe(no_mangle)]
/// # Safety
///
/// `payload_json` must be a valid, null-terminated C string pointer.
/// The returned pointer must be released by calling `befu_free_string`.
pub unsafe extern "C" fn befu_invoke_raw(payload_json: *const c_char) -> *mut c_char {
    if payload_json.is_null() {
        return c_string_ptr(
            "{\"id\":\"\",\"ok\":false,\"error\":{\"code\":\"INVALID_ARGUMENT\",\"message\":\"payload_json is null\"}}",
        );
    }

    let input = unsafe { CStr::from_ptr(payload_json) }.to_str();
    let response = match input {
        Ok(request) => handle_request(request).unwrap_or_else(|error| {
            format!(
                "{{\"id\":\"\",\"ok\":false,\"error\":{{\"code\":\"INVALID_JSON\",\"message\":\"{}\"}}}}",
                error
            )
        }),
        Err(error) => format!(
            "{{\"id\":\"\",\"ok\":false,\"error\":{{\"code\":\"INVALID_UTF8\",\"message\":\"{}\"}}}}",
            error
        ),
    };

    c_string_ptr(&response)
}

#[unsafe(no_mangle)]
/// # Safety
///
/// `value` must be a pointer previously returned by `befu_invoke_raw` and not
/// already freed.
pub unsafe extern "C" fn befu_free_string(value: *mut c_char) {
    if value.is_null() {
        return;
    }

    unsafe {
        let _ = CString::from_raw(value);
    }
}

fn c_string_ptr(value: &str) -> *mut c_char {
    let safe = value.replace('\0', "");
    CString::new(safe)
        .expect("CString::new should not fail after null-byte sanitization")
        .into_raw()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_befu_app_BefuNativeBridge_invokeRawNative(
    mut env: JNIEnv,
    _class: JClass,
    payload_json: JString,
) -> jstring {
    let response = match env.get_string(&payload_json) {
        Ok(value) => match handle_request(value.to_str().unwrap_or_default()) {
            Ok(result) => result,
            Err(error) => format!(
                "{{\"id\":\"\",\"ok\":false,\"error\":{{\"code\":\"INVALID_JSON\",\"message\":\"{}\"}}}}",
                error
            ),
        },
        Err(error) => format!(
            "{{\"id\":\"\",\"ok\":false,\"error\":{{\"code\":\"JNI_INPUT_ERROR\",\"message\":\"{}\"}}}}",
            error
        ),
    };

    match env.new_string(response) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}
