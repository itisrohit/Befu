# Befu

Befu is a lightweight runtime for building cross-platform mobile apps with:

- Rust backend
- WebView runtime (Android/iOS)
- modern web UI (SolidJS and beyond)

The goal is to keep apps small, fast, and simple while letting developers use the full JavaScript (Bun/npm) and Rust ecosystems.

## Architecture

```text
Frontend (SolidJS)
      ↓
invoke() bridge
      ↓
WebView shell
      ↓
Rust runtime
```

## Why Befu?

- tiny runtime footprint
- web-framework flexibility
- Rust ecosystem access
- minimal bridge surface

## Project Scope (Current)

- Prototype runtime for Android/iOS webview shells + Rust command backend
- Bun-first developer workflow and `create-befu-app` scaffolder
- Current priority: Rust command DX (`invoke("name", args)`) with minimal boilerplate
- Planned differentiator (debug-only): hot Rust command reload on Android and iOS simulator for faster iteration
- Focused on bridge ergonomics and iteration speed, not full production hardening yet

## Quick Start

```bash
bun run doctor
bun run bootstrap
bun run dev
```

Open `http://localhost:5173`.

## Scaffold A New App

Package: [create-befu-app on npm](https://www.npmjs.com/package/create-befu-app)

```bash
bunx create-befu-app --name my-befu-app --platform both --yes
```

If your local `bunx` cache is stale, pin explicitly:

```bash
bunx create-befu-app@0.1.2 --name my-befu-app --platform both --yes
```

## Status

Experimental prototype.

## Docs

- Setup and daily workflows: [`docs/getting-started.md`](docs/getting-started.md)
- Scaffolder usage and troubleshooting: [`docs/scaffolder-cli.md`](docs/scaffolder-cli.md)
- Current roadmap and priorities: [`docs/phases-next.md`](docs/phases-next.md)

---

Built with love ❤️
