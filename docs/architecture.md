# Befu Architecture

Befu is designed as a **Thin-High-Performance-Bridge** connecting modern web frontends with native Rust logic.

## Platform Execution Models

Befu utilizes different execution models depending on the target environment to maximize developer velocity while maintaining native performance.

| Platform    | Execution Model       | Bridge Type    | Purpose                    |
| :---------- | :-------------------- | :------------- | :------------------------- |
| **Android** | Native Binary (`.so`) | JNI / FFI      | Production & High-Freq Dev |
| **iOS**     | Static Library (`.a`) | C FFI          | Production                 |
| **Web**     | UI Sandbox (Mock)     | Mock Transport | Rapid UI Prototyping       |

### 🌉 Native (Mobile)

On mobile devices, Befu embeds a custom Rust Core within the native application container.

- The **Frontend** (React/Solid) runs inside a high-speed WebView.
- The **Bridge** serializes calls into JSON/MessagePack and sends them across the JNI (Android) or C-FFI (iOS) boundary.
- The **Core** dispatches these calls to a dynamically-loaded App module.

### 🌐 Web (Desktop Browser)

Standard browsers cannot natively execute Rust machine code without WebAssembly (WASM).

- In this environment, the Befu Bridge enters **Sandbox Mode**.
- It provides a **Mock Transport** that allows the UI to function without a running Rust process.
- This is intended for **UI Design and State Logic Verification** without needing mobile emulators.

---

## Security & Side-Loading

### The Dev/Prod Boundary

Befu's flagship feature is **High-Frequency Hot Reload** via side-loading shared libraries.

1.  **Development Mode**: The Rust Watcher pushes fresh `.so` files to the app's internal sandbox. The Core detects these and swaps the command registry live.
2.  **Production Mode**: Dynamic loading is strictly gated behind compile-time features. Production binaries should bundle all logic statically to eliminate Remote Code Execution (RCE) risks.

### iOS Limitations

Due to App Store policies regarding `dlopen` and dynamic code execution, hot-reloading on physical iOS devices is currently restricted to **bundled logic only**. High-frequency side-loading is prioritized for Android and iOS Simulators.

---

## Future Evolution (v2.0)

Our long-term goal is to unify the logic layer across all platforms using **WebAssembly (WASM)** for the web target, allowing the exact same Rust code to run in the browser as it does on native mobile.
