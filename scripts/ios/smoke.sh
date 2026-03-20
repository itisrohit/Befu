#!/usr/bin/env bash
set -euo pipefail

# Bun-based wrapper to ensure pathing is correct
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

echo "[ios:smoke] Running iOS simulator smoke flow..."
bun run --cwd "$ROOT_DIR" i:up

echo "[ios:smoke] App launched. Waiting for bridge initialization logs in simulator..."

TIMEOUT=45
ELAPSED=0
START_TS="$(date '+%Y-%m-%d %H:%M:%S')"

while [ $ELAPSED -lt $TIMEOUT ]; do
  # Check system logs since START_TS
  LOGS="$(xcrun simctl spawn booted log show --start "$START_TS" --predicate 'process == "Befu"' 2>/dev/null || true)"
  if echo "$LOGS" | grep -q '\[befu:bridge:ping\] pong'; then
    echo "[ios:smoke] [ok] Bridge verified: pong received from Rust."
    exit 0
  fi
  if echo "$LOGS" | grep -q '\[befu:bridge:error\]'; then
    echo "[ios:smoke] [error] Bridge initialization failed (detected in logs)."
    exit 1
  fi
  sleep 3
  ELAPSED=$((ELAPSED + 3))
done

echo "[ios:smoke] [error] Timeout waiting for bridge logs."
exit 1
