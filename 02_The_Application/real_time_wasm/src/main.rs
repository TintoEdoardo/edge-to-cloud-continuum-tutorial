use std::thread::JoinHandle;

fn main() {

    let mut task_handles : Vec<JoinHandle<()>> = Vec::new();
    let task_1_handle : std::thread::JoinHandle<()> = std::thread::spawn(||
        unsafe {

            // Task parameters.
            let period   : i32 = 500_000;
            let priority : i32 = 20;

            println!("Task 1 - Initialisation started. ");

            // Linux API to interact with the scheduler.
            let tid : libc::pid_t = libc::getpid();
            let sched_param :libc::sched_param = libc::sched_param {
                sched_priority : priority,
            };

            // Set a scheduling policy.
            libc::sched_setscheduler(tid, libc::SCHED_FIFO, &sched_param);

            // Fix the core affinity for the task.
            let mut cpuset : libc::cpu_set_t = std::mem::zeroed ();
            libc::CPU_ZERO (&mut cpuset);
            libc::CPU_SET (8, &mut cpuset);
            libc::sched_setaffinity (tid, size_of::<libc::cpu_set_t> (), &cpuset);

            println!("Task 1 - Initialisation completed, tid = {}. ", tid);

            // The first activation happens immediately.
            let mut next_activation : libc::timespec = libc::timespec {
                tv_sec: 0,
                tv_nsec: 0
            };
            libc::clock_gettime (libc::CLOCK_MONOTONIC, &mut next_activation);

            // The task body.
            loop {

                println!("Task 1 - New activation at {:?}. ", next_activation);

                // Compute the next activation time.
                // This step misbehaves if the period is >= 1_000_000_000.
                next_activation.tv_nsec += (period * 1000) as i64;
                if next_activation.tv_nsec > 1_000_000_000
                {
                    next_activation.tv_sec += 1;
                    next_activation.tv_nsec -= 1_000_000_000;
                }

                // WebAssembly execution.
                let mut store: wasmtime::Store<()> =
                    wasmtime::Store::default();

                let module   : wasmtime::Module    =
                    wasmtime::Module::from_file(store.engine(), "module.wat")
                        .expect("Module creation failed. ");

                let print    : wasmtime::Func      =
                    wasmtime::Func::wrap(&mut store, |param_1: i32| -> () {
                        println!("param_1 = {}", param_1);
                    });

                let instance : wasmtime::Instance  =
                    wasmtime::Instance::new(&mut store, &module, &[print.into()])
                        .expect("Instance cration failed. ");

                let wasm_import_function : wasmtime::TypedFunc<(),()> =
                    instance.get_typed_func::<(),()>(&mut store, "main_function")
                        .expect("Typed function not found. ");

                let _result : wasmtime::Result<()> =
                    wasm_import_function.call(&mut store, ());


                // Sleep until the next activation time.
                libc::clock_nanosleep (libc::CLOCK_MONOTONIC,
                                       libc::TIMER_ABSTIME,
                                       &next_activation,
                                       core::ptr::null_mut ());

            }
        }
    );
    task_handles.push(task_1_handle);

    for task_handle in task_handles {
        task_handle.join().unwrap();
    }

}
