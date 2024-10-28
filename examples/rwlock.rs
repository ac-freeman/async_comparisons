/// This example demonstrates the use of a RwLock to share mostly-read data between threads
fn main() {

    let rwlock = std::sync::RwLock::new(0);
    let arc = std::sync::Arc::new(rwlock);

    // Start a timer
    let start = std::time::Instant::now();

    // Spawn 100000 threads and wait for them to finish
    let threads: Vec<_> = (0..100000).map(|_| {
        let arc = arc.clone();
        std::thread::spawn(move || {
            // Acquire a read lock
            let data = arc.read().unwrap();
            // Do something local with it
            let _ = *data + 1;
        })
    }).collect();


    // Main thread acquires a write lock
    let mut data = arc.write().unwrap();
    for _ in 0..threads.len() {
        *data += 1;
    }


    // Get the duration
    let duration = start.elapsed();

    // Print the result and duration
    println!("Result: {}, Duration: {:?}", *data, duration);
}