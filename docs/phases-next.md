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

## 🛠️ Phase 6: Production Hardening & Safety

- [ ] **Strict Security Boundary**: Implemented compile-time gating for hot-reload logic (Zero-RCE in Production).
- [ ] **Type Parity**: Robust serialization for complex structs and arrays across the bridge.
- [ ] **Android Side-loading Stability**: Finalizing the FFI boundary for zero-panic module swaps.
- [ ] **Documentation**: Complete internal architecture guide for contributors.

## 🚀 Phase 7: v1.0 Production Release

- Official `befu-cli` for version management.
- Initial SQLite & Preferences plugins.
- Showcase: High-performance data-processing sample app.
- iOS Strategy: Finalize "Bundled-Only" vs "Dev-Side-loading" paths.
- Public npm publishing of all tools.
- Documentation site (Docusaurus).

## 🔮 Phase 8: v2.0 & Beyond

- [ ] **Unified Logic (WASM)**: Running the Rust core in browsers via WebAssembly.
- [ ] **Desktop Support**: Porting the Befu container to macOS/Windows.
