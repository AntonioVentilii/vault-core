#!/usr/bin/env bash

cargo build --locked --target wasm32-unknown-unknown --release -p bucket

gzip -c target/wasm32-unknown-unknown/release/bucket.wasm >target/wasm32-unknown-unknown/release/bucket.wasm.gz

cargo build --locked --target wasm32-unknown-unknown --release -p directory

gzip -c target/wasm32-unknown-unknown/release/directory.wasm >target/wasm32-unknown-unknown/release/directory.wasm.gz
