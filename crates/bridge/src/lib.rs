use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
pub struct BridgeRequest {
    pub id: String,
    pub command: String,
    pub args: Option<Value>,
}

#[derive(Debug, Serialize, Clone)]
pub struct BridgeError {
    pub code: &'static str,
    pub message: String,
    pub details: Option<Value>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum BridgeResponse {
    Success { id: String, ok: bool, result: Value },
    Failure { id: String, ok: bool, error: BridgeError },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandMetadata {
    pub name: &'static str,
    pub description: &'static str,
}

pub type CommandHandler = fn(&BridgeRequest) -> BridgeResponse;

pub struct RegisteredCommand {
    pub metadata: CommandMetadata,
    pub handler: CommandHandler,
}

#[derive(Default)]
pub struct CommandRegistry {
    pub commands: std::collections::HashMap<String, RegisteredCommand>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, metadata: CommandMetadata, handler: CommandHandler) {
        let name = metadata.name.to_string();
        if self.commands.contains_key(&name) {
            panic!("Duplicate command registration: {}", name);
        }
        self.commands.insert(name, RegisteredCommand { metadata, handler });
    }

    pub fn find(&self, name: &str) -> Option<&RegisteredCommand> {
        self.commands.get(name)
    }

    pub fn list_metadata(&self) -> Vec<CommandMetadata> {
        let mut meta: Vec<_> = self.commands.values().map(|c| c.metadata.clone()).collect();
        meta.sort_by(|a, b| a.name.cmp(b.name));
        meta
    }
}

pub fn success_response(id: &str, result: Value) -> BridgeResponse {
    BridgeResponse::Success { id: id.to_owned(), ok: true, result }
}

pub fn failure_response(
    id: &str,
    code: &'static str,
    message: String,
    details: Option<Value>,
) -> BridgeResponse {
    BridgeResponse::Failure {
        id: id.to_owned(),
        ok: false,
        error: BridgeError { code, message, details },
    }
}
