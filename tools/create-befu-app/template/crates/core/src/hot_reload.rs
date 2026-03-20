use befu_bridge::CommandRegistry;
use libloading::Library;
use std::sync::Mutex;

#[cfg(debug_assertions)]
static HOT_LIBRARY: Mutex<Option<Library>> = Mutex::new(None);

type InitFn = unsafe extern "C" fn(*mut CommandRegistry);

#[cfg(debug_assertions)]
pub fn load_external_commands(registry: &mut CommandRegistry) {
    let lib_name = if cfg!(target_os = "android") { "libbefu_app.so" } else { "libbefu_app.dylib" };

    let mut paths = vec![
        format!("./{}", lib_name),
        format!("/data/local/tmp/{}", lib_name),
        // On Android, we search the app's internal code_cache
        let lib_dir = "/data/data/dev.befu.app/code_cache";
        let version_file = format!("{}/befu_hot_version", lib_dir);
        if let Ok(versioned_name) = std::fs::read_to_string(&version_file) {
            let versioned_name = versioned_name.trim();
            if !versioned_name.is_empty() {
                paths.push(format!("{}/{}", lib_dir, versioned_name));
            }
        }
        paths.push(format!("{}/{}", lib_dir, lib_name));
    } else {

    if let Ok(exe) = std::env::current_exe()
        && let Some(parent) = exe.parent()
    {
        // On iOS, sync-rust.sh writes a versioned dylib name to 'befu_hot_version'
        // so dlopen is forced past its cache and loads fresh code.
        let version_file = parent.join("befu_hot_version");
        if let Ok(versioned_name) = std::fs::read_to_string(&version_file) {
            let versioned_name = versioned_name.trim();
            if !versioned_name.is_empty() {
                paths.push(format!("{}/{}", parent.to_string_lossy(), versioned_name));
            }
        }
        // Always include the canonical name as fallback (initial load)
        paths.push(format!("{}/{}", parent.to_string_lossy(), lib_name));
    }

    if let Ok(temp) = std::env::var("TMPDIR") {
        paths.push(format!("{}/{}", temp, lib_name));
    }
    paths.push(format!("/tmp/{}", lib_name));

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

#[cfg(debug_assertions)]
use std::time::SystemTime;

#[cfg(debug_assertions)]
static LAST_VERSION: Mutex<Option<SystemTime>> = Mutex::new(None);

#[cfg(debug_assertions)]
pub fn check_for_library_updates() -> bool {
    // Determine the sentinel file to watch
    let watchdog = if cfg!(target_os = "android") {
        "/data/data/dev.befu.app/code_cache/befu_hot_version".into()
    } else if let Ok(exe) = std::env::current_exe()
        && let Some(parent) = exe.parent()
    {
        parent.join("befu_hot_version")
    } else {
        return false;
    };

    if let Ok(meta) = std::fs::metadata(watchdog)
        && let Ok(mtime) = meta.modified()
    {
        let mut last = LAST_VERSION.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(prev) = *last {
            if mtime > prev {
                *last = Some(mtime);
                return true;
            }
        } else {
            *last = Some(mtime);
        }
    }
    false
}

#[cfg(not(debug_assertions))]
pub fn load_external_commands(_: &mut CommandRegistry) {}

#[cfg(not(debug_assertions))]
pub fn check_for_library_updates() -> bool {
    false
}
