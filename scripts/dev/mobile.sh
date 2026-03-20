#!/usr/bin/env bash
set -euo pipefail

# Unified Mobile Dev Cycle (Web Server + Rust Watcher + App Logs)
# This allows running the entire stack in ONE terminal tab.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

PLATFORM="${1:-android}"

# --- UNIFIED DEV WORKER ---
# This ensures that when you Ctrl+C out of the logs, 
# the background web server and watcher are also killed.

cleanup() {
  echo ""
  echo "[dev:mobile] Shutting down background workers..."
  [ -n "${SERVER_PID:-}" ] && kill "$SERVER_PID" 2>/dev/null || true
  [ -n "${WATCHER_PID:-}" ] && kill "$WATCHER_PID" 2>/dev/null || true
}
trap cleanup EXIT

echo "[dev:mobile] Launching $PLATFORM full dev cycle..."

# 1. Start Web Dev Server (Background)
# Redirecting to /tmp to keep your active terminal clean for logs
echo "[dev:mobile] Starting Web Server (logs at /tmp/befu-web.log)..."
# Use 'dev' script from web app
bun run dev > /tmp/befu-web.log 2>&1 &
SERVER_PID=$!

# 2. Start Rust Watcher (Background)
echo "[dev:mobile] Starting Rust Watcher (logs at /tmp/befu-rust.log)..."
bun run watch:rust "$PLATFORM" > /tmp/befu-rust.log 2>&1 &
WATCHER_PID=$!

sleep 1 

# 3. Start App Logs (Foreground - Persistent)
case "$PLATFORM" in
  android)
    echo "[dev:mobile] Initializing Android..."
    bun run a:up
    ;;
  ios)
    echo "[dev:mobile] Initializing iOS..."
    bun run i:up
    ;;
  *)
    echo "[dev:mobile] Unknown platform: $PLATFORM"
    exit 1
    ;;
esac
