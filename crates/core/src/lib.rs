use serde::{Deserialize, Serialize};
use serde_json::Value;

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

pub fn handle_request(request_json: &str) -> Result<String, serde_json::Error> {
    let request: BridgeRequest = serde_json::from_str(request_json)?;

    let response = match request.command.as_str() {
        "ping" => BridgeResponse::Success {
            id: request.id,
            ok: true,
            result: serde_json::json!({ "pong": ping() }),
        },
        "app.info" => BridgeResponse::Success { id: request.id, ok: true, result: app_info() },
        other => BridgeResponse::Failure {
            id: request.id,
            ok: false,
            error: BridgeError {
                code: "UNKNOWN_COMMAND",
                message: format!("Unknown command: {other}"),
                details: request.args,
            },
        },
    };

    serde_json::to_string(&response)
}
