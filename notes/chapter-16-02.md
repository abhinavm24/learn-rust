# Chapter 16.2: Using Message Passing to Transfer Data Between Threads

## Key Takeaways

### Message Passing Fundamentals
- **Communication**: Send data between threads safely
- **Channels**: MPSC (Multiple Producer, Single Consumer) channels
- **Ownership Transfer**: Data moved between threads via channels
- **Synchronization**: Coordinate thread execution through messages

### Basic Channel Usage
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

### Multiple Messages
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

### Multiple Producers
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

### Channel Types
```rust
use std::sync::mpsc;

fn main() {
    // Unbounded channel
    let (tx, rx) = mpsc::channel();
    
    // Bounded channel (sync_channel)
    let (tx_sync, rx_sync) = mpsc::sync_channel(0);  // Synchronous
    let (tx_bound, rx_bound) = mpsc::sync_channel(5);  // Bounded buffer
}
```

### Error Handling with Channels
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("Hello").unwrap();
        // tx is dropped here
    });

    // First receive works
    match rx.recv() {
        Ok(msg) => println!("Received: {}", msg),
        Err(e) => println!("Receive error: {}", e),
    }

    // Second receive fails because sender is dropped
    match rx.recv() {
        Ok(msg) => println!("Received: {}", msg),
        Err(e) => println!("Receive error: {}", e),
    }
}
```

### Non-blocking Operations
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        tx.send("delayed message").unwrap();
    });

    loop {
        match rx.try_recv() {
            Ok(msg) => {
                println!("Received: {}", msg);
                break;
            }
            Err(mpsc::TryRecvError::Empty) => {
                println!("No message yet, doing other work...");
                thread::sleep(Duration::from_millis(500));
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("Sender disconnected");
                break;
            }
        }
    }
}
```

### Worker Pool Pattern
```rust
use std::sync::mpsc;
use std::thread;

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = std::sync::Arc::new(std::sync::Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, std::sync::Arc::clone(&receiver)));
        }
        
        ThreadPool { workers, sender }
    }
    
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Worker {
    fn new(id: usize, receiver: std::sync::Arc<std::sync::Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });
        
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

Official Chapter: https://doc.rust-lang.org/book/ch16-02-message-passing.html

---
*Completed: ✓*