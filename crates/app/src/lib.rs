use befu_bridge::CommandRegistry;
use befu_macros::command;
use serde::Serialize;

#[derive(Serialize)]
pub struct AppInfo {
    pub message: String,
}

/// The main command that demonstrates hot reloading.
#[command(name = "hello")]
pub fn hello_from_app(name: String) -> AppInfo {
    AppInfo { message: format!("Hello {name} from HOT RELOAD") }
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn befu_init_app(registry: *mut CommandRegistry) {
    let registry = unsafe { &mut *registry };
    befu_macros::register_commands!(registry, hello_from_app);
}
