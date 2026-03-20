# Hot Command Reload (USP)

Befu supports near-instant Rust command updates on mobile devices and simulators. This allows you to iterate on your core logic without waiting for full Gradle/Xcode builds.

## How it Works

1.  **Dynamic Library (`befu-app`)**: In debug mode, `befu-core` looks for a dynamic library (`.so` or `.dylib`) in the app's internal filesystem.
2.  **Runtime Linking**: The `befu.reload` command triggers `libloading` to link the external library and override the command registry with new implementations.
3.  **Cross-Platform Sync**: Automated scripts build the `befu-app` crate for the target architecture and push it to the device sandbox.

## Developer Workflow

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

### 4. Click "Reload Rust Module"

Once the watcher finishes syncing (usually < 2s), click the **🔄 Reload Rust Module** button in the mobile app. The **Hot Reloading** indicator should show a pulsing green dot when active.

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
