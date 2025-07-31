use rust_book_examples::print_chapter_header;
use std::thread;
use std::time::Duration;

fn main() {
    print_chapter_header("Chapter 16.1", "Using Threads to Run Code Simultaneously");
    
    println!("🧵 Fearless Concurrency with Threads");
    println!();
    
    demonstrate_basic_threads();
    demonstrate_join_handles();
    demonstrate_move_closures();
    demonstrate_thread_communication();
}

fn demonstrate_basic_threads() {
    println!("🚀 Creating and Running Threads:");
    println!();
    
    println!("Main thread starting...");
    
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("  🧵 Thread: Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // Main thread continues
    for i in 1..5 {
        println!("🏠 Main: Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // Wait for the spawned thread to finish
    handle.join().unwrap();
    
    println!("✅ Both threads completed!");
    println!();
}

fn demonstrate_join_handles() {
    println!("🤝 Using join() to Wait for Threads:");
    println!();
    
    println!("Without join() - threads might not finish:");
    thread::spawn(|| {
        for i in 1..5 {
            println!("  🔥 Fast thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // Main thread doesn't wait - spawned thread might not complete
    println!("Main thread finished without waiting");
    thread::sleep(Duration::from_millis(10)); // Give it some time
    
    println!();
    println!("With join() - guaranteed completion:");
    
    let handles: Vec<_> = (0..3)
        .map(|i| {
            thread::spawn(move || {
                println!("  🎯 Worker thread {} starting", i);
                let work_duration = Duration::from_millis((i + 1) * 10);
                thread::sleep(work_duration);
                println!("  ✅ Worker thread {} completed", i);
                i * 2 // Return value
            })
        })
        .collect();
    
    println!("🏠 Main thread waiting for all workers...");
    
    // Collect results from all threads
    let results: Vec<_> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect();
    
    println!("📊 Results from worker threads: {:?}", results);
    println!();
}

fn demonstrate_move_closures() {
    println!("📦 Moving Data into Threads:");
    println!();
    
    let data = vec![1, 2, 3, 4, 5];
    println!("Original data: {:?}", data);
    
    // This would not compile without 'move':
    // let handle = thread::spawn(|| {
    //     println!("Data in thread: {:?}", data); // Error: may outlive borrowed value
    // });
    
    let handle = thread::spawn(move || {
        println!("  🧵 Data moved into thread: {:?}", data);
        let sum: i32 = data.iter().sum();
        println!("  🧮 Sum calculated in thread: {}", sum);
        sum
    });
    
    // data is no longer accessible here because it was moved
    // println!("{:?}", data); // This would not compile
    
    let result = handle.join().unwrap();
    println!("📊 Result from thread: {}", result);
    
    println!();
    println!("🔄 Multiple threads with different data:");
    
    let datasets = vec![
        vec![1, 2, 3],
        vec![4, 5, 6, 7],
        vec![8, 9, 10, 11, 12],
    ];
    
    let handles: Vec<_> = datasets
        .into_iter()
        .enumerate()
        .map(|(i, data)| {
            thread::spawn(move || {
                println!("  🧵 Thread {}: processing {:?}", i, data);
                let sum: i32 = data.iter().sum();
                let avg = sum as f64 / data.len() as f64;
                println!("  📊 Thread {}: sum={}, avg={:.2}", i, sum, avg);
                (sum, avg)
            })
        })
        .collect();
    
    let results: Vec<_> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect();
    
    println!("🎯 Final results: {:?}", results);
    println!();
}

fn demonstrate_thread_communication() {
    println!("📡 Thread Communication Patterns:");
    println!();
    
    // Shared state (we'll show why this doesn't work without synchronization)
    println!("❌ Why sharing mutable state is problematic:");
    println!("// let mut counter = 0;");
    println!("// thread::spawn(|| {{ counter += 1; }}); // Won't compile!");
    println!("// Rust prevents data races at compile time");
    println!();
    
    // Using message passing (covered in next chapter)
    println!("✅ Better approaches for thread communication:");
    println!("• Message passing with channels (Chapter 16.2)");
    println!("• Shared state with Mutex/Arc (Chapter 16.3)");
    println!("• Atomic operations for simple cases");
    println!();
    
    // Demonstrate thread spawning patterns
    println!("🔧 Common Thread Patterns:");
    
    // 1. Fork-Join Pattern
    println!("1. Fork-Join Pattern:");
    let data = (0..100).collect::<Vec<_>>();
    let chunk_size = data.len() / 4;
    
    let handles: Vec<_> = data
        .chunks(chunk_size)
        .enumerate()
        .map(|(i, chunk)| {
            let chunk = chunk.to_vec();
            thread::spawn(move || {
                let sum: i32 = chunk.iter().sum();
                println!("  🧵 Chunk {}: processed {} items, sum = {}", i, chunk.len(), sum);
                sum
            })
        })
        .collect();
    
    let total: i32 = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum();
    
    println!("  📊 Total sum: {}", total);
    println!();
    
    // 2. Worker Pool Pattern (simplified)
    println!("2. Worker Pool Pattern (simplified):");
    let work_items = vec!["task1", "task2", "task3", "task4", "task5"];
    
    let handles: Vec<_> = work_items
        .into_iter()
        .enumerate()
        .map(|(worker_id, task)| {
            thread::spawn(move || {
                println!("  👷 Worker {}: processing {}", worker_id, task);
                thread::sleep(Duration::from_millis(100));
                println!("  ✅ Worker {}: completed {}", worker_id, task);
                format!("result_of_{}", task)
            })
        })
        .collect();
    
    let results: Vec<_> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect();
    
    println!("  📋 All results: {:?}", results);
    println!();
    
    println!("💡 Key Thread Concepts:");
    println!("• Threads run concurrently and may be preempted");
    println!("• Use join() to wait for thread completion");
    println!("• Move semantics ensure thread safety");
    println!("• Rust prevents data races at compile time");
    println!("• Consider thread overhead for small tasks");
    
    println!();
    println!("⚡ Performance Considerations:");
    let num_cpus = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    println!("• Available CPU cores: {}", num_cpus);
    println!("• Thread creation has overhead");
    println!("• Consider thread pools for many small tasks");
    println!("• Balance parallelism with synchronization costs");
}