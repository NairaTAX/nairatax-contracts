#!/usr/bin/env bash
# Builds the optimized release wasm for the attestation contract.
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

cargo build --target wasm32v1-none --release

WASM_PATH="target/wasm32v1-none/release/nairatax_attestation.wasm"
echo "built: ${WASM_PATH} ($(du -h "${WASM_PATH}" | cut -f1))"
