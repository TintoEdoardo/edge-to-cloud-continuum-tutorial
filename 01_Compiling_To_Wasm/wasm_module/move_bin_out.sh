#!/bin/bash

rm wasm_module.wasm &> /dev/null
cp target/wasm32-unknown-unknown/release/wasm_module.wasm wasm_module.wasm

