#!/usr/bin/env bash
# Deploys the attestation contract's optimized wasm to a Soroban network.
#
# Usage:
#   SOROBAN_ACCOUNT=<identity> scripts/deploy.sh <network>
#
# <network> is any network configured in the Stellar CLI, e.g. testnet
# or mainnet. Requires the `stellar` CLI (https://developers.stellar.org/docs/tools/cli)
# and an identity already set up via `stellar keys add`.
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

NETWORK="${1:?usage: scripts/deploy.sh <network>}"
SOROBAN_ACCOUNT="${SOROBAN_ACCOUNT:?set SOROBAN_ACCOUNT to the deploying identity}"
WASM_PATH="target/wasm32v1-none/release/nairatax_attestation.wasm"

if [[ ! -f "${WASM_PATH}" ]]; then
  echo "wasm artifact not found at ${WASM_PATH}; run scripts/build.sh first" >&2
  exit 1
fi

stellar contract deploy \
  --wasm "${WASM_PATH}" \
  --network "${NETWORK}" \
  --source-account "${SOROBAN_ACCOUNT}"
