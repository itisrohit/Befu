# Roadmap: The Path to Stable

Befu is currently in intensive development. This roadmap outlines the evolution from MVP to a production-ready framework.

## ✅ Phase 1: Core Bridge Foundation (Done)

- Binary communication protocol via JNI/FFI.
- Typed command registry in Rust.
- Core CLI (scaffolder) for monorepo setup.

## ✅ Phase 2: Android & SolidJS (Done)

- Production-grade Android container.
- Full SolidJS integration for high-performance UI.
- Native view embedding.

## ✅ Phase 3: High-Frequency Hot Reload (Done)

- dylib/so side-loading without app restart.
- Background watchdog for Rust compilation.
- Frontend bridge-state self-healing.

## ✅ Phase 4: Apple Ecosystem (Done)

- iOS support via `befu-ios` static runtime.
- Xcode project generation via `xcodegen`.
- Parallel cross-compilation scripts.

## ✅ Phase 5: First-Class React Support (Done)

- Standard `@befu/bridge` support for React hooks.
- React-native style bridge integration.
- React template in `create-befu-app`.

## 🛠️ Phase 6: Ecosystem & Advanced DX

- [ ] Add Svelte template option.
- [ ] Integrated SQLite storage bridge.
- [ ] Shared state management patterns (Rust-First).
- [ ] Automated release automation for dylibs.
- [ ] State-preserving hot-reload (Rust-side persistence during swaps).

## 🚀 Phase 7: Production Release

- Public npm publishing of all tools.
- Documentation site (Docusaurus).
- Showcase sample applications.
