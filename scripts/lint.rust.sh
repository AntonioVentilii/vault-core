#!/usr/bin/env bash
set -euo pipefail

cargo clippy \
  --manifest-path src/icrc-factory/Cargo.toml \
  --locked \
  --target wasm32-unknown-unknown \
  --all-features
