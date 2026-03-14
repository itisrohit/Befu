# Befu

Lightweight runtime experiment for cross-platform mobile apps, with a SolidJS frontend and Rust backend connected through a small bridge layer.

## Workspace layout

- `apps/web`: SolidJS + Vite frontend
- `packages/bridge`: tiny `invoke()` API and transport contract
- `crates/core`: Rust backend core crate (`ping() -> "pong"`)
- `docs/plan.md`: project plan and roadmap

## Requirements

- Bun `>=1.2`
- Rust toolchain (`rustup`, `cargo`)

## Quick start

```bash
bun install
bun run dev
```

Open `http://localhost:5173`.

## Verify

```bash
bun run test:bridge
bun run test:rust
bun run build
```

## Code quality

```bash
# lint + format + types + tests + rust checks + build
bun run quality
```

Individual commands:

```bash
bun run lint
bun run lint:fix
bun run format
bun run format:check
bun run rust:fmt:check
bun run rust:clippy
```

## Pull request gates

- CI workflow: `.github/workflows/ci.yml`
- Required command for PRs: `bun run quality`
- CodeRabbit config: `.coderabbit.yaml`
- Contribution guide: `CONTRIBUTING.md`
- PR template: `.github/PULL_REQUEST_TEMPLATE.md`
- Android CI build: `Android Debug Build` (APK assemble + Rust JNI libs)

Recommended repository settings on GitHub:

- protect `main`
- require pull request before merge
- require status checks to pass (CI / Quality Gate)
- require branch to be up to date before merge
- require at least 1 approving review

## Git hooks (block bad commits)

This repo uses Lefthook (`lefthook.yml`) as a YAML-based git hooks manager.

- `pre-commit` runs: format check, lint, type-check, bridge tests, rust format check
- `pre-push` runs: full `bun run quality`

If any step fails, commit/push is blocked.

Setup:

```bash
bun install
bun run hooks:install
```

## Current bridge flow

- `apps/web/src/App.tsx` configures an in-app transport.
- UI calls typed commands (`invoke('ping')`, `invoke('app.info')`) from `@befu/bridge`.
- Transport returns a protocol response envelope (`{ id, ok, result | error }`).
- Rust crate exposes `handle_request(json)` in `crates/core/src/lib.rs` as backend command dispatcher.

## Android shell (early scaffold)

- Android project lives in `android/`.
- `MainActivity` hosts a `WebView` and exposes `window.BefuNative.invokeRaw(payloadJson)`.
- Web bridge now tries native transport first, then falls back to local mock transport for desktop/web dev.
- JNI symbol path is prepared from Android to Rust dispatcher (`handle_request`).

Run Android app (from `android/`):

```bash
./gradlew :app:assembleDebug
```

Release builds bundle web assets into Android app assets and load through
`https://appassets.androidplatform.net/assets/index.html` via `WebViewAssetLoader`.

The Android folder now includes Gradle Wrapper scripts, so `./gradlew` works without globally installed Gradle.

Prereqs for Rust JNI build inside Android:

```bash
cargo install cargo-ndk
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
```

For emulator dev mode, keep web dev server running from repo root:

```bash
bun run dev
```

## iOS shell (early scaffold)

From repo root:

```bash
bun run ios:prepare
```

Then either run from Xcode, or use CLI:

```bash
xcodebuild -project ios/BefuIOS.xcodeproj -scheme Befu -destination "platform=iOS Simulator,name=iPhone 17" -derivedDataPath ios/build build
xcrun simctl install booted ios/build/Build/Products/Debug-iphonesimulator/Befu.app
xcrun simctl launch booted dev.befu.ios
```

For debug server mode on iOS, keep web dev server running:

```bash
bun run dev
```

## Android shortcuts

See `android/README.md` for the canonical Android command flow.

Quick aliases:

```bash
bun run a:up
bun run a:restart
bun run a:logs
bun run a:down
```
