# Befu

Befu is a mobile runtime for building Rust-backed applications. It is dedicated to developer iteration velocity, combining standard Web UIs (SolidJS/Vite) with the raw performance and ecosystem of Rust.

## Hot Rust Command Reload

The core feature of Befu is **Hot Rust Command Reloading** (Debug Only). Sync Rust code changes to your mobile device or simulator in near real-time without full builds or app reinstalls.

## Architecture

```mermaid
graph TD
    UI[Web UI / SolidJS] -->|invoke| Bridge[Befu Bridge / TS]
    Bridge -->|WebView Messaging| Core[Befu Core / Rust]
    Core -->|Hot Dispatch| App[Befu App / Rust Logic]
    Watcher[Rust Watcher / Scripts] -->|Sync .dylib| Files[App Sandbox]
```

## Project Status

**Phase 2 Stable**: Procedural macro command registry and dynamic hot-reloading are fully functional.

We are currently looking for builders to help with **Phase 3 (iOS device support)** and **Phase 4 (Android Production Hardening)**. Help us take Befu from a prototype to a production-ready release.

## Quick Start

### 1. Check Requirements

Befu requires Bun, Rust, and platform-specific tools:

```bash
bun run doctor
```

### 2. Bootstrap Workspace

Install dependencies, git hooks, and prepare platform-specific assets:

```bash
bun run bootstrap
```

### 3. Launch Development

Start the web development server:

```bash
bun run dev
```

Or launch directly on mobile (requires emulator/simulator):

```bash
bun run dev:mobile android  # or ios
```

**Hot Reloading (Debug Only):**
Sync Rust code changes to your device in real-time without full rebuilds:

```bash
bun run a:hot   # For Android (watcher)
bun run i:hot   # For iOS (watcher)
```

Then click the **🔄 Reload Rust** button in the app.

## Scaffold A New App

Package: [create-befu-app on npm](https://www.npmjs.com/package/create-befu-app)

```bash
bunx create-befu-app --name my-befu-app --platform both --yes
```

If your local `bunx` cache is stale, pin explicitly:

```bash
bunx create-befu-app@0.1.3 --name my-befu-app --platform both --yes
```

## Status

Experimental prototype.

## Docs

- Setup and daily workflows: [`docs/getting-started.md`](docs/getting-started.md)
- Scaffolder usage and troubleshooting: [`docs/scaffolder-cli.md`](docs/scaffolder-cli.md)
- Current roadmap and priorities: [`docs/phases-next.md`](docs/phases-next.md)
- Rust Command DX guide: [`docs/command-dx.md`](docs/command-dx.md)
- Hot Command Reload guide: [`docs/hot-reload.md`](docs/hot-reload.md)

---

Built with love ❤️
