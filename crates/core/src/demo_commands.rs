use crate::{BridgeRequest, BridgeResponse, failure_response, success_response};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct HelloArgs {
    name: String,
}

/// Returns a greeting for the given name.
fn hello(name: &str) -> String {
    format!("Hello {name}")
}

/// Handles the demo `hello` command with typed argument parsing.
pub(crate) fn hello_command(request: &BridgeRequest) -> BridgeResponse {
    let Some(value) = request.args.clone() else {
        return failure_response(
            &request.id,
            "INVALID_ARGUMENT",
            "Missing args for command: hello",
            None,
        );
    };

    match serde_json::from_value::<HelloArgs>(value.clone()) {
        Ok(args) => {
            success_response(&request.id, serde_json::json!({ "message": hello(&args.name) }))
        }
        Err(error) => failure_response(
            &request.id,
            "INVALID_ARGUMENT",
            format!("Invalid args for command: hello ({error})"),
            Some(value),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::hello;

    #[test]
    fn returns_hello_text() {
        assert_eq!(hello("Developer"), "Hello Developer");
    }
}
