use befu_core::{handle_request, ping};
use serde_json::{Value, json};

#[test]
fn returns_pong() {
    assert_eq!(ping(), "pong");
}

#[test]
fn handles_ping_request() {
    let response = handle_request(r#"{"id":"1","command":"ping"}"#);
    let parsed: Value = serde_json::from_str(&response).expect("valid json response");

    assert_eq!(parsed, json!({ "id": "1", "ok": true, "result": { "pong": "pong" } }));
}

#[test]
fn handles_app_info_request() {
    let response = handle_request(r#"{"id":"2","command":"app.info"}"#);
    let parsed: Value = serde_json::from_str(&response).expect("valid json response");

    assert_eq!(parsed["id"], "2");
    assert_eq!(parsed["ok"], true);
    assert_eq!(parsed["result"]["name"], "Befu");
    assert_eq!(parsed["result"]["runtime"], "befu");
}

#[test]
fn handles_unknown_command() {
    let response = handle_request(r#"{"id":"3","command":"nope","args":{"foo":"bar"}}"#);
    let parsed: Value = serde_json::from_str(&response).expect("valid json response");

    assert_eq!(parsed["id"], "3");
    assert_eq!(parsed["ok"], false);
    assert_eq!(parsed["error"]["code"], "NOT_FOUND");
}

#[test]
fn handles_hello_command() {
    let response = handle_request(r#"{"id":"4","command":"hello","args":{"name":"Developer"}}"#);
    let parsed: Value = serde_json::from_str(&response).expect("valid json response");

    assert_eq!(
        parsed,
        json!({ "id": "4", "ok": true, "result": { "message": "Hello Developer" } })
    );
}

#[test]
fn rejects_hello_command_without_args() {
    let response = handle_request(r#"{"id":"5","command":"hello"}"#);
    let parsed: Value = serde_json::from_str(&response).expect("valid json response");

    assert_eq!(parsed["id"], "5");
    assert_eq!(parsed["ok"], false);
    assert_eq!(parsed["error"]["code"], "INVALID_ARGUMENT");
}

#[test]
fn rejects_hello_command_with_invalid_args_shape() {
    let response = handle_request(r#"{"id":"6","command":"hello","args":{"unexpected":true}}"#);
    let parsed: Value = serde_json::from_str(&response).expect("valid json response");

    assert_eq!(parsed["id"], "6");
    assert_eq!(parsed["ok"], false);
    assert_eq!(parsed["error"]["code"], "INVALID_ARGUMENT");
}
