use befu_bridge::CommandRegistry;
use libloading::Library;
use std::sync::Mutex;

static HOT_LIBRARIES: Mutex<Vec<Library>> = Mutex::new(Vec::new());

type InitFn = unsafe extern "C" fn(&mut CommandRegistry);

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
            // Loading the library and calling its initialization are unsafe.
            unsafe {
                if let Ok(lib) = Library::new(&path) {
                    if let Ok(init) = lib.get::<InitFn>(b"befu_init_app") {
                        println!("[befu:hot] Initializing external commands from {}", path);
                        init(registry);
                    }

                    // Keep the library handle alive so symbols remain valid
                    let mut libs = HOT_LIBRARIES.lock().unwrap();
                    libs.push(lib);
                    return;
                }
            }
        }
    }
}
