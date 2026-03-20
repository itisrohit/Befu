# Hot Command Reload

Befu supports near-instant Rust command updates on mobile devices and simulators. This allows you to iterate on your core logic without waiting for full Gradle/Xcode builds.

## How it Works

1.  **Dynamic Library (`befu-app`)**: In debug mode, `befu-core` looks for a dynamic library (`.so` or `.dylib`) in the app's internal filesystem.
2.  **Background Registry Watcher**: A low-priority thread in `befu-core` polls for binary updates every 1 second using an atomic **Sentinel File** (`befu_hot_version`).
3.  **Zero-Click Sync**: The `sync-rust.sh` scripts build the `befu-app` crate and push it to the device sandbox. Once the write is complete, the sentinel is updated, and the app swaps the command registry automatically.

## Developer Workflow

Befu provides a unified command that starts the web development server, the Rust watcher, and launches the app with live log tailing in **one terminal tab**:

```bash
bun run a:dev  # Launch everything for Android
# OR
bun run i:dev  # Launch everything for iOS
```

---

## Developer Workflow (Manual / Advanced)

If you prefer to see build logs and app logs in separate tabs:

### 1. Start the App

Launch the app normally on your emulator or simulator:

```bash
bun run a:up  # Android
# OR
bun run i:up  # iOS
```

### 2. Start the Watcher

In a **separate terminal tab**, start the Rust watcher. This will monitor `crates/app` and `crates/bridge` for changes:

```bash
bun run a:hot  # Android
# OR
bun run i:hot  # iOS
```

### 3. Change Rust Code

Modify functions in `crates/app/src/lib.rs`. For example, change a return message or logic:

```rust
#[command(name = "hello")]
pub fn hello_from_app(name: String) -> AppInfo {
    AppInfo { message: format!("Hello {name} from HOT RELOAD") }
}
```

### 4. Zero-Click Apply

Once the watcher finishes syncing (usually < 1.5s), the app will detect the change and apply the new logic **automatically**. The **Hot Reloading** indicator should show a pulsing green dot when active. You do **not** need to click the "Reload" button (it remains as a manual fallback).

---

## Technical Details

### Architecture

- **Staging**: On Android, the `.so` is pushed to `/data/local/tmp` and then moved to the app's internal files dir using `run-as`.
- **Registry Swapping**: The `CommandRegistry` is stored behind a `Mutex`. A reload safely replaces the entire map of handler pointers.
- **Safety**: Hot reloading is strictly disabled in release builds (`cfg(debug_assertions)`) to prevent dynamic code loading vulnerabilities in production.

### Troubleshooting

- **No implementation found (JNI)**: Ensure `befu_core` was successfully linked during the initial build.
- **Library not found**: Check the app logs to see where `befu-core` is looking for `libbefu_app`.
- **Architecture mismatch**: The sync script attempts to auto-detect the device ABI. If it fails, check `adb shell getprop ro.product.cpu.abi`.
