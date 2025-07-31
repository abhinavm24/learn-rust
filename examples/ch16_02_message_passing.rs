use rust_book_examples::print_chapter_header;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    print_chapter_header("Chapter 16.2", "Using Message Passing to Transfer Data Between Threads");
    
    println!("📬 Message Passing with Channels");
    println!();
    
    demonstrate_basic_channels();
    demonstrate_multiple_producers();
    demonstrate_channel_closing();
    demonstrate_practical_examples();
}

fn demonstrate_basic_channels() {
    println!("📡 Basic Channel Communication:");
    println!();
    
    println!("\"Do not communicate by sharing memory;");
    println!("instead, share memory by communicating.\" - Go proverb");
    println!();
    
    // Create a channel
    let (tx, rx) = mpsc::channel();
    
    println!("Created channel: tx (transmitter) and rx (receiver)");
    
    // Spawn a thread to send data
    thread::spawn(move || {
        let val = String::from("Hello from thread!");
        println!("  🧵 Sending: {}", val);
        tx.send(val).unwrap();
        // val is moved and no longer accessible here
    });
    
    // Receive data in main thread
    let received = rx.recv().unwrap();
    println!("🏠 Main thread received: {}", received);
    
    println!();
    println!("📝 Channel Methods:");
    println!("• send(value) - Send a value (moves ownership)");
    println!("• recv() - Block until a value is received");
    println!("• try_recv() - Non-blocking receive (returns Result)");
    println!();
}

