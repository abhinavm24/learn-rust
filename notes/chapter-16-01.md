# Chapter 16.1: Using Threads to Run Code Simultaneously

## Key Takeaways

### Thread Fundamentals
- **Concurrent Execution**: Multiple threads run simultaneously
- **OS Threads**: Rust uses operating system threads (1:1 model)
- **Thread Safety**: Rust's ownership system prevents data races
- **Performance**: Utilize multiple CPU cores effectively

### Creating Threads
```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

### Waiting for Threads with join
```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();  // Wait for thread to finish
}
```

### Moving Data into Threads
```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

### Thread Return Values
```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        // Some computation
        42
    });

    let result = handle.join().unwrap();
    println!("Thread returned: {}", result);
}
```

### Multiple Threads Example
```rust
use std::thread;

fn main() {
    let mut handles = vec![];

    for i in 0..10 {
        let handle = thread::spawn(move || {
            println!("Thread {} starting", i);
            // Simulate some work
            thread::sleep(std::time::Duration::from_millis(100));
            println!("Thread {} finished", i);
            i * 2  // Return value
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().unwrap();
        println!("Thread returned: {}", result);
    }
}
```

### Thread Builder
```rust
use std::thread;

fn main() {
    let builder = thread::Builder::new()
        .name("worker".into())
        .stack_size(32 * 1024);

    let handler = builder.spawn(|| {
        println!("Hello from custom thread!");
    }).unwrap();

    handler.join().unwrap();
}
```

Official Chapter: https://doc.rust-lang.org/book/ch16-01-threads.html

---
*Completed: ✓*