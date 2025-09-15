mod request_server;

fn main() {

    // Read the input arguments.
    let args: Vec<String> = std::env::args().collect();
    let path_to_folder: String = args[1].to_string();

    request_server::run_request(path_to_folder, 3);

    /* // Can we do better? (concurrently)
     let migrate =
         std::sync::Arc::new(std::sync::Mutex::new(false));

     // Start two threads.
     let delay: u32 = args[2].parse().unwrap();
     let mut handles : Vec<std::thread::JoinHandle<()>> = vec![];
     let request_server_handle = std::thread::spawn(move ||
        {
            // Here we are running a Wasm request.
            request_server::exec_request(path_to_folder, request_server_migrate);
        });
    handles.push(request_server_handle);

    let trigger_migrate = migrate.clone();
    let trigger_handle = std::thread::spawn(move ||
        {
            // Wait for 'delay' secs.
            std::thread::sleep(std::time::Duration::from_secs(delay as u64));

            // Trigger a migration (just a checkpoint here).
            *trigger_migrate.lock().unwrap() = true;
        });
    handles.push(trigger_handle);

    // Start all the threads.
    for handle in handles
    {
        handle.join().unwrap();
    }*/
}
