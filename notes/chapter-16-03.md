# Chapter 16.3: Shared-State Concurrency

## Key Takeaways

### Shared State Fundamentals
- **Mutual Exclusion**: Mutex ensures only one thread accesses data
- **Atomic Reference Counting**: Arc enables sharing between threads
- **Deadlock Prevention**: Careful lock ordering prevents deadlocks
- **Thread Safety**: Rust's type system enforces safe sharing

### Mutex<T> Basic Usage
```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```

### Sharing Mutex Between Threads
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

### RwLock for Multiple Readers
```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];

    // Multiple readers
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let reader = data.read().unwrap();
            println!("Reader {}: {:?}", i, *reader);
        });
        handles.push(handle);
    }

    // Single writer
    let data_clone = Arc::clone(&data);
    let writer_handle = thread::spawn(move || {
        let mut writer = data_clone.write().unwrap();
        writer.push(4);
        println!("Writer added 4");
    });
    handles.push(writer_handle);

    for handle in handles {
        handle.join().unwrap();
    }
}
```

### Deadlock Example and Prevention
```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mutex1 = Arc::new(Mutex::new(0));
    let mutex2 = Arc::new(Mutex::new(0));

    let mutex1_clone = Arc::clone(&mutex1);
    let mutex2_clone = Arc::clone(&mutex2);

    // Thread 1: locks mutex1 then mutex2
    let handle1 = thread::spawn(move || {
        let _lock1 = mutex1_clone.lock().unwrap();
        println!("Thread 1: locked mutex1");
        
        thread::sleep(Duration::from_millis(100));
        
        let _lock2 = mutex2_clone.lock().unwrap();
        println!("Thread 1: locked mutex2");
    });

    // Thread 2: locks mutex2 then mutex1 (potential deadlock)
    let handle2 = thread::spawn(move || {
        let _lock2 = mutex2.lock().unwrap();
        println!("Thread 2: locked mutex2");
        
        thread::sleep(Duration::from_millis(100));
        
        let _lock1 = mutex1.lock().unwrap();
        println!("Thread 2: locked mutex1");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

// Prevention: Always acquire locks in same order
fn deadlock_free_version() {
    let mutex1 = Arc::new(Mutex::new(0));
    let mutex2 = Arc::new(Mutex::new(0));

    let mutex1_clone = Arc::clone(&mutex1);
    let mutex2_clone = Arc::clone(&mutex2);

    let handle1 = thread::spawn(move || {
        let _lock1 = mutex1_clone.lock().unwrap();
        let _lock2 = mutex2_clone.lock().unwrap();
        println!("Thread 1: acquired both locks");
    });

    let handle2 = thread::spawn(move || {
        let _lock1 = mutex1.lock().unwrap();  // Same order
        let _lock2 = mutex2.lock().unwrap();
        println!("Thread 2: acquired both locks");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
```

### Atomic Operations
```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", counter.load(Ordering::SeqCst));
}
```

### Producer-Consumer with Shared State
```rust
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

struct Buffer<T> {
    queue: Mutex<VecDeque<T>>,
    not_empty: Condvar,
    not_full: Condvar,
    capacity: usize,
}

impl<T> Buffer<T> {
    fn new(capacity: usize) -> Self {
        Buffer {
            queue: Mutex::new(VecDeque::new()),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
            capacity,
        }
    }

    fn push(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        
        // Wait while queue is full
        while queue.len() == self.capacity {
            queue = self.not_full.wait(queue).unwrap();
        }
        
        queue.push_back(item);
        self.not_empty.notify_one();
    }

    fn pop(&self) -> T {
        let mut queue = self.queue.lock().unwrap();
        
        // Wait while queue is empty
        while queue.is_empty() {
            queue = self.not_empty.wait(queue).unwrap();
        }
        
        let item = queue.pop_front().unwrap();
        self.not_full.notify_one();
        item
    }
}

fn main() {
    let buffer = Arc::new(Buffer::new(5));
    
    // Producer
    let buffer_clone = Arc::clone(&buffer);
    let producer = thread::spawn(move || {
        for i in 0..10 {
            buffer_clone.push(i);
            println!("Produced: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Consumer
    let consumer = thread::spawn(move || {
        for _ in 0..10 {
            let item = buffer.pop();
            println!("Consumed: {}", item);
            thread::sleep(Duration::from_millis(150));
        }
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}
```

Official Chapter: https://doc.rust-lang.org/book/ch16-03-shared-state.html

---
*Completed: ✓*