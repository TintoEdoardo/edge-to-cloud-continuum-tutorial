# Aggregating modules

We can use WebAssembly modules as building blocks for applications. 

## 1. Multi-module applications

Linking modules is a way to aggregate Wasm modules (into so-called _multi-module_ applications).

1. In [this](multi_module_app/src/main.rs) example, we are instantiating [two Wasm modules](multi_module_app/wasm_modules) with dependencies between them. 
2. As usual, ```build.sh``` and ```clean.sh``` are available within ```multi_module_app/```. 

## 2. The component model

The _component model_ is a novel mechanism for aggregating modules as Wasm _components_. <br/>
Each WebAssembly component is associated with a WIT (WebAssembly Interface Types) interface, describing <br/>
the imports and exports of the modules in it. 

A complete example of how to use components is available [here](https://component-model.bytecodealliance.org/language-support/rust.html). 

## 3. References
[1] [Wasmtime documentation - Linking Modules](https://docs.wasmtime.dev/examples-rust-linking.html) <br/>
[2] [Running WebAssembly (Wasm) Components From the Command Line](https://bytecodealliance.org/articles/invoking-component-functions-in-wasmtime-cli)