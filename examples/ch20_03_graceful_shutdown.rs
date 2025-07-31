//! Chapter 20.3: Graceful Shutdown and Cleanup
//! 
//! This example demonstrates implementing graceful shutdown for the multithreaded web server:
//! - Drop trait implementation for resource cleanup
//! - Message-based worker termination
//! - Proper thread joining and resource deallocation
//! - Graceful handling of server shutdown

use rust_book_examples::print_chapter_header;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    print_chapter_header("Chapter 20.3", "Graceful Shutdown and Cleanup");
    
    println!("Starting web server with graceful shutdown...");
    println!("This server will cleanly shut down after processing a few requests");
    println!("Visit http://127.0.0.1:7880 to test the server");
    println!("The server will automatically shut down after handling 5 requests\n");
    
    // Create static HTML files
    create_html_files();
    
    // Bind to localhost on port 7880 (different from other versions)
    let listener = TcpListener::bind("127.0.0.1:7880").unwrap();
    println!("🚀 Server with graceful shutdown listening on http://127.0.0.1:7880");
    
    // Create a thread pool with 4 worker threads
    let pool = ThreadPool::new(4);
    println!("📋 Thread pool created with 4 workers\n");
    
    // Limit to 5 requests for demonstration, then shut down gracefully
    for (i, stream) in listener.incoming().enumerate() {
        if i >= 5 {
            println!("\n🛑 Reached request limit (5). Initiating graceful shutdown...");
            break;
        }
        
        let stream = stream.unwrap();
        
        println!("📝 Queuing request {} of 5", i + 1);
        
        // Submit work to the thread pool
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    
    println!("\n🔄 Graceful shutdown initiated...");
    println!("ThreadPool Drop trait will handle cleanup automatically");
    
    // ThreadPool will be dropped here, triggering graceful shutdown
    // The Drop implementation will:
    // 1. Send terminate messages to all workers
    // 2. Join all worker threads
    // 3. Clean up resources
    
    println!("✅ Server shutdown complete!");
}

/// Enhanced ThreadPool with graceful shutdown capabilities
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

/// Worker thread that can be gracefully terminated
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

/// Messages that can be sent to workers
enum Message {
    NewJob(Job),
    Terminate,
}

/// Type alias for jobs (closures) sent to workers
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool with graceful shutdown support
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        // Create a channel for message distribution
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
        
        // Send the job wrapped in a NewJob message
        match self.sender.send(Message::NewJob(job)) {
            Ok(_) => {
                // Job successfully queued
            }
            Err(_) => {
                eprintln!("❌ Failed to send job to thread pool - pool may be shutting down");
            }
        }
    }
}

/// Implement Drop for graceful shutdown
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("\n🔄 ThreadPool::drop() called - beginning graceful shutdown");
        
        // Step 1: Send terminate message to all workers
        println!("📤 Sending terminate message to all workers...");
        
        for _ in &self.workers {
            if let Err(_) = self.sender.send(Message::Terminate) {
                eprintln!("⚠️  Failed to send terminate message to a worker");
            }
        }
        
        // Step 2: Join all worker threads
        println!("⏳ Waiting for all workers to finish...");
        
        for worker in &mut self.workers {
            println!("🔄 Shutting down worker {}", worker.id);
            
            // Take the thread handle (Option::take() leaves None)
            if let Some(thread) = worker.thread.take() {
                match thread.join() {
                    Ok(_) => {
                        println!("✅ Worker {} shut down successfully", worker.id);
                    }
                    Err(_) => {
                        eprintln!("❌ Worker {} panicked during shutdown", worker.id);
                    }
                }
            }
        }
        
        println!("🎉 All workers have been shut down gracefully!");
    }
}

impl Worker {
    /// Create a new worker thread that can handle termination messages
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            println!("🔧 Worker {} started and ready for messages", id);
            
            loop {
                // Lock the receiver and wait for a message
                let message = match receiver.lock() {
                    Ok(guard) => {
                        match guard.recv() {
                            Ok(msg) => msg,
                            Err(_) => {
                                println!("🔌 Worker {} detected channel disconnect, exiting", id);
                                break;
                            }
                        }
                    }
                    Err(_) => {
                        eprintln!("❌ Worker {} failed to acquire lock, exiting", id);
                        break;
                    }
                };
                
                // Handle the message
                match message {
                    Message::NewJob(job) => {
                        println!("👷 Worker {} got a job; executing.", id);
                        
                        // Execute the job
                        job();
                        
                        println!("✅ Worker {} finished job.", id);
                    }
                    Message::Terminate => {
                        println!("🛑 Worker {} received terminate signal, shutting down.", id);
                        break;
                    }
                }
            }
            
