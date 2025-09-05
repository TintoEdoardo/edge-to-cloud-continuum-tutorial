# Compiling to Wasm

## 1. From Rust to Wasm

Several programming languages might be compiled to Wasm. <br/>
In this tutorial, we will use Rust (but check the references for a similar example with C). 

Now we are going to write a minimal Wasm [module](wasm_modules/src/lib.rs):
1. Firstly, without any imported function from the host. 
2. Then, importing a `clock` function. 

The compilation will require approximately 3GB. You can perform the building with ```./build.sh```. <br/>
For cleaning up use ```./clean.sh```.

## 2. Extending a module

To interact with the host environment, Wasm requires access to imported host functions (remember sandboxing?). <br/>
How can we expose functionalities for Wasm modules? Take a look at the [host component](host_component/src/main.rs). 

> A standard set of API exists: the WebAssembly System Interface ([WASI](https://wasi.dev/)). 

## 3. References
[1] [Surma - Rust to WebAssembly the hard way](https://surma.dev/things/rust-to-webassembly/) <br/>
[2] [Surma - Compiling C to WebAssembly without Emscripten](https://surma.dev/things/c-to-webassembly/)