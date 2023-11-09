#!/bin/sh
near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/neramind.wasm
# near deploy --wasmFile ./target/wasm32-unknown-unknown/release/neramind.wasm