            println!("👋 Worker {} exiting gracefully", id);
        });
        
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

/// Handles an individual HTTP connection
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
        "{}\r\nContent-Length: {}\r\nContent-Type: text/html\r\nX-Served-By: Worker-{:?}\r\n\r\n{}",
        status_line,
        contents.len(),
        thread_id,
        contents
    );
    
    match stream.write_all(response.as_bytes()) {
        Ok(_) => {
            stream.flush().unwrap();
            println!("✅ Response sent successfully (Thread: {:?})", thread_id);
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
            ("HTTP/1.1 200 OK", "web_assets/ch20_web_server/graceful.html")
        }
        "GET /hello HTTP/1.1" => {
            println!("👋 Serving hello page");
            ("HTTP/1.1 200 OK", "web_assets/ch20_web_server/graceful.html")
        }
        "GET /sleep HTTP/1.1" => {
            println!("😴 Starting slow request (3 second delay for demo)...");
            
            // Shorter delay for demo purposes
            thread::sleep(Duration::from_secs(3));
            
            println!("⏰ Slow request completed");
            ("HTTP/1.1 200 OK", "web_assets/ch20_web_server/graceful.html")
        }
        "GET /shutdown HTTP/1.1" => {
            println!("🛑 Serving shutdown info page");
            ("HTTP/1.1 200 OK", "web_assets/ch20_web_server/shutdown.html")
        }
        "GET /about HTTP/1.1" => {
            println!("ℹ️  Serving about page");
            ("HTTP/1.1 200 OK", "web_assets/ch20_web_server/about.html")
        }
        _ => {
            println!("❌ Unknown route: {}", request_line);
            ("HTTP/1.1 404 NOT FOUND", "web_assets/ch20_web_server/404.html")
        }
    }
}

