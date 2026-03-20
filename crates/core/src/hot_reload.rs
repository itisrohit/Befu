use befu_bridge::CommandRegistry;
use libloading::Library;
use std::sync::Mutex;

#[cfg(debug_assertions)]
static HOT_LIBRARY: Mutex<Option<Library>> = Mutex::new(None);

type InitFn = unsafe extern "C" fn(*mut CommandRegistry);

#[cfg(debug_assertions)]
pub fn load_external_commands(registry: &mut CommandRegistry) {
    let lib_name = if cfg!(target_os = "android") { "libbefu_app.so" } else { "libbefu_app.dylib" };

    let paths = [
        format!("./{}", lib_name),
        format!("/data/local/tmp/{}", lib_name),
        format!("/tmp/{}", lib_name),
    ];

    for path in paths {
        if std::path::Path::new(&path).exists() {
            println!("[befu:hot] Found library at {}", path);
            unsafe {
                match Library::new(&path) {
                    Ok(lib) => {
                        match lib.get::<InitFn>(b"befu_init_app") {
                            Ok(init) => {
                                println!("[befu:hot] Initializing external commands from {}", path);
                                init(registry as *mut _);

                                // Keep only the latest library handle alive
                                let mut current =
                                    HOT_LIBRARY.lock().unwrap_or_else(|e| e.into_inner());
                                *current = Some(lib);
                                return;
                            }
                            Err(e) => {
                                eprintln!("[befu:hot] Missing 'befu_init_app' in {}: {}", path, e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("[befu:hot] Failed to load library {}: {}", path, e);
                    }
                }
            }
        }
    }
}

#[cfg(not(debug_assertions))]
pub fn load_external_commands(_: &mut CommandRegistry) {}
