#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

echo "[ios:smoke] Running iOS simulator smoke flow..."
bun run --cwd "$ROOT_DIR" i:up

echo "[ios:smoke] App launched. Waiting for bridge initialization logs in simulator..."

TIMEOUT=30
ELAPSED=0

while [ $ELAPSED -lt $TIMEOUT ]; do
  # Check system logs for bridge pong log
  if xcrun simctl spawn booted log show --last 5m --predicate 'process == "Befu"' | grep -q '\[befu:bridge:ping\] pong'; then
    echo "[ios:smoke] [ok] Bridge verified: pong received from Rust."
    exit 0
  fi
  sleep 2
  ELAPSED=$((ELAPSED + 2))
done

echo "[ios:smoke] [failed] Timeout waiting for bridge logs."
exit 1
