use wasmtime::*;

fn main() -> Result<()> {

    // Get the name of the module to run
    // (as CLI input).
    let args : Vec<String> = std::env::args().collect();
    let path_to_module : &String = &args[1];

    let start_time = std::time::Instant::now();

    // Create the engine.
    let engine = wasmtime::Engine::default();
    let module = wasmtime::Module::from_file(&engine, path_to_module)?;

    // Create the Linker.
    let mut linker: wasmtime::Linker<wasmtime_wasi::preview1::WasiP1Ctx>  = wasmtime::Linker::new(&engine);
    wasmtime_wasi::preview1::add_to_linker_sync(&mut linker, |cx| cx)?;

    // Add the `clock` function.
    linker.func_wrap("host", "clock", move ||
        {
            start_time.elapsed().as_nanos() as u64
        }
    ).expect ("func_wrap failed. ");

    // Pre-instantiation of the module.
    let pre = linker.instantiate_pre(&module)?;

    // Create the Store.
    let wasi_ctx = wasmtime_wasi::p2::WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_env()
        .build_p1();
    let mut store = wasmtime::Store::new(&engine, wasi_ctx);

    // Instantiate the module.
    let instance = pre.instantiate(&mut store)?;

    // Export the `add` function of the module.
    let func = instance.get_func(&mut store, "add")
        .expect("Unable to find function `add`. ");

    // Invoke the function.
    let mut params = [wasmtime::Val::I64(6), wasmtime::Val::I64(27)];
    let mut result = [wasmtime::Val::I64(0)];
    func.call(&mut store, &mut params, &mut result)?;

    // Then print the result.
    println!("6 + 27 = {:?}", result.first().unwrap().i64().unwrap());

    Ok(())
}