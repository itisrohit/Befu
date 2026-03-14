use crate::{BridgeRequest, BridgeResponse, failure_response, success_response};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct HelloArgs {
    name: String,
}

fn hello(name: &str) -> String {
    format!("Hello {name}")
}

pub(crate) fn hello_command(request: &BridgeRequest) -> BridgeResponse {
    let parsed_args = request
        .args
        .clone()
        .ok_or_else(|| {
            failure_response(
                &request.id,
                "INVALID_ARGUMENT",
                "Missing args for command: hello",
                None,
            )
        })
        .and_then(|value| {
            serde_json::from_value::<HelloArgs>(value.clone()).map_err(|error| {
                failure_response(
                    &request.id,
                    "INVALID_ARGUMENT",
                    format!("Invalid args for command: hello ({error})"),
                    Some(value),
                )
            })
        });

    match parsed_args {
        Ok(args) => {
            success_response(&request.id, serde_json::json!({ "message": hello(&args.name) }))
        }
        Err(error_response) => error_response,
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
