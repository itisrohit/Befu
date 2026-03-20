# Befu Next Phases

## Phase 0 - Developer Experience Foundation

- [done] Add `bun run doctor` to validate Bun, Rust, Xcode tools, Android SDK/NDK, adb/simctl.
- [done] Add `bun run bootstrap` to install deps, install Rust targets, prepare iOS project, and verify Android toolchain.
- [done] Add `bun run dev:mobile` flow to start web dev server and launch Android/iOS in one command.
- [done] Improve smoke flows (`a:smoke`, `i:smoke`) to assert real bridge responses instead of manual UI-only checks.
- [done] Add a top-level "5-minute setup" quickstart in `README.md`.

## Phase 1 - Tauri-style Scaffolding CLI

- [done] Create `tools/create-befu-app` scaffold CLI.
- [done] Support interactive prompts and non-interactive flags (`--name`, `--platform`, `--yes`).
- [done] Generate a full Befu template workspace (Solid + Rust + bridge + Android/iOS shells).
- [done] Prepare package metadata for publish path (`bun create befu-app`, `bunx create-befu-app`).
- [done] Add selective platform shaping (`android`, `ios`, `both`) for generated content.
- [done] Publish `create-befu-app` package and verify public install flows.
- [done] Add dedicated scaffolder CI workflow validating generated templates for `android`, `ios`, and `both`.

- Goal: make Rust functions callable from frontend with near-zero bridge boilerplate.
- **Feature**: Hot reload Rust backend logic.
- **Efficiency**: No-reinstall development loop (**Install once, iterate forever**).
- Baseline developer experience:
  - Rust: `#[befu::command] fn hello(name: String) -> String { ... }`
  - Frontend: `await invoke("hello", { name: "Developer" })`
- MVP implementation:
  - [done] explicit Rust command registry (name -> handler)
  - [done] runtime dispatch map lookup by command name
  - [done] typed argument/result envelope with consistent error responses
  - [done] one end-to-end example command (`hello`) wired through web bridge and Rust
  - [done] tests for registration, dispatch, unknown command, and argument validation
  - [done] add registry ergonomics for low-boilerplate command registration
  - [done] add command introspection metadata for tooling/docs generation
- Follow-up enhancements:
  - [done] procedural macro auto-registration for `#[befu::command]`:
    - [done] generate command metadata (`name`, arg schema, return shape)
    - [done] auto-register handlers into the runtime registry at compile time
    - [done] reduce manual map wiring and registration boilerplate
    - [done] provide compile-time errors for unsupported signatures
  - [done] Hot Rust Command Reload: web-like iteration speed for Rust command logic on mobile.
    - [done] target workflow:
      - [done] edit Rust command
      - [done] save (automatic or manual sync)
      - [done] runtime reloads command implementation
      - [done] frontend `invoke(...)` reflects change without full app reinstall/rebuild
    - [done] technical runtime flow (debug only):
      - [done] compile Rust command crate as dynamic library (`befu-app`)
      - [done] runtime loads library dynamically (`libloading`)
      - [done] command handlers are resolved by exported symbols (`befu_init_app`)
      - [done] file watcher detects rebuilt library and triggers reload cycle
      - [done] reload command in bridge replaces registry
    - [done] expected value:
      - [done] dramatically shorter feedback loop than standard native rebuild flow
    - [done] Android: reload debug Rust command library without reinstalling the app
    - [done] iOS simulator: mirror reload flow where toolchain/runtime allows
    - [done] keep release builds static and deterministic (no dynamic reload path)
    - [done] safety constraints:
      - [done] debug-only guardrails at compile/runtime level
      - [done] explicit kill-switch (mode flag in `app.info`)

## Phase 3 - iOS Production Packaging

- **Next Step**: Investigate **State-preserving reload** (support reloading logic without losing in-memory Rust app state).
- Add Rust device target build (`aarch64-apple-ios`) to iOS rust prep.
- Package simulator + device artifacts as XCFramework (or equivalent robust packaging).
- Add device build/archive scripts (`ios:build:device`, optional `ios:archive`).
- Add CI validation for iOS packaging outputs.
- Document signing/distribution path for TestFlight/App Store.

## Phase 4 - Android Production Hardening

- Expand ABI/device/API coverage for runtime verification.
- Add deterministic release signing flow and release checklist.
- Add automated runtime checks for JNI mode and bridge health.
- Run final WebView/network security audit for release builds.
- Add APK/AAB size and startup profiling with optimization targets.

## Phase 5 - CI, Release, and Observability

- [done] Add release process (`CHANGELOG`, versioning, tags, release notes).
- [done] Add optional matrix CI for platform/runtime checks where feasible.
- [done] Add structured native bridge logs and failure diagnostics.
- [done] Add troubleshooting docs for common setup/build failures.
- [done] Add contributor onboarding doc for daily workflow and local cache strategy.

## Phase 6 - Ecosystem Growth

- Add additional bridge command examples beyond `ping`/`app.info`.
- Add optional frontend template choices (React/Svelte) in scaffolder.
- Add plugin-style extension model for Rust/native command modules.
- Add starter app examples demonstrating full-stack patterns.
