# Laboratory
___

### Objectives
1. Practice the use of WebAssembly
   1. Dealing with the Wasm bytecode
   2. Compiling (from Rust) to Wasm
   3. Aggregating Wasm modules into applications


2. Migrating a WebAssembly computation
    1. Discussing the prerequisites for live migration
    2. Familiarizing with a checkpoint and restore mechanism for Wasm computations
    3. Migrating a computation

---

### Outline of the activity

1. WebAssembly (Wasm) bytecode \[ [first laboratory](01_wasm_bytecode/wasm_bytecode.md) \]
2. From Rust to Wasm \[ [second laboratory](02_compiling_to_wasm/compiling_to_wasm.md) \]
3. Applications as aggregates of Wasm modules \[ [third laboratory](03_aggregating_modules/aggregating_modules.md) \]
4. _Live_ migrations of a Wasm module \[ [fourth laboratory](04_live_migration/live_migration.md) \]


---

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