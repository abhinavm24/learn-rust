//! Chapter 20.2: Multithreaded Web Server
//! 
//! This example demonstrates building a multithreaded HTTP server using a thread pool:
//! - Thread pool implementation with worker threads
//! - Channel-based job distribution system
//! - Concurrent request processing
//! - Resource management and performance improvements

use rust_book_examples::print_chapter_header;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    print_chapter_header("Chapter 20.2", "Multithreaded Web Server");
    
    println!("Starting multithreaded web server with thread pool...");
    println!("This server can handle multiple requests concurrently!");
    println!("Visit http://127.0.0.1:7879 to test the server");
    println!("Try opening multiple tabs to /sleep to see concurrent processing");
    println!("Press Ctrl+C to stop the server\n");
    
    // Create static HTML files
    create_html_files();
    
    // Bind to localhost on port 7879 (different from single-threaded version)
    let listener = TcpListener::bind("127.0.0.1:7879").unwrap();
    println!("🚀 Multithreaded server listening on http://127.0.0.1:7879");
    
    // Create a thread pool with 4 worker threads
    let pool = ThreadPool::new(4);
    println!("📋 Thread pool created with 4 workers\n");
    
    // Handle connections using the thread pool
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        // Submit work to the thread pool instead of handling directly
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    
    println!("Shutting down server...");
}

/// Thread pool implementation for concurrent request handling
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

/// Individual worker thread
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

/// Type alias for jobs (closures) sent to workers
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool with the specified number of threads
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        // Create a channel for job distribution
        let (sender, receiver) = mpsc::channel();
        
        // Wrap receiver in Arc<Mutex> so multiple workers can share it
        let receiver = Arc::new(Mutex::new(receiver));
        
        // Pre-allocate worker vector
        let mut workers = Vec::with_capacity(size);
        
        // Create worker threads
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool { workers, sender }
    }
    
    /// Execute a closure on one of the worker threads
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        
        match self.sender.send(job) {
            Ok(_) => {
                // Job successfully queued
            }
            Err(_) => {
                eprintln!("Failed to send job to thread pool");
            }
        }
    }
}

impl Worker {
    /// Create a new worker thread that listens for jobs
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            println!("🔧 Worker {} started", id);
            
            loop {
                // Lock the receiver and wait for a job
                let job = receiver
                    .lock()
                    .unwrap()
                    .recv()
                    .unwrap();
                
                println!("👷 Worker {} got a job; executing.", id);
                
                // Execute the job
                job();
                
                println!("✅ Worker {} finished job.", id);
            }
        });
        
        Worker { id, thread }
    }
}

/// Handles an individual HTTP connection (same as single-threaded version)
fn handle_connection(mut stream: TcpStream) {
    let thread_id = thread::current().id();
    println!("\n--- New Connection (Thread: {:?}) ---", thread_id);
    
    // Read the request
    let mut buffer = [0; 1024];
    
    match stream.read(&mut buffer) {
        Ok(0) => {
            println!("Client disconnected immediately");
            return;
        }
        Ok(bytes_read) => {
            println!("Read {} bytes from client", bytes_read);
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
            return;
        }
    }
    
    // Parse the request
    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");
    
    println!("📨 Request: {} (Thread: {:?})", request_line, thread_id);
    
    // Route the request
    let (status_line, filename) = route_request(request_line);
    
    // Read file content
    let contents = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(_) => {
            println!("⚠️  File '{}' not found, using fallback", filename);
            create_fallback_content(filename)
        }
    };
    
    // Build and send response
    let response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    
    match stream.write_all(response.as_bytes()) {
        Ok(_) => {
            stream.flush().unwrap();
            println!("✅ Response sent (Thread: {:?})", thread_id);
        }
        Err(e) => {
            eprintln!("❌ Failed to send response: {}", e);
        }
    }
}

/// Routes HTTP requests to appropriate handlers
fn route_request(request_line: &str) -> (&'static str, &'static str) {
    match request_line {
        "GET / HTTP/1.1" => {
            println!("🏠 Serving home page");
            ("HTTP/1.1 200 OK", "hello.html")
        }
        "GET /hello HTTP/1.1" => {
            println!("👋 Serving hello page");
            ("HTTP/1.1 200 OK", "hello.html")
        }
        "GET /sleep HTTP/1.1" => {
            println!("😴 Starting slow request (5 second delay)...");
            println!("✨ NOTICE: Other requests can now be processed concurrently!");
            
            // Simulate slow processing
            thread::sleep(Duration::from_secs(5));
            
            println!("⏰ Slow request completed");
            ("HTTP/1.1 200 OK", "multithreaded.html")
        }
        "GET /about HTTP/1.1" => {
            println!("ℹ️  Serving about page");
            ("HTTP/1.1 200 OK", "about.html")
        }
        "GET /test HTTP/1.1" => {
            println!("🧪 Serving test page");
            ("HTTP/1.1 200 OK", "test.html")
        }
        "GET /concurrent HTTP/1.1" => {
            println!("🔄 Serving concurrent test page");
            ("HTTP/1.1 200 OK", "concurrent.html")
        }
        _ => {
            println!("❌ Unknown route: {}", request_line);
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        }
    }
}

