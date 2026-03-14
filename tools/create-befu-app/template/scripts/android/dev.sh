#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

start_emulator_if_needed() {
  if adb devices | rg -q "\tdevice$"; then
    echo "[android:dev] Device/emulator already connected."
    return
  fi

  if emulator -list-avds | rg -q "^Pixel_7$"; then
    echo "[android:dev] Starting emulator Pixel_7..."
    nohup emulator -avd Pixel_7 >/tmp/befu-emulator.log 2>&1 &
    adb wait-for-device
  else
    echo "[android:dev] No connected device and Pixel_7 AVD not found."
    echo "[android:dev] Start a device manually and rerun this command."
    exit 1
  fi
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
echo "[android:dev] Dev server logs: /tmp/befu-dev.log"
echo "[android:dev] Emulator logs: /tmp/befu-emulator.log"
