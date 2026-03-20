#!/usr/bin/env bash
set -euo pipefail

# Sync Rust commands to a running Android emulator for hot reloading.
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
APP_ID="dev.befu.app"

# Detect device ABI
ABI=$(adb shell getprop ro.product.cpu.abi | tr -d '[:space:]')
echo "[android:hot] Detected device ABI: $ABI"

case "$ABI" in
  arm64-v8a)
    TARGET="aarch64-linux-android"
    ;;
  x86_64)
    TARGET="x86_64-linux-android"
    ;;
  armeabi-v7a)
    TARGET="armv7-linux-androideabi"
    ;;
  *)
    echo "[android:hot] [error] Unsupported ABI: $ABI"
    exit 1
    ;;
esac

echo "[android:hot] Building befu-app for Android ($TARGET)..."
if command -v cargo-ndk &> /dev/null; then
  cargo ndk -t "$ABI" build --package befu-app --manifest-path "$ROOT_DIR/Cargo.toml"
else
  cargo build --package befu-app --target "$TARGET" --manifest-path "$ROOT_DIR/Cargo.toml"
fi

LIB_NAME="libbefu_app.so"
LIB_PATH="$ROOT_DIR/target/$TARGET/debug/$LIB_NAME"

# Fix for cargo-ndk output path inside target/<arch>/debug?
# Actually cargo-ndk puts it in target/<target>/debug usually.

if [ ! -f "$LIB_PATH" ]; then
    echo "[android:hot] [error] Could not find library at $LIB_PATH"
    exit 1
fi

DEST_PATH="/data/local/tmp/$LIB_NAME"

# Use a versioned filename so dlopen is forced to load fresh code
# and to avoid reading a partial file during copy.
VERSION=$(date +%s)
VERSIONED_NAME="libbefu_app_${VERSION}.so"

echo "[android:hot] Pushing library to device staging ($DEST_PATH)..."
adb push "$LIB_PATH" "$DEST_PATH"

echo "[android:hot] Moving library to app-internal directory as $VERSIONED_NAME..."
# Use run-as to discover the app sandbox and copy the library
APP_FILES_DIR=$(adb shell "run-as $APP_ID sh -c 'echo \$HOME'" | tr -d '\r')

# 1. Ensure code_cache exists
adb shell "run-as $APP_ID mkdir -p code_cache"

# 2. Copy the versioned file
adb shell "run-as $APP_ID cp $DEST_PATH $APP_FILES_DIR/code_cache/$VERSIONED_NAME"

# 3. Write the VERSION sentinel (ATOMIC update for the watcher thread)
adb shell "run-as $APP_ID sh -c \"echo $VERSIONED_NAME > code_cache/befu_hot_version\""

# 4. Cleanup old versions
adb shell "run-as $APP_ID sh -c \"find code_cache -name 'libbefu_app_*.so' ! -name '$VERSIONED_NAME' -delete\"" 2>/dev/null || true

echo "[android:hot] [ok] Library synced as $VERSIONED_NAME. Auto-reload triggered."
