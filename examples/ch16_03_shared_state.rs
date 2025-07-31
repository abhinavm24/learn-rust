use rust_book_examples::print_chapter_header;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    print_chapter_header("Chapter 16.3", "Shared-State Concurrency");
    
    println!("🔒 Shared State Concurrency with Mutex and Arc");
    println!();
    
    demonstrate_mutex_basics();
    demonstrate_arc_mutex_combination();
    demonstrate_rwlock();
    demonstrate_deadlock_prevention();
    demonstrate_practical_examples();
}

fn demonstrate_mutex_basics() {
    println!("🔐 Mutex (Mutual Exclusion) Basics:");
    println!();
    
    println!("Mutex ensures only one thread can access data at a time");
    println!("Two main rules:");
    println!("1. Must acquire the lock before using the data");
    println!("2. Must release the lock when done");
    println!();
    
    let m = Mutex::new(5);
    println!("Created Mutex with value: 5");
    
    {
        let mut num = m.lock().unwrap();
        println!("Acquired lock, current value: {}", *num);
        *num = 6;
        println!("Modified value to: {}", *num);
    } // lock is automatically released when `num` goes out of scope
    
    println!("Lock released, final value: {:?}", m);
    
    println!();
    println!("🔓 Lock Guard:");
    println!("• lock() returns a MutexGuard");
    println!("• Guard implements Drop to release lock automatically");
    println!("• Deref trait allows treating guard like the contained data");
    println!("• Lock is released when guard goes out of scope");
    println!();
}

fn demonstrate_arc_mutex_combination() {
    println!("🤝 Arc<Mutex<T>> - Shared Ownership + Thread Safety:");
    println!();
    
    println!("Problem: Mutex alone can't be shared between threads");
    println!("Solution: Arc (Atomically Reference Counted) provides shared ownership");
    println!();
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    println!("Starting 10 threads to increment counter...");
    
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("  🧵 Thread {} incremented counter to {}", i, *num);
            thread::sleep(Duration::from_millis(10));
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("📊 Final counter value: {}", *counter.lock().unwrap());
    println!("✅ All threads completed successfully!");
    
    println!();
    println!("🏗️ More Complex Example - Shared Counter with Statistics:");
    
    #[derive(Debug)]
    struct Stats {
        count: usize,
        sum: i64,
        max: i64,
        min: i64,
    }
    
    impl Stats {
        fn new() -> Self {
            Stats {
                count: 0,
                sum: 0,
                max: i64::MIN,
                min: i64::MAX,
            }
        }
        
        fn add_value(&mut self, value: i64) {
            self.count += 1;
            self.sum += value;
            self.max = self.max.max(value);
            self.min = self.min.min(value);
        }
        
        fn average(&self) -> f64 {
            if self.count == 0 {
                0.0
            } else {
                self.sum as f64 / self.count as f64
            }
        }
    }
    
    let stats = Arc::new(Mutex::new(Stats::new()));
    let mut handles = vec![];
    
    // Spawn threads that add random-ish values
    for i in 0..5 {
        let stats = Arc::clone(&stats);
        let handle = thread::spawn(move || {
            for j in 0..3 {
                let value = (i * 10 + j) as i64;
                {
                    let mut s = stats.lock().unwrap();
                    s.add_value(value);
                    println!("  📊 Thread {} added value {}, count now: {}", i, value, s.count);
                }
                thread::sleep(Duration::from_millis(50));
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_stats = stats.lock().unwrap();
    println!("📈 Final Statistics:");
    println!("  Count: {}", final_stats.count);
    println!("  Sum: {}", final_stats.sum);
    println!("  Average: {:.2}", final_stats.average());
    println!("  Min: {}", final_stats.min);
    println!("  Max: {}", final_stats.max);
    
    println!();
}

fn demonstrate_rwlock() {
    println!("📚 RwLock - Multiple Readers, Single Writer:");
    println!();
    
    println!("RwLock allows:");
    println!("• Multiple simultaneous readers");
    println!("• OR one exclusive writer");
    println!("• Better performance when reads are frequent");
    println!();
    
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    
    // Spawn reader threads
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 10)); // Stagger access
            
            let reader = data.read().unwrap();
            println!("  👀 Reader {}: data = {:?}", i, *reader);
            println!("  👀 Reader {}: sum = {}", i, reader.iter().sum::<i32>());
            thread::sleep(Duration::from_millis(100)); // Hold read lock
            println!("  👀 Reader {} finished", i);
        });
        handles.push(handle);
    }
    
    // Spawn writer thread
    let data_writer = Arc::clone(&data);
    let writer_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50)); // Let readers start first
        
        println!("  ✏️  Writer waiting for exclusive access...");
        let mut writer = data_writer.write().unwrap();
        println!("  ✏️  Writer got exclusive lock!");
        
        writer.push(6);
        writer.push(7);
        println!("  ✏️  Writer added elements: {:?}", *writer);
        
        thread::sleep(Duration::from_millis(100)); // Hold write lock
        println!("  ✏️  Writer finished");
    });
    handles.push(writer_handle);
    
    // Another reader after writer
    let data_reader = Arc::clone(&data);
    let late_reader = thread::spawn(move || {
        thread::sleep(Duration::from_millis(200)); // Start after writer
        
        let reader = data_reader.read().unwrap();
        println!("  👀 Late reader: final data = {:?}", *reader);
    });
    handles.push(late_reader);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("✅ RwLock example completed");
    println!();
}

