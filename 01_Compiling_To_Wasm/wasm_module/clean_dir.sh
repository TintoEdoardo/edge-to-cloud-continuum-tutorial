#!/bin/bash

cargo clean

rm wasm_module.wasm &> /dev/null
rm wasm_module.wat &> /dev/null
