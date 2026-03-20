#!/usr/bin/env bash
set -euo pipefail

# Development script for iOS Simulator.
# Matches the feel and workflow of Android dev.sh (includes tailing logs).

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

# Ensure the Simulator app is open and a device is booted
ensure_simulator() {
  if ! xcrun simctl list devices available | grep -q " (Booted)"; then
    echo "[ios:dev] No booted simulators found. Attempting to start one..."
    # Choose a recent iPhone model
    ID=$(xcrun simctl list devices available | grep "iPhone 1" | head -1 | grep -oE "([0-9A-F-]+)")
    if [ -z "$ID" ]; then
        echo "[ios:dev] Could not find a suitable iOS simulator ID."
        exit 1
    fi
    xcrun simctl boot "$ID"
    open -a Simulator
    echo "[ios:dev] Waiting for simulator to ready up..."
    xcrun simctl bootstatus "$ID"
  else
    # Just make sure the UI is visible
    open -a Simulator
  fi
}

echo "[ios:dev] Ensuring iOS simulator is ready..."
ensure_simulator

echo "[ios:dev] Preparing assets and Rust core..."
bun run ios:prepare 

echo "[ios:dev] Building for simulator..."
bun run ios:build:sim 

echo "[ios:dev] Installing app..."
bun run ios:sim:install 

echo "[ios:dev] Launching app..."
bun run ios:sim:launch 

echo "[ios:dev] Ready. Tailing app logs (Ctrl+C to stop)..."
# Stream logs for our specific process/bundle
xcrun simctl spawn booted log stream --level debug --predicate 'process == "Befu" || category == "Befu"'