fn demonstrate_deadlock_prevention() {
    println!("💀 Deadlock Prevention:");
    println!();
    
    println!("Deadlock occurs when two or more threads wait for each other");
    println!("Common causes:");
    println!("• Circular waiting for locks");
    println!("• Acquiring locks in different orders");
    println!();
    
    println!("🛡️ Prevention strategies:");
    println!("1. Always acquire locks in the same order");
    println!("2. Use timeouts (try_lock_for)");
    println!("3. Avoid holding multiple locks when possible");
    println!("4. Use higher-level abstractions");
    println!();
    
    // Example of deadlock-prone code (commented out to avoid actual deadlock)
    println!("❌ Deadlock-prone pattern:");
    println!("// Thread A: lock1.lock(), then lock2.lock()");
    println!("// Thread B: lock2.lock(), then lock1.lock()");
    println!("// This can deadlock!");
    println!();
    
    // Safe pattern
    println!("✅ Safe pattern - consistent lock ordering:");
    
    let resource1 = Arc::new(Mutex::new("Resource 1".to_string()));
    let resource2 = Arc::new(Mutex::new("Resource 2".to_string()));
    
    let r1_clone = Arc::clone(&resource1);
    let r2_clone = Arc::clone(&resource2);
    
    let handle1 = thread::spawn(move || {
        // Always lock resource1 first, then resource2
        let _lock1 = r1_clone.lock().unwrap();
        println!("  🔒 Thread 1: got resource1 lock");
        thread::sleep(Duration::from_millis(50));
        
        let _lock2 = r2_clone.lock().unwrap();
        println!("  🔒 Thread 1: got resource2 lock");
        println!("  ✅ Thread 1: using both resources");
    });
    
    let r1_clone = Arc::clone(&resource1);
    let r2_clone = Arc::clone(&resource2);
    
    let handle2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10)); // Slight delay
        
        // Same order: resource1 first, then resource2
        let _lock1 = r1_clone.lock().unwrap();
        println!("  🔒 Thread 2: got resource1 lock");
        thread::sleep(Duration::from_millis(50));
        
        let _lock2 = r2_clone.lock().unwrap();
        println!("  🔒 Thread 2: got resource2 lock");
        println!("  ✅ Thread 2: using both resources");
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    println!("✅ No deadlock occurred!");
    println!();
}

