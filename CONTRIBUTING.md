# Contributing to Befu

Thanks for contributing to Befu.

## Before You Start

- Review the active roadmap: [`docs/phases-next.md`](docs/phases-next.md)
- Use `main` as the base branch
- Keep changes scoped and focused

## Local Setup

Requirements:

- Bun `>=1.2`
- Rust toolchain (`rustup`, `cargo`)
- Android/iOS toolchains when touching mobile shell code

Setup commands:

```bash
bun run doctor
bun run bootstrap
```

## Branching And Commits

- Create a feature branch from `main`
- Prefer simple Conventional Commit prefixes:
  - `feat:`
  - `fix:`
  - `docs:`
  - `chore:`

## Required Local Checks

Run before opening a PR:

```bash
bun run quality
```

## Git Hooks

Lefthook is configured in `lefthook.yml`.

- `pre-commit`: format check, lint, type-check, bridge tests, rust fmt check
- `pre-push`: full quality gate (`bun run quality`)

Install hooks if needed:

```bash
bun run hooks:install
```

## Pull Request Expectations

- Use the PR template
- Explain what changed and why
- Include validation notes (commands run + outcomes)
- Update docs for any workflow, command, or behavior changes

## CI And Review

- `Quality Gate` must pass
- Relevant platform/scaffolder CI checks must pass
- Address CodeRabbit feedback before merge
