#!/usr/bin/env bash
set -euo pipefail

# Sync Rust commands to a running iOS Simulator for hot reloading.
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
APP_ID="dev.befu.ios"

echo "[ios:hot] Building befu-app for iOS Simulator (x86_64/arm64)..."
# In dev, we target the host arch usually (arm64 for Apple Silicon)
# We can use a single target for speed.
ARCH="$(uname -m)"
if [ "$ARCH" == "arm64" ]; then
    TARGET="aarch64-apple-ios-sim"
else
    # There is no x86_64-apple-ios-sim, only x86_64-apple-ios (always simulator)
    TARGET="x86_64-apple-ios"
fi

cargo build --package befu-app --target "$TARGET"

LIB_NAME="libbefu_app.dylib"
LIB_PATH="$ROOT_DIR/target/$TARGET/debug/$LIB_NAME"

echo "[ios:hot] Locating app container (bundle) for $APP_ID..."
CONTAINER_DIR="$(xcrun simctl get_app_container booted "$APP_ID" app)"

if [ -z "$CONTAINER_DIR" ]; then
    echo "[ios:hot] [error] Could not find app container. Is the app installed and the simulator booted?"
    exit 1
fi

DEST_DIR="$CONTAINER_DIR" # App-bundle root for fast ./ lookup

# Use a versioned filename so dlopen is forced to load fresh code.
# Without this, dlopen caches the old library handle even after the file changes.
VERSION=$(date +%s)
VERSIONED_NAME="libbefu_app_${VERSION}.dylib"

echo "[ios:hot] Copying library as $VERSIONED_NAME..."
cp "$LIB_PATH" "$DEST_DIR/$VERSIONED_NAME"
# Ad-hoc sign for the simulator (mandatory for M1/M2)
codesign -f -s - "$DEST_DIR/$VERSIONED_NAME"

# Write the current version to a sentinel file the Rust engine reads
echo "$VERSIONED_NAME" > "$DEST_DIR/befu_hot_version"

# Clean up old versioned libraries (keep only the latest)
find "$DEST_DIR" -name "libbefu_app_*.dylib" ! -name "$VERSIONED_NAME" -delete 2>/dev/null || true

echo "[ios:hot] [ok] Library synced as $VERSIONED_NAME. Call 'befu.reload' to apply."