/// Creates static HTML files for the server
fn create_html_files() {
    let hello_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Multithreaded Rust Web Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #f0f0f0; }
        .container { background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #d73502; }
        .nav { margin: 20px 0; }
        .nav a { margin-right: 15px; color: #d73502; text-decoration: none; padding: 5px 10px; border-radius: 3px; }
        .nav a:hover { background: #d73502; color: white; }
        .feature { background: #e8f5e8; padding: 15px; border-left: 4px solid #4CAF50; margin: 20px 0; }
        .improvement { background: #fff3cd; padding: 15px; border-left: 4px solid #ffc107; margin: 20px 0; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🦀⚡ Multithreaded Rust Web Server</h1>
        <p>Welcome to our improved HTTP server with concurrent request processing!</p>
        
        <div class="nav">
            <a href="/">Home</a>
            <a href="/about">About</a>
            <a href="/test">Test</a>
            <a href="/concurrent">Concurrent Test</a>
            <a href="/sleep">Sleep Test</a>
        </div>
        
        <div class="feature">
            <strong>✨ New!</strong> This server now uses a thread pool to handle multiple requests simultaneously!
        </div>
        
        <div class="improvement">
            <strong>Performance Boost:</strong> Try opening multiple tabs to <a href="/sleep">/sleep</a> - 
            they'll process concurrently instead of blocking each other!
        </div>
        
        <h2>Server Improvements</h2>
        <ul>
            <li>✅ Thread pool with 4 worker threads</li>
            <li>✅ Concurrent request processing</li>
            <li>✅ Channel-based job distribution</li>
            <li>✅ No more blocking on slow requests</li>
            <li>✅ Better resource utilization</li>
        </ul>
        
        <h2>Architecture</h2>
        <ul>
            <li><strong>ThreadPool:</strong> Manages worker threads and job queue</li>
            <li><strong>Workers:</strong> Individual threads that process requests</li>
            <li><strong>Jobs:</strong> Closures containing request handling logic</li>
            <li><strong>Channel:</strong> MPSC channel for job distribution</li>
        </ul>
        
        <p><small>Built with ❤️ in Rust | Chapter 20.2 | Multithreaded Edition</small></p>
    </div>
</body>
</html>"#;

    let multithreaded_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Slow Request Completed - Multithreaded Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #f0f0f0; }
        .container { background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #d73502; }
        .nav { margin: 20px 0; }
        .nav a { margin-right: 15px; color: #d73502; text-decoration: none; padding: 5px 10px; border-radius: 3px; }
        .nav a:hover { background: #d73502; color: white; }
        .success { background: #d4edda; padding: 20px; border-radius: 5px; border-left: 4px solid #28a745; }
        .highlight { background: #e8f4fd; padding: 15px; border-radius: 5px; margin: 15px 0; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🎉 Slow Request Completed!</h1>
        
        <div class="nav">
            <a href="/">Home</a>
            <a href="/about">About</a>
            <a href="/test">Test</a>
            <a href="/concurrent">Concurrent Test</a>
            <a href="/sleep">Sleep Test</a>
        </div>
        
        <div class="success">
            <h3>✅ Request processed successfully!</h3>
            <p>This request took 5 seconds to complete, but thanks to our multithreaded architecture, 
            other requests could be processed concurrently!</p>
        </div>
        
        <div class="highlight">
            <h3>🧵 Thread Pool Magic</h3>
            <p>While this request was sleeping for 5 seconds, the other worker threads in our pool 
            were free to handle additional incoming requests. This is the power of concurrent programming!</p>
        </div>
        
        <h2>What Just Happened?</h2>
        <ol>
            <li>Your request was received by the main thread</li>
            <li>The request was packaged as a "job" (closure)</li>
            <li>The job was sent through a channel to the thread pool</li>
            <li>An available worker thread picked up the job</li>
            <li>The worker processed the request (including the 5-second sleep)</li>
            <li>Meanwhile, other workers were free to handle other requests</li>
        </ol>
        
        <p><a href="/concurrent">🔄 Try the concurrent test page</a> to see this in action!</p>
        
        <p><small>Processed by multithreaded Rust server | Worker thread completed successfully</small></p>
    </div>
</body>
</html>"#;

    let concurrent_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Concurrency Test - Multithreaded Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #f0f0f0; }
        .container { background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #d73502; }
        .nav { margin: 20px 0; }
        .nav a { margin-right: 15px; color: #d73502; text-decoration: none; padding: 5px 10px; border-radius: 3px; }
        .nav a:hover { background: #d73502; color: white; }
        .test-section { background: #f8f9fa; padding: 20px; margin: 15px 0; border-radius: 5px; border: 1px solid #dee2e6; }
        button { background: #d73502; color: white; border: none; padding: 12px 20px; border-radius: 5px; cursor: pointer; margin: 5px; }
        button:hover { background: #b52d02; }
        .instruction { background: #e8f4fd; padding: 15px; border-left: 4px solid #2196F3; margin: 20px 0; }
        .timer { font-family: monospace; font-size: 1.2em; font-weight: bold; color: #d73502; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🔄 Concurrency Test Page</h1>
        
        <div class="nav">
            <a href="/">Home</a>
            <a href="/about">About</a>
            <a href="/test">Test</a>
            <a href="/concurrent">Concurrent Test</a>
            <a href="/sleep">Sleep Test</a>
        </div>
        
        <div class="instruction">
            <strong>🧪 How to test concurrency:</strong>
            <ol>
                <li>Click "Open Sleep Test" multiple times quickly</li>
                <li>Watch the timestamps - they should start at nearly the same time</li>
                <li>All requests will complete around the same time (after ~5 seconds)</li>
                <li>Check your terminal to see different worker threads handling requests</li>
            </ol>
        </div>
        
        <div class="test-section">
            <h3>Multiple Sleep Requests</h3>
            <p>Each click opens a new tab with a 5-second delay. With our thread pool, 
            up to 4 requests can process simultaneously!</p>
            
            <button onclick="openSleepTest()">Open Sleep Test</button>
            <button onclick="openMultiple()">Open 3 Sleep Tests</button>
            <button onclick="openManyTests()">Open 6 Sleep Tests</button>
            
            <p class="timer" id="timer">Page loaded at: <span id="timestamp"></span></p>
        </div>
        
        <div class="test-section">
            <h3>Expected Behavior</h3>
            <ul>
                <li><strong>First 4 requests:</strong> Process immediately on available workers</li>
                <li><strong>Additional requests:</strong> Queue until a worker becomes free</li>
                <li><strong>Terminal output:</strong> Shows which worker handles each request</li>
                <li><strong>Response times:</strong> Multiple requests complete around the same time</li>
            </ul>
        </div>
        
        <div class="test-section">
            <h3>Compare with Single-Threaded</h3>
            <p>In the single-threaded version (port 7878), requests would process one at a time.
            Here, our 4-worker thread pool can handle multiple requests concurrently!</p>
            
            <p><strong>Thread Pool Benefits:</strong></p>
            <ul>
                <li>Better resource utilization</li>
                <li>Improved response times under load</li>
                <li>No blocking on slow requests</li>
                <li>Configurable number of worker threads</li>
            </ul>
        </div>
        
        <script>
            function updateTimestamp() {
                document.getElementById('timestamp').textContent = new Date().toLocaleTimeString();
            }
            
            function openSleepTest() {
                const start = Date.now();
                console.log(`Opening sleep test at: ${new Date().toISOString()}`);
                window.open('/sleep', '_blank');
            }
            
            function openMultiple() {
                console.log(`Opening 3 sleep tests at: ${new Date().toISOString()}`);
                for(let i = 0; i < 3; i++) {
                    setTimeout(() => window.open('/sleep', '_blank'), i * 100);
                }
            }
            
            function openManyTests() {
                console.log(`Opening 6 sleep tests at: ${new Date().toISOString()}`);
                for(let i = 0; i < 6; i++) {
                    setTimeout(() => window.open('/sleep', '_blank'), i * 150);
                }
            }
            
            updateTimestamp();
            setInterval(updateTimestamp, 1000);
            
            console.log('🦀 Multithreaded server concurrency test page loaded!');
            console.log('Thread pool: 4 workers available');
        </script>
    </div>
</body>
</html>"#;

    let about_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>About - Multithreaded Rust Web Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #f0f0f0; }
        .container { background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #d73502; }
        .nav { margin: 20px 0; }
        .nav a { margin-right: 15px; color: #d73502; text-decoration: none; padding: 5px 10px; border-radius: 3px; }
        .nav a:hover { background: #d73502; color: white; }
        code { background: #f5f5f5; padding: 2px 6px; border-radius: 3px; }
        .architecture { background: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }
    </style>
</head>
<body>
    <div class="container">
        <h1>About This Multithreaded Server</h1>
        
        <div class="nav">
            <a href="/">Home</a>
            <a href="/about">About</a>
            <a href="/test">Test</a>
            <a href="/concurrent">Concurrent Test</a>
            <a href="/sleep">Sleep Test</a>
        </div>
        
        <p>This is the improved version of our Rust web server from Chapter 20.2, 
        featuring a custom thread pool for concurrent request processing.</p>
        
        <h2>Key Improvements</h2>
        <ul>
            <li><strong>Thread Pool:</strong> 4 pre-created worker threads</li>
            <li><strong>Concurrent Processing:</strong> Multiple requests handled simultaneously</li>
            <li><strong>Channel Communication:</strong> MPSC channel for job distribution</li>
            <li><strong>Resource Management:</strong> Fixed thread count prevents resource exhaustion</li>
        </ul>
        
        <div class="architecture">
            <h3>🏗️ Architecture Overview</h3>
            <pre><code>
Main Thread
    │
    ├─ TcpListener (accepts connections)
    │
    └─ ThreadPool
        ├─ MPSC Channel (job queue)
        │
        ├─ Worker 0 ┐
        ├─ Worker 1 ├─ Process jobs concurrently
        ├─ Worker 2 │
        └─ Worker 3 ┘
            </code></pre>
        </div>
        
        <h2>Technical Implementation</h2>
        <ul>
            <li><strong>ThreadPool:</strong> Manages workers and job distribution</li>
            <li><strong>Worker:</strong> Individual threads that process HTTP requests</li>
            <li><strong>Job:</strong> <code>Box&lt;dyn FnOnce() + Send + 'static&gt;</code></li>
            <li><strong>Synchronization:</strong> <code>Arc&lt;Mutex&lt;Receiver&gt;&gt;</code> for shared access</li>
        </ul>
        
        <h2>Performance Benefits</h2>
        <ul>
            <li>✅ No blocking on slow requests</li>
            <li>✅ Better CPU utilization</li>
            <li>✅ Improved throughput under load</li>
            <li>✅ Predictable resource usage</li>
        </ul>
        
        <h2>Testing Concurrency</h2>
        <p>Visit the <a href="/concurrent">Concurrent Test</a> page to see the thread pool in action.
        Open multiple sleep requests and watch them process simultaneously!</p>
        
        <h2>Next Steps</h2>
        <p>Chapter 20.3 will add graceful shutdown and cleanup capabilities to properly
        manage thread lifecycle and resource deallocation.</p>
        
        <p><small>Multithreaded Rust Web Server | Chapter 20.2 | Port 7879</small></p>
    </div>
</body>
</html>"#;

    // Create test and 404 pages (simplified versions)
    let test_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Test - Multithreaded Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #f0f0f0; }
        .container { background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #d73502; }
        .nav a { margin-right: 15px; color: #d73502; text-decoration: none; padding: 5px 10px; }
        .nav a:hover { background: #d73502; color: white; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🧪 Multithreaded Server Test</h1>
        <div class="nav">
            <a href="/">Home</a>
            <a href="/about">About</a>
            <a href="/test">Test</a>
            <a href="/concurrent">Concurrent Test</a>
            <a href="/sleep">Sleep Test</a>
        </div>
        <p>This page confirms the multithreaded server is working correctly.</p>
        <p>Check the <a href="/concurrent">Concurrent Test</a> page for thread pool testing.</p>
    </div>
</body>
</html>"#;

    let error_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>404 Not Found - Multithreaded Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #f0f0f0; text-align: center; }
        .container { background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #d73502; font-size: 4em; margin: 0; }
    </style>
</head>
<body>
    <div class="container">
        <h1>404</h1>
        <h2>Page Not Found</h2>
        <p>The multithreaded server couldn't find this page.</p>
        <p><a href="/">🏠 Return Home</a></p>
    </div>
</body>
</html>"#;

    // Write files
    write_file_if_not_exists("hello.html", hello_html);
    write_file_if_not_exists("multithreaded.html", multithreaded_html);
    write_file_if_not_exists("concurrent.html", concurrent_html);
    write_file_if_not_exists("about.html", about_html);
    write_file_if_not_exists("test.html", test_html);
    write_file_if_not_exists("404.html", error_html);
}

/// Helper function to write file only if it doesn't exist
fn write_file_if_not_exists(filename: &str, content: &str) {
    if !std::path::Path::new(filename).exists() {
        match fs::write(filename, content) {
            Ok(_) => println!("📄 Created {}", filename),
            Err(e) => eprintln!("❌ Failed to create {}: {}", filename, e),
        }
    }
}

/// Creates fallback content when files are missing
fn create_fallback_content(filename: &str) -> String {
    format!(r#"<html><body>
        <h1>Multithreaded Server</h1>
        <p>File '{}' not found - served by worker thread</p>
        <p><a href="/">Return Home</a></p>
    </body></html>"#, filename)
}