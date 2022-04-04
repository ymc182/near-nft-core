#!/bin/bash
set -e
source flags.sh
cd nft
cargo build --all --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ../out/main.wasm
cd ..
