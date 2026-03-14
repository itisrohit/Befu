# create-befu-app

Scaffold a full Befu workspace template.

## Local usage

From repo root:

```bash
bun run create:app
```

The CLI prompts for app name and target platform (`android`, `ios`, `both`) then creates a full workspace scoped to that platform choice.

Non-interactive mode:

```bash
node tools/create-befu-app/bin/create-befu-app.mjs --name my-app --platform both --yes
```
