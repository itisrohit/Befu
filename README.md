# Befu

Lightweight cross-platform runtime prototype: SolidJS frontend, Rust core, Android/iOS webview shells, and a typed bridge.

## Quick Start

```bash
bun run doctor
bun run bootstrap
bun run dev
```

Open `http://localhost:5173`.

## Scaffold A New App

```bash
bunx create-befu-app
```

Package: [create-befu-app on npm](https://www.npmjs.com/package/create-befu-app)

Non-interactive:

```bash
bunx create-befu-app --name my-befu-app --platform both --yes
```

If your local `bunx` cache is stale, pin explicitly:

```bash
bunx create-befu-app@0.1.2 --name my-befu-app --platform both --yes
```

## Core Commands

```bash
bun run quality
bun run a:up
bun run i:up
```

## CI/CD

- Main CI: [`.github/workflows/ci.yml`](.github/workflows/ci.yml)
- Scaffolder CI: [`.github/workflows/scaffolder-ci.yml`](.github/workflows/scaffolder-ci.yml)

## Docs

- Setup and daily workflows: [`docs/getting-started.md`](docs/getting-started.md)
- Scaffolder usage and troubleshooting: [`docs/scaffolder-cli.md`](docs/scaffolder-cli.md)
- Current roadmap and next priorities: [`docs/phases-next.md`](docs/phases-next.md)
