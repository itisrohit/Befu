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

## Daily Workflow

```bash
bun run dev
```

Optional mobile launch helpers:

```bash
bun run dev:mobile android
bun run dev:mobile ios
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
