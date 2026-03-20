use befu_macros::command;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct HelloResponse {
    pub message: String,
}

/// Returns a greeting for the given name.
#[command]
pub(crate) fn hello(name: String) -> HelloResponse {
    HelloResponse { message: format!("Hello {name}") }
}

#[cfg(test)]
mod tests {
    use super::hello;

    #[test]
    fn returns_hello_text() {
        assert_eq!(hello("Developer".to_string()).message, "Hello Developer");
    }
}
