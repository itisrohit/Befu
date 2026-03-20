# Rust Command Developer Experience (DX)

Befu aims for a "Tauri-style" DX with minimal boilerplate when adding native functionality. Commands are implemented in Rust and called from the SolidJS frontend.

## 1. Implement and Wrap the Command

Use the `#[command]` attribute in your Rust module (e.g., `crates/core/src/demo_commands.rs`).
The macro automatically handles:

1. Argument deserialization from the bridge JSON.
2. Doc comment extraction for the "description" metadata.
3. Success/failure response wrapping.

```rust
use crate::{CommandMetadata, command};
use serde::Serialize;

#[derive(Serialize)]
pub struct MyResponse {
    pub message: String,
}

/// My first befu command. (This doc comment becomes the description)
#[command]
pub fn my_command(name: String) -> MyResponse {
   MyResponse { message: format!("Hello, {}!", name) }
}
```

## 2. Register the Command

In `crates/core/src/lib.rs`, the `CommandRegistry::new()` function collects all available commands. Use the `register_commands!` macro for bulk registration.

It takes the registry instance followed by a list of command paths. It automatically finds the macro-generated wrappers and metadata.

```rust
impl CommandRegistry {
    pub fn new() -> Self {
        let mut registry = Self::default();
        // ... (other registrations)
        befu_macros::register_commands!(registry, demo_commands::my_command);
        registry
    }
}
```

## 3. Call from the Frontend

Import the `invoke` helper from `@befu/bridge` and call your command using its Rust function name.

```typescript
import { invoke } from '@befu/bridge'

const result = await invoke('my_command', { name: 'Developer' })
console.log(result.message) // "Hello, Developer!"
```

## 4. Introspection (Testing)

You can list all registered commands and their descriptions from the frontend by calling the special `befu.commands` command.

```typescript
const available = await invoke('befu.commands')
// Returns [ { name: "my_command", description: "My first befu command." }, ... ]
```

## Command Validation

The macro generates runtime validation. If you send the wrong JSON shape for arguments, the bridge will return an `INVALID_ARGUMENT` response with the error details.
You can test this using the standard test suite:

```bash
bun run test:rust
```

---

## 5. Hot Reloading (Debug Only)

For a faster developer iteration loop, you can implement commands in the `befu-app` crate (`crates/app/src/lib.rs`) instead of `befu-core`.

- **`befu-core` commands**: Shipped with the main binary. Require a full app build/reinstall. Best for stable, core bridge functionality.
- **`befu-app` commands**: Dynamically loaded at runtime in debug mode. Support **Hot Reloading** without app re-installation. Best for active development of business logic.

See the [Hot Command Reload guide](hot-reload.md) for the sync workflow.
