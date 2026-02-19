#!/bin/bash
set -euxo pipefail

# Ensure we are in project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/.."

# 1. Build canisters (required for tests to find them)
# Using release profile as integration tests currently look for release build
cargo build -p bucket -p directory -p test_proxy --target wasm32-unknown-unknown --release

# 2. Setup PocketIC
export POCKET_IC_SERVER_VERSION=7.0.0
export POCKET_IC_SERVER_PATH="target/pocket-ic"
export POCKET_IC_BIN="${PWD}/${POCKET_IC_SERVER_PATH}"
export POCKET_IC_MUTE_SERVER=""

scripts/pic-install

# 3. Run tests
echo "Running integration tests."
cargo test -p it ${1+"$@"}
