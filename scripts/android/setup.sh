#!/usr/bin/env bash

set -euo pipefail

echo "[android:setup] Checking Rust Android toolchain..."

if ! cargo ndk --version >/dev/null 2>&1; then
  echo "[android:setup] Installing cargo-ndk..."
  cargo install cargo-ndk --version 4.1.2 --locked
fi

echo "[android:setup] Ensuring Rust Android targets are installed..."
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android

echo "[android:setup] Done."
