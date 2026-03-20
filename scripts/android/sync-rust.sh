#!/usr/bin/env bash
set -e

# Sync Rust commands to a running Android emulator for hot reloading.
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
APP_ID="dev.befu.app"

# Detect device ABI
ABI=$(adb shell getprop ro.product.cpu.abi)
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
  cargo ndk -t "$ABI" build --package befu-app
else
  cargo build --package befu-app --target "$TARGET"
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

echo "[android:hot] Pushing library to device staging ($DEST_PATH)..."
adb push "$LIB_PATH" "$DEST_PATH"

echo "[android:hot] Moving library to app-internal directory..."
# Use run-as to get into the app's private sandbox
adb shell "run-as $APP_ID cp $DEST_PATH /data/user/0/$APP_ID/files/$LIB_NAME"

echo "[android:hot] [ok] Library synced. Call 'befu.reload' from the bridge to apply."
