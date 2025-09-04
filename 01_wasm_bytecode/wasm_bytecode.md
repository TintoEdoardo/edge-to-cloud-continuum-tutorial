# WebAssembly (Wasm) Bytecode

## 1. What is Wasm?

Take a look at two examples: 
1. The first one computing the GCD (gratest common divisor) of two numbers. <br/>Here is the Wasm [module](wasm_modules/gcd.wat). To the Rust program invoking it is [this one](gcd/src/main.rs). 
2. The second example shows some actions on the linear memory. <br/>The module is [here](wasm_modules/memory.wat), the Rust main is [here](memory/src/main.rs). 


## 2. Building and running
To build the two applications run ```./build.sh``` (approx. 5GB required). <br/>
Once the building is done, you will find the applications and modules in ```out/```. <br/>
From inside ```out/``` you can run the examples with: 
1. ```./gcd```
2. ```./memory```


For cleaning up the repository (and reclaiming your 5GB) run ```./clean.sh```.   

## 3. References
[KodeKloud - Getting started with WebAssembly](https://notes.kodekloud.com/docs/Exploring-WebAssembly-WASM/Getting-Started-with-WebAssembly/Understanding-the-WebAssembly-Binary-Format) <br/>
[Mozilla Developer - Understanding the text format](https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Understanding_the_text_format)
