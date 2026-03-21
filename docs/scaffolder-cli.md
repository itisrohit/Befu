# Scaffolder CLI

Befu provides a unified CLI for bootstrapping new monorepos.

## Creating a Project

```bash
bunx create-befu-app --name my-app
```

Non-interactive mode:

```bash
bunx create-befu-app --name my-app --platform both --framework react --yes
```

### Options

| Flag          | Description                  | Values                   | Default    |
| ------------- | ---------------------------- | ------------------------ | ---------- |
| `--name`      | Project directory name       | string                   | `befu-app` |
| `--framework` | Frontend framework to use    | `solid`, `react`         | `solid`    |
| `--platform`  | Target mobile platform       | `android`, `ios`, `both` | `both`     |
| `--yes`       | Skip all interactive prompts | N/A                      | N/A        |

## Project Structure

A scaffolded project contains:

- `apps/web`: The selected frontend (Solid or React).
- `crates/core`: Rust logic and binary bridge.
- `crates/app`: Dynamic side-loadable project commands.
- `android/`: Native Android project setup.
- `ios/`: Native iOS project setup (using `project.yml`).

## Next Steps In Generated Project

```bash
cd my-app
bun run bootstrap
bun run dev
```
