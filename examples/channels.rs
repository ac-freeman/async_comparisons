/// This example demonstrates the use of mpsc channels to update a shared value between threads
/// without reading
fn main() {
    // Create a channel for sending value updates
    let (tx, rx) = std::sync::mpsc::channel();

    // Start a timer
    let start = std::time::Instant::now();

    // Spawn 100000 threads and wait for them to finish
    let threads: Vec<_> = (0..100000).map(|_| {
        let tx = tx.clone();
        std::thread::spawn(move || {
            // Send a value update through the channel
            tx.send(1).unwrap();
        })
    }).collect();


    // Continually listen on the rx to update the sum
    let mut sum = 0;
    for _ in 0..threads.len() {
        sum += rx.recv().unwrap();
    }


    // Get the duration
    let duration = start.elapsed();

    // Print the result and duration
    println!("Result: {}, Duration: {:?}", sum, duration);

}