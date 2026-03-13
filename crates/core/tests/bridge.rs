use befu_core::{handle_request, ping};
use serde_json::{Value, json};

#[test]
fn returns_pong() {
    assert_eq!(ping(), "pong");
}

#[test]
fn handles_ping_request() {
    let response = handle_request(r#"{"id":"1","command":"ping"}"#)
        .expect("request should serialize to response");
    let parsed: Value = serde_json::from_str(&response).expect("valid json response");

    assert_eq!(parsed, json!({ "id": "1", "ok": true, "result": { "pong": "pong" } }));
}

#[test]
fn handles_app_info_request() {
    let response =
        handle_request(r#"{"id":"2","command":"app.info"}"#).expect("serializable response");
    let parsed: Value = serde_json::from_str(&response).expect("valid json response");

    assert_eq!(parsed["id"], "2");
    assert_eq!(parsed["ok"], true);
    assert_eq!(parsed["result"]["name"], "Befu");
    assert_eq!(parsed["result"]["runtime"], "befu");
}

#[test]
fn handles_unknown_command() {
    let response = handle_request(r#"{"id":"3","command":"nope","args":{"foo":"bar"}}"#)
        .expect("request should serialize to response");
    let parsed: Value = serde_json::from_str(&response).expect("valid json response");

    assert_eq!(parsed["id"], "3");
    assert_eq!(parsed["ok"], false);
    assert_eq!(parsed["error"]["code"], "UNKNOWN_COMMAND");
    assert_eq!(parsed["error"]["details"]["foo"], "bar");
}