fn demonstrate_multiple_producers() {
    println!("🏭 Multiple Producers, Single Consumer:");
    println!();
    
    let (tx, rx) = mpsc::channel();
    
    // Clone the transmitter for multiple producers
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    
    // Producer 1
    thread::spawn(move || {
        let vals = vec![
            String::from("Message 1 from producer 1"),
            String::from("Message 2 from producer 1"),
            String::from("Message 3 from producer 1"),
        ];
        
        for val in vals {
            println!("  🏭 Producer 1 sending: {}", val);
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Producer 2
    thread::spawn(move || {
        let vals = vec![
            String::from("Message A from producer 2"),
            String::from("Message B from producer 2"),
            String::from("Message C from producer 2"),
        ];
        
        for val in vals {
            println!("  🏭 Producer 2 sending: {}", val);
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });
    
    // Drop the original transmitter
    drop(tx);
    
    // Consumer receives from all producers
    println!("🏠 Consumer receiving messages:");
    for received in rx {
        println!("  📬 Received: {}", received);
    }
    
    println!("✅ All producers finished");
    println!();
}

fn demonstrate_channel_closing() {
    println!("🔚 Channel Closing and Error Handling:");
    println!();
    
    let (tx, rx) = mpsc::channel();
    
    // Spawn thread that sends a few messages then closes
    thread::spawn(move || {
        for i in 1..=3 {
            match tx.send(format!("Message {}", i)) {
                Ok(()) => println!("  📤 Sent message {}", i),
                Err(e) => println!("  ❌ Send error: {}", e),
            }
            thread::sleep(Duration::from_millis(100));
        }
        println!("  🔚 Transmitter going out of scope (channel closes)");
    });
    
    // Receive until channel closes
    loop {
        match rx.recv() {
            Ok(msg) => println!("  📬 Received: {}", msg),
            Err(e) => {
                println!("  🔚 Receive error (channel closed): {}", e);
                break;
            }
        }
    }
    
    println!();
    println!("🔄 Non-blocking receive with try_recv():");
    
    let (tx, rx) = mpsc::channel();
    
    // Send a message after a delay
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        tx.send("Delayed message").unwrap();
    });
    
    // Try to receive immediately (will fail)
    match rx.try_recv() {
        Ok(msg) => println!("  📬 Got message: {}", msg),
        Err(mpsc::TryRecvError::Empty) => println!("  ⏳ No message available yet"),
        Err(mpsc::TryRecvError::Disconnected) => println!("  🔚 Channel disconnected"),
    }
    
    // Wait for the message
    thread::sleep(Duration::from_millis(300));
    
    // Try again
    match rx.try_recv() {
        Ok(msg) => println!("  📬 Got delayed message: {}", msg),
        Err(mpsc::TryRecvError::Empty) => println!("  ⏳ Still no message"),
        Err(mpsc::TryRecvError::Disconnected) => println!("  🔚 Channel disconnected"),
    }
    
    println!();
}

fn demonstrate_practical_examples() {
    println!("🏗️ Practical Examples:");
    println!();
    
    // Example 1: Worker Pool
    println!("1. Worker Pool Pattern:");
    worker_pool_example();
    
    println!();
    
    // Example 2: Pipeline Processing
    println!("2. Pipeline Processing:");
    pipeline_example();
    
    println!();
    
    // Example 3: Fan-out / Fan-in
    println!("3. Fan-out / Fan-in Pattern:");
    fan_out_fan_in_example();
}

fn worker_pool_example() {
    #[derive(Debug)]
    struct Job {
        id: usize,
        data: Vec<i32>,
    }
    
    #[derive(Debug)]
    struct JobResult {
        id: usize,
        result: i32,
    }
    
    let (job_tx, job_rx) = mpsc::channel::<Job>();
    let (result_tx, result_rx) = mpsc::channel::<JobResult>();
    
    // Create worker threads
    for worker_id in 0..3 {
        // let job_rx = job_rx.clone(); // This won't work! mpsc::Receiver doesn't implement Clone
        let result_tx = result_tx.clone();
        
        // We need to use Arc<Mutex<Receiver>> for shared receivers
        // For now, let's simulate with a simpler example
        println!("Worker {} would be created here", worker_id);
    }
    
    // Simplified worker pool simulation
    let jobs = vec![
        Job { id: 1, data: vec![1, 2, 3, 4, 5] },
        Job { id: 2, data: vec![10, 20, 30] },
        Job { id: 3, data: vec![100, 200] },
    ];
    
    let handles: Vec<_> = jobs
        .into_iter()
        .map(|job| {
            thread::spawn(move || {
                println!("  👷 Processing job {}", job.id);
                let sum: i32 = job.data.iter().sum();
                thread::sleep(Duration::from_millis(100));
                JobResult { id: job.id, result: sum }
            })
        })
        .collect();
    
    let results: Vec<_> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    
    for result in results {
        println!("  📊 Job {} result: {}", result.id, result.result);
    }
}

fn pipeline_example() {
    // Stage 1: Generate numbers
    let (stage1_tx, stage1_rx) = mpsc::channel();
    
    thread::spawn(move || {
        for i in 1..=10 {
            println!("  🏭 Stage 1: Generating {}", i);
            stage1_tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });
    
    // Stage 2: Square the numbers
    let (stage2_tx, stage2_rx) = mpsc::channel();
    
    thread::spawn(move || {
        for num in stage1_rx {
            let squared = num * num;
            println!("  🔧 Stage 2: {} squared = {}", num, squared);
            stage2_tx.send(squared).unwrap();
        }
    });
    
    // Stage 3: Sum the squares
    thread::spawn(move || {
        let mut sum = 0;
        for squared in stage2_rx {
            sum += squared;
            println!("  📊 Stage 3: Running sum = {}", sum);
        }
        println!("  ✅ Final sum: {}", sum);
    })
    .join()
    .unwrap();
}

fn fan_out_fan_in_example() {
    let input_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Fan-out: Distribute work to multiple processors
    let (work_tx, work_rx) = mpsc::channel::<i32>();
    let (result_tx, result_rx) = mpsc::channel::<i32>();
    
    // Start worker threads
    for worker_id in 0..3 {
        // let work_rx = work_rx.clone(); // This won't work directly  
        let result_tx = result_tx.clone();
        
        // Simulate workers processing different chunks
        let chunk_start = worker_id * 3;
        let chunk_end = std::cmp::min(chunk_start + 4, input_data.len());
        
        if chunk_start < input_data.len() {
            let chunk: Vec<_> = input_data[chunk_start..chunk_end].to_vec();
            
            thread::spawn(move || {
                println!("  👷 Worker {} processing {:?}", worker_id, chunk);
                let sum: i32 = chunk.iter().sum();
                thread::sleep(Duration::from_millis(100));
                println!("  📊 Worker {} result: {}", worker_id, sum);
                result_tx.send(sum).unwrap();
            });
        }
    }
    
    // Fan-in: Collect results
    drop(result_tx); // Close the channel after all workers are done
    
    let mut total = 0;
    for result in result_rx {
        total += result;
        println!("  📥 Collected result, running total: {}", total);
    }
    
    println!("  🎯 Final total: {}", total);
    
    println!();
    println!("💡 Channel Best Practices:");
    println!("• Use channels for communicating between threads");
    println!("• Channels transfer ownership (no data races)");
    println!("• Drop transmitters to close channels");
    println!("• Use try_recv() for non-blocking operations");
    println!("• Consider bounded channels for backpressure");
    println!("• Channels are FIFO (first-in, first-out)");
    
    println!();
    println!("🔧 Channel Types:");
    println!("• mpsc::channel() - Unbounded, async");
    println!("• mpsc::sync_channel(n) - Bounded, can block on send");
    println!("• Multiple producers, single consumer");
    println!("• For multiple consumers, use Arc<Mutex<Receiver>>");
}