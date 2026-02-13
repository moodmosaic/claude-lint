#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
IMAGE="claude-lint"

# Build image (Docker layer cache makes repeated runs fast).
docker build -q -t "$IMAGE" "$SCRIPT_DIR" >/dev/null

# Run with auto-cleanup, mounting the caller's working directory read-only.
docker run --rm -v "$(pwd):/workspace:ro" "$IMAGE" "$@"
