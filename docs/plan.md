# Befu Project Plan

## Executive Summary

**Befu** is an experimental, lightweight runtime designed for building cross-platform mobile applications. It leverages **SolidJS** for the user interface and **Rust** for the backend logic, connected via a minimal system WebView bridge.

The primary objective is to provide a thin, non-intrusive runtime layer that allows developers to utilize the full capabilities of both the web and Rust ecosystems without the overhead or lock-in of traditional frameworks.

### Current Baseline (March 2026)

- Bun-based monorepo initialized with workspaces.
- `apps/web` scaffolded with SolidJS + Vite + TypeScript.
- `packages/bridge` exposes typed `configureBridge()` and `invoke()` with response envelopes.
- `crates/core` exposes JSON request dispatcher (`handle_request`) with command routing.
- End-to-end demo path works in development (`ping` and `app.info`).
- Android shell scaffold added with `WebView` host and native bridge entrypoint.
- JNI symbol path from Android to Rust dispatcher is implemented with Kotlin fallback.

---

## Core Architectural Principles

Befu serves as a high-performance glue layer between the frontend and the native mobile environment.

### Communication Flow

1. **Frontend**: SolidJS / React / Vue
2. **Bridge**: `invoke()` API
3. **Shell**: Mobile System WebView
4. **Backend**: Rust Command Handlers

### Design Philosophy

- **Standardization**: The frontend remains a standard Bun workspace package; the backend remains a standard Cargo project.
- **Minimality**: Befu only facilitates the connection between these two environments.
- **Portability**: Native shells provide the necessary environment for cross-platform execution.

---

## Technology Stack

### Frontend logic

- **Framework**: SolidJS
- **Bundler**: Vite
- **Language**: TypeScript
- **Package Manager**: Bun

### Backend Engine

- **Language**: Rust
- **Serialization**: Serde (JSON)
- **Concurrency**: Tokio (as required)

### Native Integration

- **Android**: Android System WebView
- **iOS**: WKWebView
- **Bridge**: Lightweight JSON-over-MessagePort protocol

---

## Implementation Roadmap

### Phase 1: Frontend Infrastructure

Establish a robust SolidJS environment with TypeScript.

- Initialize project via Vite.
- Configure development server for rapid iteration.
- Implement responsive UI components.

**Status**: Completed baseline scaffold.

### Phase 2: Rust Backend Development

Build the core execution engine.

- Initialize Cargo project.
- Implement command routing logic.
- Define internal API for native lifecycle events.

**Status**: In progress (`befu-core` command dispatcher and starter handlers implemented).

### Phase 3: Communication Bridge

Develop the `befu.ts` API to expose Rust functionality to the UI.

- Implement `invoke(command, arguments)` helper.
- Target a bridge footprint of less than 1KB.
- Ensure asynchronous fulfillment using native callback markers.

**Status**: In progress (`@befu/bridge` typed command map and error envelope implemented).

### Phase 4: Mobile Shell Integration

#### Android Support

- Implement `MainActivity` with embedded WebView.
- Configure `JavascriptInterface` for message passing.
- Enable bridge routing to the Rust library.

**Status**: In progress (WebView shell and bridge interface done, JNI entrypoint wired, Rust `.so` packaging flow added via `cargo-ndk`).

#### iOS Support

- Use `WKWebView` with `WKScriptMessageHandler`.
- Route messages from Swift to the Rust library via FFI (Foreign Function Interface).

---

## Operational Workflows

### Development Mode

During development, the mobile shell points to the Vite dev server for an optimal experience.

- **URL**: `http://localhost:5173`
- **Features**: Hot Module Replacement (HMR) and Chrome DevTools debugging.
- **Run**: `bun run dev`
- **Android Emulator URL**: `http://10.0.2.2:5173`

### Production Mode

For production, the frontend is bundled and embedded directly into the application.

- **Build**: `bun run build`
- **Deployment**: Assets are copied to `android/assets/` or iOS bundle resources.
- **Backend**: Rust code is compiled into a native shared library (.so or .framework).

---

## Performance Targets

The project aims for an extremely small footprint compared to Electron or standard React Native apps.

| Component              | Target Size      |
| :--------------------- | :--------------- |
| Rust Native Library    | 1.0 – 2.0 MB     |
| UI Assets (Compressed) | 30 – 100 KB      |
| Native Shell Overhead  | ~2.0 MB          |
| **Total Bundle Size**  | **3.0 – 5.0 MB** |

---

## Future Scope and Experiments

- **Macros**: Implementation of `#[befu::command]` for automated code generation.
- **Standard Plugins**: Pre-built modules for Filesystem, HTTP, and SQLite access.
- **Desktop Support**: Extending the runtime to support macOS and Windows WebViews.
- **WebAssembly**: Exploring Rust-to-WASM compilation for high-performance UI logic.

---

## Success Criteria

1. SolidJS UI renders correctly within native mobile WebViews.
2. Bidirectional communication between JS and Rust functions is verified.
3. Successful deployment and execution on physical Android and iOS devices.
4. Minimal performance overhead during command execution.
5. Bun workspace workflow remains stable across local development and CI.

---

## Risk Assessment and Difficulty

| Phase          | Complexity | Description                       |
| :------------- | :--------- | :-------------------------------- |
| Frontend Setup | Low        | standard web tooling.             |
| Rust Backend   | Low        | Standard Cargo environment.       |
| JS Bridge      | Low        | Simple message passing logic.     |
| Android Shell  | Medium     | JNI and WebView configuration.    |
| iOS Shell      | Medium     | Swift/Rust interop and WKWebView. |

**Project Status**: High-priority experimental systems prototype.
