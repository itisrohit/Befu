# Befu Next Phases

## Phase 0 - Developer Experience Foundation

- Add `bun run doctor` to validate Bun, Rust, Xcode tools, Android SDK/NDK, adb/simctl.
- Add `bun run bootstrap` to install deps, install Rust targets, prepare iOS project, and verify Android toolchain.
- Add `bun run dev:mobile` flow to start web dev server and launch Android/iOS in one command.
- Improve smoke flows (`a:smoke`, `i:smoke`) to assert real bridge responses instead of manual UI-only checks.
- Add a top-level "5-minute setup" quickstart in `README.md`.

## Phase 1 - Tauri-style Scaffolding CLI

- [in progress] Create `tools/create-befu-app` scaffold CLI.
- [in progress] Support interactive prompts (project name, platforms; CI/hooks next).
- [next] Ship first full template: Solid + Rust core + bridge + Android/iOS shells.
- [next] Expose command flow similar to Tauri ergonomics (`bun create befu-app`, `bunx create-befu-app`).
- [next] Generate project with prewired scripts: `dev`, `quality`, `doctor`, `bootstrap`, `a:up`, `i:up`.

## Phase 2 - iOS Production Packaging

- Add Rust device target build (`aarch64-apple-ios`) to iOS rust prep.
- Package simulator + device artifacts as XCFramework (or equivalent robust packaging).
- Add device build/archive scripts (`ios:build:device`, optional `ios:archive`).
- Add CI validation for iOS packaging outputs.
- Document signing/distribution path for TestFlight/App Store.

## Phase 3 - Android Production Hardening

- Expand ABI/device/API coverage for runtime verification.
- Add deterministic release signing flow and release checklist.
- Add automated runtime checks for JNI mode and bridge health.
- Run final WebView/network security audit for release builds.
- Add APK/AAB size and startup profiling with optimization targets.

## Phase 4 - CI, Release, and Observability

- Add release process (`CHANGELOG`, versioning, tags, release notes).
- Add optional matrix CI for platform/runtime checks where feasible.
- Add structured native bridge logs and failure diagnostics.
- Add troubleshooting docs for common setup/build failures.
- Add contributor onboarding doc for daily workflow and local cache strategy.

## Phase 5 - Ecosystem Growth

- Add additional bridge command examples beyond `ping`/`app.info`.
- Add optional frontend template choices (React/Svelte) in scaffolder.
- Add plugin-style extension model for Rust/native command modules.
- Add starter app examples demonstrating full-stack patterns.
