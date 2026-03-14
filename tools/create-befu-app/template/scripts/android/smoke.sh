#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

echo "[android:smoke] Running Android emulator smoke flow..."
bun run --cwd "$ROOT_DIR" a:up

echo "[android:smoke] App launched. Verify UI shows one of:"
echo "  - Native backend mode: jni"
echo "  - Native backend mode: fallback"
echo "and bridge status shows: Bridge is live (pong)"
