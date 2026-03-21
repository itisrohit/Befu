# Contributing to Befu

Befu is a high-performance framework for building cross-platform applications with Rust and modern web technologies. We welcome contributions that prioritize technical excellence, performance, and developer experience.

## Technical Architecture

Befu is organized as a monorepo with specialized crates and packages. Understanding this structure is essential for contributors:

| Component       | Path                     | Language | Purpose                                                      |
| :-------------- | :----------------------- | :------- | :----------------------------------------------------------- |
| **Befu Core**   | `crates/core/`           | Rust     | The internal runtime, dynamic loader, and JNI/FFI bridge.    |
| **Befu Bridge** | `packages/bridge/`       | TS       | The frontend client library used to invoke Rust commands.    |
| **Befu Hub**    | `crates/app/`            | Rust     | The default business logic crate where commands are defined. |
| **Scaffolder**  | `tools/create-befu-app/` | JS       | The CLI tool for initializing new Befu projects.             |

## Development Standards

### Prerequisites

All contributors must ensure their environment passes the automated diagnostic check:

```bash
bun run doctor
```

### Quality Assurance

Before submitting a pull request, your changes must pass the unified quality gate:

```bash
bun run quality
```

This command executes:

1.  **TypeScript Verification**: Type-checking and linting for all packages.
2.  **Rust Integrity**: `cargo fmt` check and `clippy` analysis.
3.  **Bridge Testing**: Vitest suite for the communication protocol.

## Contribution Workflow

1.  **Identify or Create an Issue**: Check the GitHub issue tracker for "Help Wanted" tasks or create a new issue to discuss proposed architectural changes.
2.  **Branching**: Create a feature branch from the `main` branch.
3.  **Implementation**: adhere to the established naming conventions and formatting rules.
4.  **Template Synchronization**: If your changes affect the core bridge or runtime, you **must** apply the corresponding updates to the project template in `tools/create-befu-app/template/`.
5.  **Quality Check**: Run `bun run quality` locally.
6.  **Pull Request**: Submit your PR with a technical description of the implementation and the problem it addresses.

## Template Synchronization Policy

Befu maintains a project template that serves as the foundation for all new applications. Ensuring this template is synchronized with the core crates is critical for ecosystem health. Pull requests that modify bridge or runtime logic without updating the template will be requested to do so before merging.

## Community Standards

Befu follows a professional engineering culture. For behavioral expectations, please refer to our [Code of Conduct](CODE_OF_CONDUCT.md).

---

Thank you for helping build the future of high-performance mobile development.
