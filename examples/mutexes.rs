/// This example demonstrates the use of an arc mutex to update a shared value between threads
fn main() {
    // Create a mutex for a shared resource we want to write to
    let mutex = std::sync::Mutex::new(0);
    let arc = std::sync::Arc::new(mutex);

    // Start a timer
    let start = std::time::Instant::now();

    // Spawn 100000 threads and wait for them to finish
    let threads: Vec<_> = (0..100000).map(|_| {
        let arc = arc.clone();
        std::thread::spawn(move || {
            // Lock the mutex to write to the shared resource
            let mut data = arc.lock().unwrap();
            *data += 1;
        })
    }).collect();

    for thread in threads {
        thread.join().unwrap();
    }

    // Lock the mutex to read from the shared resource
    let data = arc.lock().unwrap();

    // Get the duration
    let duration = start.elapsed();

    // Print the result and duration
    println!("Result: {}, Duration: {:?}", *data, duration);
}