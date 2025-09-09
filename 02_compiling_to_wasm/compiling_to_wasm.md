# Compiling to Wasm

## 1. From Rust to Wasm

WebAssembly is a compilation target for several programming languages. <br/>
In this tutorial, we will use Rust (in the references, you will find a similar example with C). 

We are going to write a minimal Wasm [module](wasm_modules/src/lib.rs):
1. Initially, without any import from the outside environment (also known as 'host'). 
2. Then, with an imported `clock` function from the host. 

Building the module and the Rust 'host' will require approximately 3GB. <br/>
To start building, launch ```./build.sh``` (or ```./clean.sh``` to remove the compilation artifacts).

## 2. Extending a module

A Wasm module needs imported host functions to interact with the external environment (remember sandboxing?). <br/>
How does importing work? Take a look at the [host component](host_component/src/main.rs). 

> A standard API set exists: the WebAssembly System Interface ([WASI](https://wasi.dev/)). 

## 3. References
[1] [Surma - Rust to WebAssembly the hard way](https://surma.dev/things/rust-to-webassembly/) <br/>
[2] [Surma - Compiling C to WebAssembly without Emscripten](https://surma.dev/things/c-to-webassembly/)