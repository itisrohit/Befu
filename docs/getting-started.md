# Getting Started

## Requirements

- Bun `>=1.2`
- Rust toolchain (`rustup`, `cargo`)
- Android Studio SDK/NDK + `adb` (for Android work)
- Xcode `16+` + command-line tools (for iOS work)

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
bun run a:up
bun run a:restart
bun run a:logs
bun run a:down
bun run a:smoke
```

## iOS Commands

```bash
bun run i:list
bun run i:build
bun run i:install
bun run i:launch
bun run i:up
bun run i:smoke
```

For platform-specific details, see `android/README.md` and `ios/README.md`.
