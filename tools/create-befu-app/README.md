# create-befu-app

Scaffold a full Befu workspace template.

## Local usage

From repo root:

```bash
bun run create:app
```

Publish-ready usage target:

```bash
bun create befu-app
# or
bunx create-befu-app
```

The CLI prompts for app name and target platform (`android`, `ios`, `both`) then creates a full workspace scoped to that platform choice.

Note: published package includes an embedded template snapshot under `template/` so scaffolding works outside the source repository.

Non-interactive mode:

```bash
node tools/create-befu-app/bin/create-befu-app.mjs --name my-app --platform both --yes
```
