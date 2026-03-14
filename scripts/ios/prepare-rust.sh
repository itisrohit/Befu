#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
IOS_LIB_DIR="$ROOT_DIR/ios/App/Libraries"

# Simulator-only output for current CI and local simulator runs.
# Device target packaging can be added when on-device iOS flow is introduced.
SIM_TARGET_ARM64="aarch64-apple-ios-sim"
SIM_TARGET_X86_64="x86_64-apple-ios"

echo "[ios:prepare-rust] Ensuring Rust iOS simulator targets are installed..."
rustup target add "$SIM_TARGET_ARM64" "$SIM_TARGET_X86_64"

echo "[ios:prepare-rust] Building Rust core static library (arm64 simulator)..."
cargo build --manifest-path "$ROOT_DIR/crates/core/Cargo.toml" --target "$SIM_TARGET_ARM64" --release

echo "[ios:prepare-rust] Building Rust core static library (x86_64 simulator)..."
cargo build --manifest-path "$ROOT_DIR/crates/core/Cargo.toml" --target "$SIM_TARGET_X86_64" --release

mkdir -p "$IOS_LIB_DIR"
lipo -create \
  "$ROOT_DIR/target/$SIM_TARGET_ARM64/release/libbefu_core.a" \
  "$ROOT_DIR/target/$SIM_TARGET_X86_64/release/libbefu_core.a" \
  -output "$IOS_LIB_DIR/libbefu_core.a"

echo "[ios:prepare-rust] Synced $IOS_LIB_DIR/libbefu_core.a"
