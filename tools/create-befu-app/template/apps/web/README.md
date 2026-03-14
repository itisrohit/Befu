# Befu Web App

SolidJS frontend for Befu, powered by Vite and managed with Bun.

## Run locally

From the repository root:

```bash
bun install
bun run dev
```

Open `http://localhost:5173`.

## Scripts

From repo root:

```bash
bun run dev
bun run build
bun run check
bun run lint
bun run format:check
```

From `apps/web` directly:

```bash
bun run dev
bun run build
```

## Current status

- `App.tsx` is wired to `@befu/bridge`.
- The UI sends `invoke('ping')` and receives `pong` through a local transport.
- This is the frontend side of the bridge contract while native shells are still pending.

## References

- SolidJS: <https://solidjs.com>
- Vite: <https://vite.dev>
