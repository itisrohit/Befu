#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

start_emulator_if_needed() {
  if adb devices | grep -q "[0-9]	device$"; then
    echo "[android:dev] Device/emulator already connected."
  elif emulator -list-avds | grep -q "^Pixel_7$"; then
    echo "[android:dev] Starting emulator Pixel_7..."
    # Start emulator in background with UI
    nohup emulator -avd Pixel_7 -gpu host >/tmp/befu-emulator.log 2>&1 &
    echo "[android:dev] Waiting for ADB connection..."
    adb wait-for-device
  else
    echo "[android:dev] No connected device and Pixel_7 AVD not found."
    echo "[android:dev] Start a device manually and rerun this command."
    exit 1
  fi

  # WAIT FOR FULL BOOT (Crucial for Gradle installDebug)
  echo "[android:dev] Waiting for system boot to complete (this may take 20s)..."
  while [ "$(adb shell getprop sys.boot_completed | tr -d '\r')" != "1" ]; do
    sleep 2
  done
  echo "[android:dev] Android is ready."
}

start_dev_server_if_needed() {
  if lsof -iTCP:5173 -sTCP:LISTEN >/dev/null 2>&1; then
    echo "[android:dev] Dev server already running on :5173"
    return
  fi

  echo "[android:dev] Starting web dev server on :5173..."
  bun run android:dev >/tmp/befu-dev.log 2>&1 &
}

echo "[android:dev] Checking Android toolchain..."
bash scripts/android/setup.sh

echo "[android:dev] Ensuring device/emulator is available..."
start_emulator_if_needed

start_dev_server_if_needed

sleep 2

echo "[android:dev] Building and installing Android debug app..."
bun run android:install:debug

echo "[android:dev] Launching app..."
bun run android:app:restart

echo "[android:dev] Ready."
echo "[android:dev] Tailing app logs (Ctrl+C to exit)..."
adb logcat -v time | rg -i "befu|WebView|chromium|ERR_"
