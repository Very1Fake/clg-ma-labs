#!/usr/bin/bash
set -eu
script_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$script_path"

CRATE_NAME="ma-labs"
CRATE_NAME_SNAKE_CASE="${CRATE_NAME//-/_}"

# This is required to enable the web_sys clipboard API which egui_web uses
export RUSTFLAGS=--cfg=web_sys_unstable_apis

# Clear old build
rm -f docs/${CRATE_NAME_SNAKE_CASE}_bg.wasm
rm -f docs/${CRATE_NAME_SNAKE_CASE}.js

echo "Compiling crate..."
BUILD=final
cargo build -p ${CRATE_NAME} --profile=final --lib --target wasm32-unknown-unknown

# Get target directory
TARGET=`cargo metadata --format-version=1 | jq --raw-output .target_directory`

echo "Generating JS bindnings for wasm..."
WASM_FILE="${CRATE_NAME_SNAKE_CASE}.wasm"
wasm-bindgen "${TARGET}/wasm32-unknown-unknown/${BUILD}/${WASM_FILE}" \
    --out-dir docs --no-modules --no-typescript

echo "Optimizing wasm..."
# Requires "binaryen" (apt/brew/dnf install binaryen)
wasm-opt docs/${CRATE_NAME_SNAKE_CASE}_bg.wasm -O3 --fast-math -o docs/${CRATE_NAME_SNAKE_CASE}_bg.wasm

echo "Finished."
