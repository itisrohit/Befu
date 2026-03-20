#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "[bootstrap] Installing JS dependencies..."
bun install

echo "[bootstrap] Installing git hooks..."
bun run --cwd "$ROOT_DIR" hooks:install

echo "[bootstrap] Setting up Android Rust toolchain..."
bash "$ROOT_DIR/scripts/android/setup.sh"

echo "[bootstrap] Preparing iOS assets and Rust library..."
bun run --cwd "$ROOT_DIR" ios:prepare

echo "[bootstrap] Done."
echo "Next:"
echo "  - bun run dev"
echo "  - bun run a:up"
echo "  - bun run a:hot (Hot reload Android - requires cargo-watch)"
echo "  - bun run i:up"
echo "  - bun run i:hot (Hot reload iOS - requires cargo-watch)"
echo
echo "Tip: Run 'cargo install cargo-watch' for the best hot-reload experience."
