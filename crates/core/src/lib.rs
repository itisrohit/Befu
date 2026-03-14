use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod demo_commands;

type CommandHandler = fn(&BridgeRequest) -> BridgeResponse;

const COMMANDS: [(&str, CommandHandler); 2] =
    [("ping", ping_command), ("app.info", app_info_command)];
const DEMO_COMMANDS: [(&str, CommandHandler); 1] = [("hello", demo_commands::hello_command)];

#[derive(Debug, Deserialize)]
pub struct BridgeRequest {
    pub id: String,
    pub command: String,
    #[serde(default)]
    pub args: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct BridgeError {
    pub code: &'static str,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum BridgeResponse {
    Success { id: String, ok: bool, result: Value },
    Failure { id: String, ok: bool, error: BridgeError },
}

pub fn ping() -> &'static str {
    "pong"
}

pub fn app_info() -> Value {
    serde_json::json!({
        "name": "Befu",
        "version": env!("CARGO_PKG_VERSION"),
        "runtime": "befu"
    })
}

fn find_command(command: &str) -> Option<CommandHandler> {
    COMMANDS
        .iter()
        .chain(DEMO_COMMANDS.iter())
        .find(|(name, _)| *name == command)
        .map(|(_, handler)| *handler)
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
