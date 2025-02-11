#!/bin/bash
set -e #fail on first error

[ -e ./pkg ] && rm -rf ./pkg
mkdir ./pkg

cargo clean
wasm-pack build --dev --target web

cp ./pkg/target_wasm.js ./src
cp ./pkg/target_wasm_bg.wasm ./src
