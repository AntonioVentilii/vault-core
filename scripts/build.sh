#!/usr/bin/env bash

cargo build --locked --target wasm32-unknown-unknown --release -p bucket

cargo build --locked --target wasm32-unknown-unknown --release -p directory
