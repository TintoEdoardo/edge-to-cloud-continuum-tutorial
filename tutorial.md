# Tutorial
___

### Required tools

The tools used in this tutorial are the following: 
1. ```rustc``` and ```cargo```.
2. ```wasm-tools``` to analyse the Wasm bytecode. 

To install those tools, you can use the following script: 

> [!CAUTION]
> The script will uninstall rustc; hence, if you already have a Rust toolchain installed 
> on your machine, skip the first portion of the script. 

```
./install_tools.sh
```

___
### Sections

1. WebAssembly (Wasm) bytecode \[ [first tutorial](01_wasm_bytecode/wasm_bytecode.md) \]
2. From Rust to Wasm \[ [second tutorial](02_compiling_to_wasm/compiling_to_wasm.md) \]
3. Applications as aggregates of Wasm modules \[ [third tutorial](03_aggregating_modules/aggregating_modules.md) \]
4. _Live_ migrations of a Wasm module \[ [fourth tutorial](04_live_migration/live_migration.md) \]