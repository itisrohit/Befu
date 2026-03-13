#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

echo "=== Befu Android Doctor ==="

echo "[1/4] Java"
java -version || true

echo
echo "[2/4] cargo-ndk"
if cargo ndk --version >/dev/null 2>&1; then
  cargo ndk --version
else
  echo "cargo-ndk not installed"
fi

echo
echo "[3/4] Rust Android targets"
rustup target list --installed | rg "android" || true

echo
echo "[4/4] Devices"
adb devices -l || true

echo
echo "Doctor finished."
