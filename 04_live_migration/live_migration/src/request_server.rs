use std::io::{Read, Write};

pub fn exec_request (path_to_module_folder: String,
                     should_migrate: std::sync::Arc<std::sync::Mutex<bool>>)
{

    let path_to_module = format! ("{}module.wasm", path_to_module_folder);
    let path_to_main_memory = format! ("{}main_memory.b", path_to_module_folder);
    let path_to_checkpoint_memory = format! ("{}checkpoint_memory.b", path_to_module_folder);

    // Sharing data between module and
    // host requires a dedicated struct.
    struct MyState
    {
        wasi              : wasmtime_wasi::preview1::WasiP1Ctx,
        should_migrate    : std::sync::Arc<std::sync::Mutex<bool>>,
        main_memory_file        : Option<std::fs::File>,
        checkpoint_memory_file  : Option<std::fs::File>,
    }

    // Create the engine.
    let engine = wasmtime::Engine::default ();

    // Load the module.
    let module =
        wasmtime::Module::from_file (&engine, path_to_module)
            .expect ("Failed to load wasm file. ");

    // Create the Linker.
    let mut linker: wasmtime::Linker<MyState>  = wasmtime::Linker::new (&engine);
    wasmtime_wasi::preview1::add_to_linker_sync (&mut linker, |cx| &mut cx.wasi)
        .expect ("add_to_linker_sync failed. ");

    // Add the should_migrate function.
    linker.func_wrap ("host", "should_migrate", |caller: wasmtime::Caller<'_, MyState>|
        {
            print!("host - should migrate? ");
            let mut result    : i32   = 0;
            if *caller.data().should_migrate.lock().unwrap()
            {
                result = 1;
            }

            if result == 0
            {
                println!("no");
            }
            else
            {
                println!("yes");
            }

            result
        }
    ).expect ("func_wrap failed. ");

    // Open the file containing the main linear memory
    // or create it if missing.
    let main_memory_file_r = std::fs::OpenOptions::new ()
        .read (true)
        .create (true)
        .write (true)
        .open (&path_to_main_memory);
    let main_memory_file = match main_memory_file_r
    {
        Ok (file) => Some (file),
        Err (_) => None,
    };

    // Same for the checkpoint memory.
    let checkpoint_memory_file_r = std::fs::OpenOptions::new ()
        .read (true)
        .create (true)
        .write (true)
        .open (&path_to_checkpoint_memory);
    let checkpoint_memory_file = match checkpoint_memory_file_r
    {
        Ok (file) => Some (file),
        Err (_) => None,
    };

    // Now we have to define a callback for restoring
    // the content of the linear memories.
    // To do that, we have to pass through the module exports.
    let main_mem_export = module.get_export_index ("memory")
        .expect ("Unable to find main_mem_export. ");
    let checkpoint_mem_export = module.get_export_index ("checkpoint_memory")
        .expect("Unable to find checkpoint_mem_export. ");

    // Add the restore_memory.
    linker.func_wrap ("host", "restore_memory", move |mut caller: wasmtime::Caller<'_, MyState>|
        {
            println! ("host - restore memory");
            let main_memory = match caller.get_module_export (&main_mem_export)
            {
                Some (wasmtime::Extern::Memory (mem)) => mem,
                _ => panic! ("Failed to find host memory. "),
            };
            let main_mem_ptr = main_memory.data_ptr (&caller);

            let checkpoint_mem = match caller.get_module_export (&checkpoint_mem_export)
            {
                Some (wasmtime::Extern::Memory (mem)) => mem,
                _ => panic! ("Failed to find host checkpoint memory. "),
            };
            let checkpoint_memory_ptr = checkpoint_mem.data_ptr (&caller);

            // Restore the main memory if a checkpoint is provided.
            match &caller.data().main_memory_file
            {
                None =>
                    {
                        // Do nothing.
                    }
                Some(file) =>
                    {
                        let mut bytes = [0; 128 * 1024];
                        let mut file = file;
                        file.read (&mut bytes).expect ("Unable to read main_memory.b");
                        for i in 0..128 * 1024
                        {
                            unsafe
                                {
                                    *main_mem_ptr.wrapping_add (i) = bytes[i];
                                }
                        }
                    }
            }

            // Same for the checkpoint memory.
            match &caller.data().checkpoint_memory_file
            {
                None =>
                    {
                        // Do nothing.
                    }
                Some(file) =>
                    {
                        let mut bytes = [0; 64 * 1024];
                        let mut file = file;
                        file.read (&mut bytes).expect ("Unable to read checkpoint_memory.b");
                        for i in 0..64 * 1024
                        {
                            unsafe
                                {
                                    *checkpoint_memory_ptr.wrapping_add (i) = bytes[i];
                                }
                        }
                    }
            }
        }
    )
        .expect ("func_wrap failed. ");

    let pre = linker.instantiate_pre (&module)
        .expect ("Instantiate failed. ");

    // Create the Store.
    let wasi_ctx = wasmtime_wasi::WasiCtxBuilder::new ()
        .inherit_stdio ()
        .inherit_env ()
        .build_p1 ();

    let state = MyState
    {
        wasi              : wasi_ctx,
        should_migrate    : should_migrate.clone (),
        main_memory_file,
        checkpoint_memory_file,
    };
    let mut store = wasmtime::Store::new (&engine, state);

    // Instantiate the module.
    let instance = pre.instantiate (&mut store)
        .expect ("instantiate failed. ");

    // Invoke the start function of the module.
    let func = instance.get_func (&mut store, "_start")
        .expect ("Unable to find function _start. ");
    let mut result = [];

    let function_result = func.call (&mut store, &[], &mut result);

    // Finalize.
    match function_result
    {
        Ok (_) =>
            {
                println!("host - end of request");
                // Remove the directory.
                // std::fs::remove_dir_all (path_to_module_folder).unwrap ();
            }
        Err (error) =>
            {
                let trap = *error.downcast_ref::<wasmtime::Trap> ().unwrap ();
                if trap == wasmtime::Trap::UnreachableCodeReached
                {
                    // We have a checkpoint here!
                    println!("host - checkpoint");

                    // Read the content of the main linear memory (not very efficient!).
                    let main_linear_memory =
                        instance.get_memory (&mut store, "memory")
                            .expect ("Unable to find memory");
                    let main_lin_mem_data = main_linear_memory.data (&mut store).to_vec ();

                    // Read the content of the checkpoint memory.
                    let checkpoint_memory =
                        instance.get_memory (&mut store, "checkpoint_memory")
                            .expect ("Unable to find memory");
                    let checkpoint_data = checkpoint_memory.data (&mut store).to_vec ();

                    // Save to file the content of the two memories.
                    let mut main_memory_file = std::fs::OpenOptions::new ()
                        .create (true)
                        .write (true)
                        .open (&path_to_main_memory)
                        .expect ("Unable to open main_memory.b");
                    main_memory_file.write (&main_lin_mem_data)
                        .expect("Unable to write in main_memory.b");
                    let mut checkpoint_memory_file = std::fs::OpenOptions::new ()
                        .create (true)
                        .write (true)
                        .open (&path_to_checkpoint_memory)
                        .expect ("Unable to open checkpoint_memory.b");
                    checkpoint_memory_file.write (&checkpoint_data)
                        .expect ("Unable to write in checkpoint_memory.b");
                }
                else
                {
                    println!("host - error");
                    // Remove the directory, this is an error.
                    // std::fs::remove_dir_all (path_to_module_folder).unwrap ();
                }
            }
    }
}