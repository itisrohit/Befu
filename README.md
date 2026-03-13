# Befu

Lightweight runtime experiment for cross-platform mobile apps, with a SolidJS frontend and Rust backend connected through a small bridge layer.

## Workspace layout

- `apps/web`: SolidJS + Vite frontend
- `packages/bridge`: tiny `invoke()` API and transport contract
- `crates/core`: Rust backend core crate (`ping() -> "pong"`)
- `docs/plan.md`: project plan and roadmap

## Requirements

- Bun `>=1.2`
- Rust toolchain (`rustup`, `cargo`)

## Quick start

```bash
bun install
bun run dev
```

Open `http://localhost:5173`.

## Verify

```bash
bun run test:bridge
bun run test:rust
bun run build
```

## Code quality

```bash
# lint + format + types + tests + rust checks + build
bun run quality
```

Individual commands:

```bash
bun run lint
bun run lint:fix
bun run format
bun run format:check
bun run rust:fmt:check
bun run rust:clippy
```

## Pull request gates

- CI workflow: `.github/workflows/ci.yml`
- Required command for PRs: `bun run quality`
- CodeRabbit config: `.coderabbit.yaml`

Recommended repository settings on GitHub:

- protect `main`
- require pull request before merge
- require status checks to pass (CI / Quality Gate)
- require branch to be up to date before merge
- require at least 1 approving review

## Git hooks (block bad commits)

This repo uses Lefthook (`lefthook.yml`) as a YAML-based git hooks manager.

- `pre-commit` runs: format check, lint, type-check, bridge tests, rust format check
- `pre-push` runs: full `bun run quality`

If any step fails, commit/push is blocked.

Setup:

```bash
bun install
bun run hooks:install
```

## Current bridge flow

- `apps/web/src/App.tsx` configures an in-app transport.
- UI calls `invoke('ping')` from `@befu/bridge`.
- Transport responds with `pong`.
- Rust crate provides the backend counterpart API in `crates/core/src/lib.rs`.
