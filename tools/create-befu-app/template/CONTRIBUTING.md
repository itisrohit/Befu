# Contributing to Befu

Thanks for contributing.

## Development setup

Requirements:

- Bun `>=1.2`
- Rust toolchain (`rustup`, `cargo`)

Install dependencies and hooks:

```bash
bun install
bun run hooks:install
```

## Branching and commits

- Create feature branches from `main`
- Keep pull requests focused and small when possible
- Use Conventional Commit style when possible (for example: `feat: ...`, `fix: ...`, `chore: ...`)

## Local quality checks (required)

Before opening a PR, run:

```bash
bun run quality
```

This runs linting, formatting checks, type checks, tests, Rust checks, and web build.

## Git hooks

This repo uses Lefthook (`lefthook.yml`):

- `pre-commit`: format check, lint, type-check, bridge tests, Rust format check
- `pre-push`: full quality gate (`bun run quality`)

If hooks fail, commit or push is blocked.

## Pull requests

- Use the PR template and fill every relevant section
- Link related issue(s)
- Add test notes that explain what you validated
- Update docs when behavior or developer workflow changes

## CI and review policy

- CI `Quality Gate` must pass
- At least one approving review is required
- CodeRabbit reviews are enabled and should be addressed before merge
