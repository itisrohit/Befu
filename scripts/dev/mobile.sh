#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

PLATFORM="${1:-android}"

case "$PLATFORM" in
  android)
    echo "[dev:mobile] Launching Android flow..."
    bun run --cwd "$ROOT_DIR" a:up
    ;;
  ios)
    echo "[dev:mobile] Launching iOS flow..."
    bun run --cwd "$ROOT_DIR" i:up
    ;;
  both)
    echo "[dev:mobile] Launching Android then iOS..."
    bun run --cwd "$ROOT_DIR" a:up
    bun run --cwd "$ROOT_DIR" i:up
    ;;
  *)
    echo "[dev:mobile] Unknown platform: $PLATFORM"
    echo "Usage: bun run dev:mobile [android|ios|both]"
    exit 1
    ;;
esac

echo "[dev:mobile] Done."
