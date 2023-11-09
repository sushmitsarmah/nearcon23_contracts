#!/bin/sh

echo ">> Deploying contract"

near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/neramind.wasm
# near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/libpayment.wasm
# near deploy --wasmFile ./target/wasm32-unknown-unknown/release/neramind.wasm