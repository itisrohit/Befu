# Getting Started with Befu

Befu is a high-performance framework for building cross-platform applications using Rust for business logic and React or SolidJS for the user interface. This guide provides instructions for setting up your environment and creating your first project.

## Prerequisites

Before using Befu, ensure your development machine meets the following requirements:

- **Runtime**: Bun >= 1.2
- **Compiler**: Rust toolchain (stable edition)
- **Android Support**: Android Studio, SDK/NDK, and `adb`
- **iOS Support**: Xcode 16+ and command-line tools
- **Utility**: `cargo-watch` (recommended for low-latency hot reloading)

## 1. Project Initialization

Create a new Befu project using the official scaffolder. This command sets up a complete monorepo with the required native containers and frontend workspace.

```bash
bunx create-befu-app --name my-app --framework react --platform both --yes
```

If you prefer a manual setup, you can omit the flags to enter an interactive configuration mode.

## 2. Environment Verification

Once the project is created, navigate to the project directory and run the diagnostic tools to ensure all native dependencies are correctly configured.

```bash
# Verify system dependencies
bun run doctor

# Install internal dependencies and prepare native assets
bun run bootstrap
```

## 3. Development Workflow

Befu is designed around an "Install Once, Iterate Forever" philosophy. After the initial installation on a device or simulator, business logic changes can be synced instantly without full rebuilds.

### Launching for Android

Start the unified development process which includes the Web UI server, the Rust watcher, and the Android application launcher:

```bash
bun run a:dev
```

### Launching for iOS

Execute the following to start the development cycle for iOS simulators:

```bash
bun run i:dev
```

## 4. Project Structure

A standard Befu project is organized as follows:

| Directory      | Purpose                                          |
| :------------- | :----------------------------------------------- |
| `apps/web/`    | The frontend application (React or SolidJS).     |
| `crates/app/`  | The Rust business logic and command definitions. |
| `crates/core/` | The internal Befu runtime and bridge core.       |
| `android/`     | The Android native project and JNI bridge.       |
| `ios/`         | The iOS native project and FFI bridge.           |

## 5. Quality Assurance

Maintain project health by running the integrated testing and formatting suites.

```bash
# Run all frontend and backend tests
bun run test:bridge
bun run test:rust

# Execute the full quality gate (format, lint, and type check)
bun run quality
```

## Next Steps

- **Command DX**: Learn how to define Rust commands in [command-dx.md](command-dx.md).
- **Architecture**: Understand the execution model in [architecture.md](architecture.md).
- **Hot Reload**: Deep dive into the side-loading system in [hot-reload.md](hot-reload.md).
