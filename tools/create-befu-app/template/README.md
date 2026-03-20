# My Befu App

Welcome to your new Befu application! This workspace is set up with a SolidJS frontend and a modular Rust backend.

## Workspace layout

- `apps/web`: SolidJS + Vite frontend
- `crates/app`: Your hot-reloadable Rust logic (commands live here)
- `crates/core`: Rust backend core and native bridge integration
- `crates/bridge`: Rust type definitions for the bridge protocol
- `packages/bridge`: TypeScript API for calling Rust from JS
- `android/`: Android native shell project
- `ios/`: iOS native shell project

## Requirements

- Bun `>=1.3`
- Rust toolchain (`rustup`, `cargo`)
- Xcode `16+` (for iOS)
- Android Studio / NDK (for Android)

## Quick Start

1. **Bootstrap the project**:

   ```bash
   bun run bootstrap
   ```

2. **Start the web dev server**:

   ```bash
   bun run dev
   ```

3. **Launch mobile**:
   - Android: `bun run a:up`
   - iOS: `bun run i:up`

## Hot Reloading (USP) 🚀

Befu supports instant Rust hot-reloading for mobile.

1. Install `cargo-watch`: `cargo install cargo-watch`
2. Run the hot-reload watcher:
   - Android: `bun run a:hot`
   - iOS: `bun run i:hot`

Any changes you make in `crates/app/src/lib.rs` will be compiled and synced to the running app immediately!

## Code Quality

Maintain your app with these commands:

```bash
bun run quality  # runs all checks
bun run lint
bun run format
bun run test:bridge
bun run test:rust
```

## Learn More

Check out the `docs/` folder for deeper architectural details.
