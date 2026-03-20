#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

echo "[android:smoke] Running Android emulator smoke flow..."
bun run --cwd "$ROOT_DIR" a:up

echo "[android:smoke] App launched. Waiting for bridge initialization logs..."

TIMEOUT=30
ELAPSED=0
while [ $ELAPSED -lt $TIMEOUT ]; do
  if adb logcat -d | grep -q "\[befu:bridge:ping\] pong"; then
    echo "[android:smoke] [ok] Bridge verified: pong received from Rust."
    exit 0
  fi
  if adb logcat -d | grep -q "\[befu:bridge:error\]"; then
    echo "[android:smoke] [error] Bridge initialization failed (detected in logs)."
    exit 1
  fi
  sleep 2
  ELAPSED=$((ELAPSED + 2))
done

echo "[android:smoke] [failed] Timeout waiting for bridge logs."
exit 1
