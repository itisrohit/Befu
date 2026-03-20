#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

check_cmd() {
  local cmd="$1"
  local label="$2"
  if command -v "$cmd" >/dev/null 2>&1; then
    echo "[ok] $label"
  else
    echo "[missing] $label"
  fi
}

echo "=== Befu Doctor ==="

echo
echo "[Core]"
check_cmd bun "Bun"
check_cmd rustup "rustup"
check_cmd cargo "cargo"
check_cmd cargo-watch "cargo-watch (recommended for hot reload)"
check_cmd rg "ripgrep (rg)"

echo
echo "[Android]"
check_cmd java "Java"
check_cmd adb "adb"
check_cmd emulator "Android emulator"
if cargo ndk --version >/dev/null 2>&1; then
  echo "[ok] cargo-ndk"
else
  echo "[missing] cargo-ndk"
fi

echo
echo "[iOS]"
check_cmd xcodebuild "xcodebuild"
check_cmd xcrun "xcrun"
check_cmd xcodegen "xcodegen"

echo
echo "[Details]"
if command -v bun >/dev/null 2>&1; then
  bun --version
fi
if command -v rustup >/dev/null 2>&1; then
  rustup --version
fi
if command -v cargo >/dev/null 2>&1; then
  cargo --version
fi
if command -v java >/dev/null 2>&1; then
  java -version || true
fi

echo
echo "Android Rust targets installed:"
rustup target list --installed | rg "android" || true

echo
echo "iOS Rust targets installed:"
rustup target list --installed | rg "apple-ios" || true

echo
echo "Connected Android devices:"
if command -v adb >/dev/null 2>&1; then
  adb devices -l || true
fi

echo
echo "Available iOS simulators:"
if command -v xcrun >/dev/null 2>&1; then
  xcrun simctl list devices available || true
fi

echo
echo "Doctor finished."
echo "Tip: run 'bun run bootstrap' to install common dependencies."
