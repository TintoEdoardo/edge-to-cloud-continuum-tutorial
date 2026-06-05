use wasmtime;
use wasmtime::anyhow::Result;

fn main() -> Result<()> {

    let mut store: wasmtime::Store<()> =
        wasmtime::Store::default();

    let module   : wasmtime::Module    =
        wasmtime::Module::from_file(store.engine(), "second_module.wat")?;

    let print    : wasmtime::Func      =
        wasmtime::Func::wrap(&mut store, |param_1: i32| -> () {
            println!("param_1 = {}", param_1);
        });

    let instance : wasmtime::Instance  =
        wasmtime::Instance::new(&mut store, &module, &[print.into()])?;

    let wasm_import_function : wasmtime::TypedFunc<(),()> =
        instance.get_typed_func::<(),()>(&mut store, "main_function")?;

    let result : wasmtime::Result<()> =
        wasm_import_function.call(&mut store, ());

    assert!(result.is_ok());

    Ok(())
}
