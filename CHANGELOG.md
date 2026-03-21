# Changelog

All notable changes to the Befu project will be documented in this file.

## [0.1.4] - 2026-03-21

### Added

- **Configurable `applicationId`**: The `create-befu-app` scaffolder now prompts for and automatically injects a custom `applicationId` (bundle ID) throughout the generated project.
- **Improved Scaffolder UX**: Added project-specific metadata substitution for README and package versions during generation.

### Fixed

- **CodeRabbit Security & Quality Audit**: Addressed 10+ issues including UI Stylelint errors, ABI pointer stability in the template, and memory retain cycles in the iOS shell.
- **iOS Hot Reload Resilience**: Fixed simulator UUID extraction to be case-insensitive and added versioned library paths for atomic registry swaps.
- **Android Dev Stability**: Fixed `adb` device detection patterns and added a system boot-wait timeout to prevent `installDebug` failures.
- **Process Management**: Fixed `dev:mobile` script to correctly block and maintain background workers (Vite/Rust watcher) until termination.
- **Accessibilty**: Introduced ARIA live regions for async command results in the web UI.

## [0.1.3] - 2026-03-20

### Added

- **Hot Rust Command Reload**: Near-instant updates for Rust logic on Android and iOS simulators without full app rebuilds.
- **Procedural Macro Registry**: Introduced `#[command]` and `register_commands!` for zero-boilerplate bridge wiring.
- **Modular Architecture**: Split the core runtime (`befu-core`), command logic (`befu-app`), and bridge infrastructure.
- **Scaffolder CLI**: First stable release of `create-befu-app` with complete template generation.
- **Improved UI**: Native status indicators and bridge health reporting in the default SolidJS view.
- **Android Hardening**: Dynamic sandbox path discovery and workspace-aware sync scripts.

### Changed

- Standardized project and template to use **Bun v1.3.11**.
- Updated all internal crates to use the **Rust 2024 edition**.
- Refactored bridge transport to use stricter argument validation (`serde(deny_unknown_fields)`).

### Fixed

- Fixed nested `Cargo.toml` issues in the scaffolder template.
- Resolved FFI safety warnings for the dynamic library initialization boundary.
- Improved error visibility for bridge initialization and hot-reload failures.

---

[0.1.0] - Initial prototype.
