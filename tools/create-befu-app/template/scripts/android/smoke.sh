#!/usr/bin/env bash
set -e

# Bun-based wrapper to ensure pathing is correct
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

echo "[android:smoke] Running Android emulator smoke flow..."
bun run --cwd "$ROOT_DIR" a:up

echo "[android:smoke] App launched. Waiting for bridge initialization logs..."

TIMEOUT=30
ELAPSED=0
START_TS="$(date '+%m-%d %H:%M:%S.000')"

while [ $ELAPSED -lt $TIMEOUT ]; do
  LOGS="$(adb logcat -d -t "$START_TS" 2>/dev/null || true)"
  if echo "$LOGS" | grep -q "\[befu:bridge:ping\] pong"; then
    echo "[android:smoke] [ok] Bridge verified: pong received from Rust."
    exit 0
  fi
  if echo "$LOGS" | grep -q "\[befu:bridge:error\]"; then
    echo "[android:smoke] [error] Bridge initialization failed (detected in logs)."
    exit 1
  fi
  sleep 2
  ELAPSED=$((ELAPSED + 2))
done

echo "[android:smoke] [error] Timeout waiting for bridge logs."
exit 1
