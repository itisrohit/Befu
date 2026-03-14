#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
IOS_WEB_DIR="$ROOT_DIR/ios/App/Resources/web"

echo "[ios:prepare-assets] Building web assets..."
bun run --cwd "$ROOT_DIR/apps/web" build

echo "[ios:prepare-assets] Syncing assets to iOS resources..."
rm -rf "$IOS_WEB_DIR"
mkdir -p "$IOS_WEB_DIR"
cp -R "$ROOT_DIR/apps/web/dist/." "$IOS_WEB_DIR/"

touch "$IOS_WEB_DIR/.gitkeep"

echo "[ios:prepare-assets] Done."
