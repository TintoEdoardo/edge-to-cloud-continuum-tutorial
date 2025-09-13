# Live migration of a computation

The same WebAssembly module can run on any host exposing a Wasm runtime and satisfying its imports. 
<br/> We can implement a cross-platform checkpoint and restore mechanism. 

## 1. The state of a Wasm computation

**What is a computation?**<br/>
In this context, a computation is a function defined within a Wasm module. 

**What is the "state" of a computation?** <br/>
The state of a computation consists of (1) the Wasm stack and (2) the linear memory. <br/>
(_What is the content of the stack? Check [[1]](https://webassembly.github.io/spec/core/exec/runtime.html)._ ) 

**How can we save the state of a computation?**<br/>
Saving the linear memory is simple and can be done by the host. <br/>
For the stack, we can save the _variables_ at any point of the execution when the stack is empty, e.g., at the end of a block. <br/>
In these points, we can inject checkpoint and restore routines directly into the bytecode at compile-time.

### 1.1 Inject checkpoint and restore (C/R)

Prepare for this tutorial: 
```sh
./prepare.sh
```

For this step, you need an extended version of ```wasm-tools```. Run this script to download and build it: 
```sh
./build_wasm_tools.sh
```

Then you can inject the C/R procedures in the function with index ```12``` defined in ```3mm.wasm```:
```sh
./out/wasm_tools migrate 12 out/3mm.wasm > out/3mm_cr.wat
```

### 1.2 Live migration of a Wasm function

The next step is trying to migrate a computation. 

```sh
./build_live_migration.sh
```

Then, from ```out/```, insert C/R into a Wasm module, and try to run it:
```sh
./main "./" 10
```

How can we check if the result is correct? 

## References
[1] [WebAssembly stack](https://webassembly.github.io/spec/core/exec/runtime.html). 