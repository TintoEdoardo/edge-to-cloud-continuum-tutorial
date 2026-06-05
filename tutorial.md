# Laboratory
___

### Objectives
1. Practice the use of WebAssembly
   1. The Wasm bytecode, its constructs, and its properties
   2. Compiling (from Rust) to WebAssembly
2. Using WebAssembly within an application
   1. Running Wasm in a real-time task
3. Live migration with WebAssembly
   1. Instrumenting the bytecode to perform checkpoints
   2. Resuming a Wasm computation

---

### Outline of the activity

1. WebAssembly (Wasm) bytecode \[ [first part](00_WebAssembly_Spec) \]
2. From Rust to Wasm \[ [second part](01_Compiling_To_Wasm) \]
3. Applications as aggregates of Wasm modules \[ [third part](02_The_Application) \]

---

### Required tools

The tools used in this tutorial are the following: 
1. ```rustc``` and ```cargo```.
2. ```wasm-tools``` to analyse the Wasm bytecode. 

To install those tools, you can use the following script: 

> [!CAUTION]
> The script will uninstall rustc; hence, if you already have a Rust toolchain installed 
> on your machine, skip the first portion of the script. 

```shell
./install_tools_1.sh
```
```shell
./install_tools_2.sh
```
___