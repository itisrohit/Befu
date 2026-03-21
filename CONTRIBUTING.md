# Contributing to Befu

Befu has moved into stable **Phase 2**, focusing on zero-click hot-reloading for Rust-backed mobile apps. We welcome contributions to help stabilize the bridge and move into physical device support (Phase 3+).

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
3.  **Unified Dev Cycle**: Launch the full stack (web server + Rust watcher + app logs) in **one command**:
    ```bash
    bun run a:dev  # Android
    bun run i:dev  # iOS
    ```

**Zero-Click Hot Reloading:**
Once `a:dev` or `i:dev` is running, simply save your Rust code in `crates/app/src/lib.rs`. The watcher will rebuild the library and the bridge will automatically swap the registry within ~1 second—no app reinstall required.

## Template Synchronization

> [!IMPORTANT]
> If you modify **core scripts**, **bridge logic**, or **app logic**, you **MUST** ensure these changes are synchronized to the template in `tools/create-befu-app/template`. This ensures that all future projects scaffolded with `create-befu-app` benefit from your improvements.

## Mission & Priority Areas

Currently, we are looking for builders to help with:

- **iOS Physical Device Support**: Moving beyond simulator-only artifacts to robust `aarch64-apple-ios` XCFrameworks.
- **State-Preserving Reload**: Designing a mechanism to maintain Rust global state across hot-swaps.
- **Ecosystem Expansion**: Supporting more frontend frameworks (React, Svelte) and improved project scaffolding.
- **Android Hardening**: Finalizing release-signing flows and optimizing application binary size.

## House Rules

- **Rust Formatting**: Run `cargo fmt --all` before committing.
- **Quality Gate**: Changes should pass `bun run quality` (combines linting, type-check, and bridge tests).
- **Branch Strategy**: Open a feature branch from `main` and create a PR with a description of the "why" behind the change.

---

Befu is a next-generation mobile runtime. Break stuff, improve the bridge, and have fun!
