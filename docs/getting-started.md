# Getting Started

## Requirements

- Bun `>=1.2`
- Rust toolchain (`rustup`, `cargo`)
- Android Studio SDK/NDK + `adb` (for Android work)
- Xcode `16+` + command-line tools (for iOS work)

> [!TIP]
> Install `cargo install cargo-watch` for significantly better performance and lower latency during Hot Command Reload sessions.

## New Machine Setup

```bash
bun run doctor
bun run bootstrap
```

## Daily Workflow: **Install Once, Iterate Forever**

Traditional mobile development requires a full build and reinstall for every Rust change. Befu eliminates this loop by letting you sync logic directly into the running app.

### 1. Launch the Runtime

```bash
bun run dev:mobile android  # Launch everything for Android
# OR
bun run dev:mobile ios      # Launch everything for iOS
```

## Verification

```bash
bun run test:bridge
bun run test:rust
bun run build
```

## Quality Gate

```bash
bun run quality
```

## Android Commands

```bash
bun run a:up       # Launch app + tail logs
bun run a:hot      # Watch and sync Rust changes
bun run a:restart  # Force restart app
bun run a:logs     # View adb logcat
bun run a:down     # Stop everything
bun run a:smoke    # Automated health check
```

## iOS Commands

```bash
bun run i:up       # Launch app (sim)
bun run i:hot      # Watch and sync Rust changes
bun run i:list     # List simulators
bun run i:build    # Build simulator target
bun run i:install  # Install into simulator
bun run i:launch   # Launch on simulator
bun run i:smoke    # Automated health check
```

For platform-specific details, see `android/README.md` and `ios/README.md`.
