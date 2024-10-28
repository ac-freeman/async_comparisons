/// This example demonstrates the use of an arc atomic to update a shared value between threads

fn main() {
    // Create an AtomicI32 for a shared resource
    let atomic = std::sync::Arc::new(std::sync::atomic::AtomicI32::new(0));

    // Start a timer
    let start = std::time::Instant::now();

    // Spawn 100000 threads and wait for them to finish
    let threads: Vec<_> = (0..100000).map(|_| {
        let atomic = atomic.clone();
        std::thread::spawn(move || {
            // Increment the atomic value
            atomic.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        })
    }).collect();

    for thread in threads {
        thread.join().unwrap();
    }

    // Read the atomic value
    let result = atomic.load(std::sync::atomic::Ordering::Relaxed);

    // Get the duration
    let duration = start.elapsed();

    // Print the result and duration
    println!("Result: {}, Duration: {:?}", result, duration);
}