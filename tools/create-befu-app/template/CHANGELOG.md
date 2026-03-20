# Changelog

All notable changes to the Befu project will be documented in this file.

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
