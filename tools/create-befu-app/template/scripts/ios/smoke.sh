#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

echo "[ios:smoke] Running iOS simulator smoke flow..."
bun run --cwd "$ROOT_DIR" i:up

echo "[ios:smoke] App launched. Verify UI shows:"
echo "  - Bridge is live (pong)"
echo "  - Native backend mode: ios"
