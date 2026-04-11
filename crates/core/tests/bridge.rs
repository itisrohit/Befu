use befu_core::{handle_request, ping};
use serde_json::{Value, json};

// ── Complex struct bridge tests (Phase 6) ───────────────────────

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

    assert_eq!(parsed["id"], "4");
    assert_eq!(parsed["ok"], true);
    let message = parsed["result"]["message"].as_str().expect("message must be a string");
    assert!(
        message.starts_with("Hello Developer"),
        "Expected message to start with 'Hello Developer', got: {}",
        message
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

// ── Phase 6: Complex nested struct tests ────────────────────────

/// Deeply nested JSON object round-trips through the bridge losslessly.
#[test]
fn handles_user_profile_echo_with_deeply_nested_json() {
    let profile = json!({
        "id": 42,
        "name": "Alice Nakamura",
        "email": "alice@example.com",
        "active": true,
        "tags": ["rust", "wasm", "mobile", "oss"],
        "scores": [95.5, 87.0, 92.3, 100.0],
        "address": {
            "street": "1-2-3 Shibuya",
            "city": "Tokyo",
            "country": "JP",
            "coords": {
                "lat": 35.6595,
                "lng": 139.7004
            }
        },
        "metadata": {
            "role": "engineer",
            "level": 5,
            "active_projects": ["befu", "wasm-bridge"],
            "preferences": { "theme": "dark", "notifications": true }
        }
    });

    let request = json!({
        "id": "nested-1",
        "command": "user.profile.echo",
        "args": { "profile": profile }
    });

    let response = handle_request(&serde_json::to_string(&request).unwrap());
    let parsed: Value = serde_json::from_str(&response).expect("valid json response");

    assert_eq!(parsed["id"], "nested-1");
    assert_eq!(parsed["ok"], true);

    // Profile echoed back identically
    assert_eq!(parsed["result"]["profile"], profile);

    // Computed fields are correct
    assert_eq!(parsed["result"]["computed"]["tag_count"], 4);
    let avg = parsed["result"]["computed"]["avg_score"].as_f64().unwrap();
    assert!((avg - 93.7).abs() < 0.01, "Expected avg ~93.7, got {avg}");
}

/// Vec of nested structs with nested_tags (Vec<Vec<String>>) round-trips correctly.
#[test]
fn handles_data_aggregate_with_nested_arrays() {
    let items = json!([
        {
            "label": "Widget A",
            "value": 29.95,
            "category": "hardware",
            "nested_tags": [["sensor", "iot"], ["v2", "stable"]]
        },
        {
            "label": "Widget B",
            "value": 15.50,
            "category": "software",
            "nested_tags": [["cli"]]
        },
        {
            "label": "Widget C",
            "value": 44.55,
            "category": "hardware",
            "nested_tags": [["sensor"], ["beta", "experimental", "v3"]]
        }
    ]);

    let request = json!({
        "id": "nested-2",
        "command": "data.aggregate",
        "args": { "items": items }
    });

    let response = handle_request(&serde_json::to_string(&request).unwrap());
    let parsed: Value = serde_json::from_str(&response).expect("valid json response");

    assert_eq!(parsed["id"], "nested-2");
    assert_eq!(parsed["ok"], true);

    assert_eq!(parsed["result"]["count"], 3);
    let total = parsed["result"]["total"].as_f64().unwrap();
    assert!((total - 90.0).abs() < 0.01, "Expected total ~90.0, got {total}");
    let avg = parsed["result"]["average"].as_f64().unwrap();
    assert!((avg - 30.0).abs() < 0.01, "Expected avg ~30.0, got {avg}");

    // Categories deduplicated and sorted
    assert_eq!(parsed["result"]["categories"], json!(["hardware", "software"]));

    // Items preserved with nested arrays intact
    assert_eq!(parsed["result"]["items"], items);
}

/// Rejects user.profile.echo when a required nested field is missing.
#[test]
fn rejects_user_profile_echo_with_missing_nested_field() {
    let request = json!({
        "id": "nested-3",
        "command": "user.profile.echo",
        "args": {
            "profile": {
                "id": 1,
                "name": "Bob",
                "email": "bob@example.com",
                "active": true,
                "tags": [],
                "scores": [],
                "address": {
                    "street": "456 Oak Ave",
                    "city": "Portland",
                    "country": "US"
                    // coords intentionally missing
                },
                "metadata": {}
            }
        }
    });

    let response = handle_request(&serde_json::to_string(&request).unwrap());
    let parsed: Value = serde_json::from_str(&response).expect("valid json response");

    assert_eq!(parsed["id"], "nested-3");
    assert_eq!(parsed["ok"], false);
    assert_eq!(parsed["error"]["code"], "INVALID_ARGUMENT");
    let msg = parsed["error"]["message"].as_str().unwrap();
    assert!(msg.contains("coords"), "Error should mention missing 'coords' field, got: {msg}");
}

/// Empty items array is handled gracefully with zero averages.
#[test]
fn handles_data_aggregate_with_empty_items() {
    let request = json!({
        "id": "nested-4",
        "command": "data.aggregate",
        "args": { "items": [] }
    });

    let response = handle_request(&serde_json::to_string(&request).unwrap());
    let parsed: Value = serde_json::from_str(&response).expect("valid json response");

    assert_eq!(parsed["id"], "nested-4");
    assert_eq!(parsed["ok"], true);
    assert_eq!(parsed["result"]["count"], 0);
    assert_eq!(parsed["result"]["total"], 0.0);
    assert_eq!(parsed["result"]["average"], 0.0);
    assert_eq!(parsed["result"]["categories"], json!([]));
    assert_eq!(parsed["result"]["items"], json!([]));
}