fn demonstrate_practical_examples() {
    println!("🏗️ Practical Examples:");
    println!();
    
    // Example 1: Thread-safe cache
    println!("1. Thread-safe Cache:");
    thread_safe_cache_example();
    
    println!();
    
    // Example 2: Producer-consumer with shared buffer
    println!("2. Producer-Consumer with Shared Buffer:");
    producer_consumer_example();
}

fn thread_safe_cache_example() {
    use std::collections::HashMap;
    
    type Cache = Arc<RwLock<HashMap<String, String>>>;
    
    let cache: Cache = Arc::new(RwLock::new(HashMap::new()));
    
    // Function to get from cache or compute
    fn get_or_compute(cache: &Cache, key: &str) -> String {
        // Try to read from cache first
        {
            let reader = cache.read().unwrap();
            if let Some(value) = reader.get(key) {
                println!("  💾 Cache hit for key: {}", key);
                return value.clone();
            }
        }
        
        // Not in cache, compute the value
        println!("  🔄 Cache miss for key: {}, computing...", key);
        thread::sleep(Duration::from_millis(100)); // Simulate expensive computation
        let computed_value = format!("computed_value_for_{}", key);
        
        // Store in cache
        {
            let mut writer = cache.write().unwrap();
            writer.insert(key.to_string(), computed_value.clone());
            println!("  📝 Stored in cache: {} -> {}", key, computed_value);
        }
        
        computed_value
    }
    
    let mut handles = vec![];
    
    // Multiple threads accessing the cache
    for i in 0..5 {
        let cache = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let key = format!("key_{}", i % 3); // Some overlap in keys
            let value = get_or_compute(&cache, &key);
            println!("  🧵 Thread {}: {} = {}", i, key, value);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_cache = cache.read().unwrap();
    println!("  📊 Final cache size: {}", final_cache.len());
}

fn producer_consumer_example() {
    use std::collections::VecDeque;
    
    let buffer = Arc::new(Mutex::new(VecDeque::new()));
    let mut handles = vec![];
    
    // Producer thread
    let producer_buffer = Arc::clone(&buffer);
    let producer = thread::spawn(move || {
        for i in 0..10 {
            let item = format!("item_{}", i);
            {
                let mut buf = producer_buffer.lock().unwrap();
                buf.push_back(item.clone());
                println!("  📦 Produced: {}", item);
            }
            thread::sleep(Duration::from_millis(50));
        }
        println!("  ✅ Producer finished");
    });
    handles.push(producer);
    
    // Consumer threads
    for consumer_id in 0..2 {
        let consumer_buffer = Arc::clone(&buffer);
        let consumer = thread::spawn(move || {
            loop {
                let item = {
                    let mut buf = consumer_buffer.lock().unwrap();
                    buf.pop_front()
                };
                
                match item {
                    Some(item) => {
                        println!("  🍽️  Consumer {} consumed: {}", consumer_id, item);
                        thread::sleep(Duration::from_millis(100));
                    },
                    None => {
                        thread::sleep(Duration::from_millis(10));
                        // In a real implementation, you'd have a better way to signal completion
                        let buf_len = consumer_buffer.lock().unwrap().len();
                        if buf_len == 0 {
                            // Simple termination condition - in practice you'd use channels or other signaling
                            break;
                        }
                    }
                }
            }
            println!("  ✅ Consumer {} finished", consumer_id);
        });
        handles.push(consumer);
    }
    
    // Wait a bit then join all threads
    thread::sleep(Duration::from_millis(1000));
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!();
    println!("💡 Shared State Best Practices:");
    println!("• Use Arc<Mutex<T>> for shared mutable data");
    println!("• Use Arc<RwLock<T>> for read-heavy workloads");
    println!("• Keep critical sections small");
    println!("• Acquire locks in consistent order to avoid deadlocks");
    println!("• Consider using channels instead of shared state");
    println!("• Use atomic types for simple operations");
    println!("• Profile to ensure locking doesn't become a bottleneck");
}