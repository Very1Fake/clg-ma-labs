#!/usr/bin/bash
set -eu

# Pre-requisites:
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.78  # Version without bug
cargo update -p wasm-bindgen
