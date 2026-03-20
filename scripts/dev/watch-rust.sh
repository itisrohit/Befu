#!/usr/bin/env bash
set -e

# Watcher for Rust hot reloading.
# Detects changes in crates/bridge, crates/macros, and crates/app.
# Triggers build and sync to connected mobile devices.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

# Determine which platform to sync to (default: android)
PLATFORM="${1:-android}"

echo "[watch:rust] Starting interactive watcher for $PLATFORM..."

# Use 'cargo-watch' if available, or a simple loop
if command -v cargo-watch &> /dev/null; then
  cargo watch -w "$ROOT_DIR/crates/app" -w "$ROOT_DIR/crates/bridge" -s "bash \"$ROOT_DIR/scripts/$PLATFORM/sync-rust.sh\""
else
  echo "[watch:rust] cargo-watch not found. Falling back to simple loop (less efficient)."
  echo "[watch:rust] For the best experience, run: cargo install cargo-watch"
  
  # Initial hash using ls -lTR on the relevant crates
  GET_HASH() {
    ls -lTR "$ROOT_DIR/crates/app/src" "$ROOT_DIR/crates/bridge/src" | md5
  }
  
  LAST_HASH=$(GET_HASH)
  
  while true; do
    sleep 2
    CURRENT_HASH=$(GET_HASH)
    
    if [ "$CURRENT_HASH" != "$LAST_HASH" ]; then
      echo "[watch:rust] Changes detected. Syncing..."
      bash "$ROOT_DIR/scripts/$PLATFORM/sync-rust.sh" || true
      LAST_HASH="$CURRENT_HASH"
    fi
  done
fi
