# create-befu-app CLI

## Install And Run

```bash
bunx create-befu-app
```

Non-interactive mode:

```bash
bunx create-befu-app --name my-befu-app --platform both --yes
```

Supported platforms:

- `android`
- `ios`
- `both`

## Deterministic Version Pinning

Use explicit versions in CI or debugging:

```bash
bunx create-befu-app@0.1.2 --name my-befu-app --platform both --yes
```

## Next Steps In Generated Project

```bash
cd my-befu-app
bun run bootstrap
bun run dev
```

## Troubleshooting

- `bunx create-befu-app` pulls an older build:
  - pin version once (`@0.1.2`) or clear local bunx cache.
- `bun install` in generated folder fails on hook install:
  - run inside a git repository, or initialize git first with `git init`.
