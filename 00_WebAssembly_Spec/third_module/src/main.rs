use wasmtime;
use wasmtime::anyhow::Result;

fn main() -> Result<()> {

    let mut store: wasmtime::Store<()> =
        wasmtime::Store::default();

    let module   : wasmtime::Module    =
        wasmtime::Module::from_file(store.engine(), "third_module.wat")?;

    let instance : wasmtime::Instance  =
        wasmtime::Instance::new(&mut store, &module, &[])?;

    let wasm_import_get : wasmtime::TypedFunc<i32, i32> =
        instance.get_typed_func::<i32, i32>(&mut store, "get_from_offset")?;

    let wasm_import_set : wasmtime::TypedFunc<(i32, i32), ()> =
        instance.get_typed_func::<(i32, i32), ()>(&mut store, "set_to_offset")?;

    let module_lin_mem : wasmtime::Memory =
        instance.get_memory(&mut store, "module_lin_mem")
            .expect("Failed to load memory");

    // Explore the memory content.
    let addr_2 : wasmtime::Result<i32> =
        wasm_import_get.call(&mut store, 2);
    assert_eq!(addr_2?, 1);

    unsafe {
        println!("Memory content is [ {:?}... ]",
                [
                    *(module_lin_mem.data_ptr(&store).wrapping_add(0)) as i32,
                    *(module_lin_mem.data_ptr(&store).wrapping_add(1)) as i32,
                    *(module_lin_mem.data_ptr(&store).wrapping_add(2)) as i32,
                    *(module_lin_mem.data_ptr(&store).wrapping_add(3)) as i32,
                    *(module_lin_mem.data_ptr(&store).wrapping_add(4)) as i32,
                ]
            );
    }

    Ok(())
}
