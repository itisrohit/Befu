# Contributing to Befu

Befu is in its early stages (Phase 2), focusing on developer experience and hot-reloading for Rust-backed mobile apps. We welcome contributions to help stabilize the bridge and move into production hardening (Phase 3+).

## Tech Stack Overview

- **Core (Rust)**: JNI/FFI-based bridge with a modular crate structure.
- **Frontend (TS/JS)**: SolidJS + Vite (current default).
- **Transport**: WebView-to-Native protocol using serialized JSON bundles.
- **Tooling**: Bun-first scripts for low setup friction.

## Developer Environment

1.  **Doctor Check**: Ensure your Bun, Rust, and SDK paths are valid:
    ```bash
    bun run doctor
    ```
2.  **Bootstrap**: Install dependencies and initialize git hooks:
    ```bash
    bun run bootstrap
    ```
3.  **Local Dev**: Run the web server and your platform of choice:
    ```bash
    bun run dev
    bun run a:up  # Android
    bun run i:up  # iOS
    ```

## Mission & Priority Areas

Currently, we are looking for builders to help with:

- **iOS Production Packaging**: Switching from simulator-only artifacts to robust XCFrameworks for device builds.
- **Android Hardening**: Optimizing APK/AAB size and finalizing release-signing flows.
- **CLI Ecosystem**: Support for more frontend frameworks (React, Svelte) and improved project scaffolding.
- **Bridge Reliability**: Expanding test coverage for large payloads and concurrent command calls.

## House Rules

- **Rust Formatting**: Run `cargo fmt --all` before committing.
- **Quality Gate**: Changes should pass `bun run quality` (combines linting, type-check, and bridge tests).
- **Branch Strategy**: Open a feature branch from `main` and create a PR with a description of the "why" behind the change.

---

Befu is an experimental runtime. Break stuff, improve the bridge, and have fun!
