#!/usr/bin/env bash

set -euo pipefail

if lsof -iTCP:5173 -sTCP:LISTEN >/dev/null 2>&1; then
  PIDS=$(lsof -tiTCP:5173 -sTCP:LISTEN)
  if [[ -n "$PIDS" ]]; then
    echo "[android:stop] Stopping dev server on :5173"
    kill $PIDS || true
  fi
else
  echo "[android:stop] No dev server running on :5173"
fi

echo "[android:stop] Done."
