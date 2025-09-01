#!/usr/bin/env bash

set -e

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ../voxel-for-the-web/ \
    --out-name "voxel" \
    ./target/wasm32-unknown-unknown/release/voxel.wasm
cp -a assets ../voxel-for-the-web/