/// Creates static HTML files for the server
fn create_html_files() {
    let graceful_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Graceful Shutdown - Rust Web Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #f0f0f0; }
        .container { background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #d73502; }
        .nav { margin: 20px 0; }
        .nav a { margin-right: 15px; color: #d73502; text-decoration: none; padding: 5px 10px; border-radius: 3px; }
        .nav a:hover { background: #d73502; color: white; }
        .feature { background: #e8f5e8; padding: 15px; border-left: 4px solid #4CAF50; margin: 20px 0; }
        .warning { background: #fff3cd; padding: 15px; border-left: 4px solid #ffc107; margin: 20px 0; }
        code { background: #f5f5f5; padding: 2px 6px; border-radius: 3px; font-family: monospace; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🦀🛡️ Graceful Shutdown Web Server</h1>
        <p>Welcome to our Rust web server with graceful shutdown capabilities!</p>
        
        <div class="nav">
            <a href="/">Home</a>
            <a href="/about">About</a>
            <a href="/shutdown">Shutdown Info</a>
            <a href="/sleep">Sleep Test</a>
        </div>
        
        <div class="feature">
            <strong>✨ New Feature:</strong> This server implements graceful shutdown using Rust's Drop trait!
        </div>
        
        <div class="warning">
            <strong>⚠️ Demo Behavior:</strong> This server will automatically shut down after handling 5 requests
            to demonstrate the graceful shutdown process.
        </div>
        
        <h2>Graceful Shutdown Features</h2>
        <ul>
            <li>✅ <code>Drop</code> trait implementation for cleanup</li>
            <li>✅ Message-based worker termination</li>
            <li>✅ Proper thread joining</li>
            <li>✅ Resource deallocation</li>
            <li>✅ No abrupt termination</li>
        </ul>
        
        <h2>Shutdown Process</h2>
        <ol>
            <li><strong>Stop accepting new requests</strong> - Main loop exits</li>
            <li><strong>Signal workers</strong> - Send <code>Message::Terminate</code> to all workers</li>
            <li><strong>Wait for completion</strong> - Join all worker threads</li>
            <li><strong>Clean up resources</strong> - Drop trait handles cleanup</li>
        </ol>
        
        <h2>Technical Implementation</h2>
        <ul>
            <li><strong>Message Enum:</strong> <code>NewJob(Job)</code> and <code>Terminate</code></li>
            <li><strong>Optional JoinHandle:</strong> <code>Option&lt;thread::JoinHandle&lt;()&gt;&gt;</code></li>
            <li><strong>RAII Pattern:</strong> Resource cleanup in Drop trait</li>
            <li><strong>Channel Communication:</strong> Coordinated shutdown</li>
        </ul>
        
        <p>Check your terminal to see the detailed shutdown process when the server stops!</p>
        
        <p><small>Built with ❤️ in Rust | Chapter 20.3 | Graceful Shutdown Edition</small></p>
    </div>
</body>
</html>"#;

    let shutdown_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Shutdown Information - Graceful Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #f0f0f0; }
        .container { background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #d73502; }
        .nav { margin: 20px 0; }
        .nav a { margin-right: 15px; color: #d73502; text-decoration: none; padding: 5px 10px; border-radius: 3px; }
        .nav a:hover { background: #d73502; color: white; }
        .code-block { background: #f8f9fa; padding: 15px; border-radius: 5px; margin: 15px 0; border-left: 4px solid #6c757d; }
        .step { background: #e8f4fd; padding: 10px; margin: 10px 0; border-radius: 5px; }
        code { background: #f5f5f5; padding: 2px 6px; border-radius: 3px; font-family: monospace; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🛑 Graceful Shutdown Information</h1>
        
        <div class="nav">
            <a href="/">Home</a>
            <a href="/about">About</a>
            <a href="/shutdown">Shutdown Info</a>
            <a href="/sleep">Sleep Test</a>
        </div>
        
        <h2>How Graceful Shutdown Works</h2>
        
        <div class="step">
            <strong>Step 1: Stop Accepting Requests</strong><br>
            The main server loop exits, preventing new connections from being accepted.
        </div>
        
        <div class="step">
            <strong>Step 2: Signal All Workers</strong><br>
            Send <code>Message::Terminate</code> to each worker thread through the MPSC channel.
        </div>
        
        <div class="step">
            <strong>Step 3: Wait for Worker Completion</strong><br>
            Use <code>thread.join()</code> to wait for each worker to finish its current job and exit.
        </div>
        
        <div class="step">
            <strong>Step 4: Resource Cleanup</strong><br>
            All channels, threads, and resources are properly deallocated.
        </div>
        
        <h2>Drop Trait Implementation</h2>
        <div class="code-block">
            <code>
            impl Drop for ThreadPool {<br>
            &nbsp;&nbsp;&nbsp;&nbsp;fn drop(&mut self) {<br>
            &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;// Send terminate messages<br>
            &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;for _ in &self.workers {<br>
            &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;self.sender.send(Message::Terminate).unwrap();<br>
            &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;}<br>
            &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<br>
            &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;// Join all threads<br>
            &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;for worker in &mut self.workers {<br>
            &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;if let Some(thread) = worker.thread.take() {<br>
            &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;thread.join().unwrap();<br>
            &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;}<br>
            &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;}<br>
            &nbsp;&nbsp;&nbsp;&nbsp;}<br>
            }
            </code>
        </div>
        
        <h2>Benefits of Graceful Shutdown</h2>
        <ul>
            <li><strong>Data Integrity:</strong> Current requests complete before shutdown</li>
            <li><strong>Resource Cleanup:</strong> No leaked threads or memory</li>
            <li><strong>Predictable Behavior:</strong> Clean termination process</li>
            <li><strong>Production Ready:</strong> Proper handling of server lifecycle</li>
        </ul>
        
        <h2>Real-World Applications</h2>
        <ul>
            <li>Production web servers responding to SIGTERM signals</li>
            <li>Database connection pools with proper cleanup</li>
            <li>Background job processors with graceful stop</li>
            <li>Long-running services with clean shutdown</li>
        </ul>
        
        <p><strong>Watch your terminal</strong> when the server shuts down to see the graceful shutdown process in action!</p>
        
        <p><small>Graceful shutdown demonstration | Chapter 20.3</small></p>
    </div>
</body>
</html>"#;

    let about_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>About - Graceful Shutdown Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #f0f0f0; }
        .container { background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #d73502; }
        .nav { margin: 20px 0; }
        .nav a { margin-right: 15px; color: #d73502; text-decoration: none; padding: 5px 10px; border-radius: 3px; }
        .nav a:hover { background: #d73502; color: white; }
        .timeline { background: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }
    </style>
</head>
<body>
    <div class="container">
        <h1>About the Complete Web Server Journey</h1>
        
        <div class="nav">
            <a href="/">Home</a>
            <a href="/about">About</a>
            <a href="/shutdown">Shutdown Info</a>
            <a href="/sleep">Sleep Test</a>
        </div>
        
        <p>This server represents the culmination of our Rust web server journey through Chapter 20.</p>
        
        <div class="timeline">
            <h3>🚀 The Complete Journey</h3>
            
            <p><strong>Chapter 20.1:</strong> Single-Threaded Server (Port 7878)</p>
            <ul>
                <li>Basic TCP socket programming</li>
                <li>HTTP request/response handling</li>
                <li>Sequential request processing</li>
                <li>Performance limitations identified</li>
            </ul>
            
            <p><strong>Chapter 20.2:</strong> Multithreaded Server (Port 7879)</p>
            <ul>
                <li>Thread pool implementation</li>
                <li>Concurrent request processing</li>
                <li>Channel-based job distribution</li>
                <li>Improved performance and responsiveness</li>
            </ul>
            
            <p><strong>Chapter 20.3:</strong> Graceful Shutdown (Port 7880) - <em>You are here!</em></p>
            <ul>
                <li>Drop trait implementation</li>
                <li>Message-based worker termination</li>
                <li>Proper resource cleanup</li>
                <li>Production-ready shutdown process</li>
            </ul>
        </div>
        
        <h2>Key Concepts Learned</h2>
        <ul>
            <li><strong>Network Programming:</strong> TCP sockets and HTTP protocol</li>
            <li><strong>Concurrency:</strong> Thread pools and message passing</li>
            <li><strong>Resource Management:</strong> RAII and Drop trait</li>
            <li><strong>Systems Programming:</strong> Building robust server applications</li>
        </ul>
        
        <h2>Production Considerations</h2>
        <p>While this server demonstrates core concepts, production servers would also include:</p>
        <ul>
            <li>Signal handling (SIGTERM, SIGINT)</li>
            <li>Configuration management</li>
            <li>Logging and monitoring</li>
            <li>Security features (HTTPS, input validation)</li>
            <li>Load balancing and scaling</li>
        </ul>
        
        <h2>Next Steps</h2>
        <p>This completes the Rust Book's final project! You've learned how to:</p>
        <ul>
            <li>Build systems-level applications</li>
            <li>Handle concurrency safely</li>
            <li>Manage resources properly</li>
            <li>Create production-ready code</li>
        </ul>
        
        <p>Consider exploring Rust web frameworks like Actix-web, Warp, or Axum for production web development!</p>
        
        <p><small>Final chapter completed! 🎉 | Chapter 20.3 | Rust Journey Complete</small></p>
    </div>
</body>
</html>"#;

    let error_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>404 Not Found - Graceful Shutdown Server</title>
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
        <p>The graceful shutdown server couldn't find this page.</p>
        <p><a href="/">🏠 Return Home</a> | <a href="/shutdown">🛑 Shutdown Info</a></p>
        <p><small>Error handled gracefully</small></p>
    </div>
</body>
</html>"#;

    // Write files to web_assets directory
    std::fs::create_dir_all("web_assets/ch20_web_server").unwrap_or_default();
    write_file_if_not_exists("web_assets/ch20_web_server/graceful.html", graceful_html);
    write_file_if_not_exists("web_assets/ch20_web_server/shutdown.html", shutdown_html);
    write_file_if_not_exists("web_assets/ch20_web_server/about.html", about_html);
    write_file_if_not_exists("web_assets/ch20_web_server/404.html", error_html);
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
        <h1>Graceful Shutdown Server</h1>
        <p>File '{}' not found - served with graceful handling</p>
        <p><a href="/">Return Home</a> | <a href="/shutdown">Shutdown Info</a></p>
    </body></html>"#, filename)
}