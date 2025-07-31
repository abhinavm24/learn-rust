//! Chapter 20.1: Building a Single-Threaded Web Server
//! 
//! This example demonstrates building a basic single-threaded HTTP server:
//! - TCP socket programming with std::net
//! - HTTP request parsing and response generation
//! - Request routing based on URL paths
//! - Serving static HTML files
//! - Understanding performance limitations

use rust_book_examples::print_chapter_header;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    print_chapter_header("Chapter 20.1", "Single-Threaded Web Server");
    
    println!("Starting single-threaded web server...");
    println!("This server will process requests sequentially, one at a time.");
    println!("Visit http://127.0.0.1:7878 to test the server");
    println!("Try http://127.0.0.1:7878/sleep to see blocking behavior");
    println!("Press Ctrl+C to stop the server\n");
    
    // Create static HTML files if they don't exist
    create_html_files();
    
    // Bind to localhost on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("🚀 Server listening on http://127.0.0.1:7878");
    
    // Process requests sequentially (single-threaded)
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        println!("\n--- New Connection ---");
        handle_connection(stream);
    }
}

/// Handles an individual HTTP connection
fn handle_connection(mut stream: TcpStream) {
    // Read the request into a buffer
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
    
    // Convert buffer to string for parsing
    let request = String::from_utf8_lossy(&buffer[..]);
    
    // Extract the first line (request line)
    let request_line = request
        .lines()
        .next()
        .unwrap_or("");
    
    println!("📨 Request: {}", request_line);
    
    // Route the request based on the path
    let (status_line, filename) = route_request(request_line);
    
    // Read the file content
    let contents = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(_) => {
            println!("⚠️  File '{}' not found, using fallback content", filename);
            create_fallback_content(filename)
        }
    };
    
    // Build the complete HTTP response
    let response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    
    // Send the response
    match stream.write_all(response.as_bytes()) {
        Ok(_) => {
            stream.flush().unwrap();
            println!("✅ Response sent successfully");
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
            println!("😴 Simulating slow request (5 second delay)...");
            println!("⚠️  NOTICE: This will block ALL other requests until complete!");
            
            // Simulate a slow request
            thread::sleep(Duration::from_secs(5));
            
            println!("⏰ Slow request completed");
            ("HTTP/1.1 200 OK", "hello.html")
        }
        "GET /about HTTP/1.1" => {
            println!("ℹ️  Serving about page");
            ("HTTP/1.1 200 OK", "about.html")
        }
        "GET /test HTTP/1.1" => {
            println!("🧪 Serving test page");
            ("HTTP/1.1 200 OK", "test.html")
        }
        _ => {
            println!("❌ Unknown route: {}", request_line);
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        }
    }
}

/// Creates static HTML files for the server to serve
fn create_html_files() {
    // Create hello.html
    let hello_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Rust Web Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #f0f0f0; }
        .container { background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #d73502; }
        .nav { margin: 20px 0; }
        .nav a { margin-right: 15px; color: #d73502; text-decoration: none; }
        .nav a:hover { text-decoration: underline; }
        .info { background: #e8f4fd; padding: 15px; border-left: 4px solid #2196F3; margin: 20px 0; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🦀 Rust Single-Threaded Web Server</h1>
        <p>Welcome to our simple HTTP server built with Rust!</p>
        
        <div class="nav">
            <a href="/">Home</a>
            <a href="/about">About</a>
            <a href="/test">Test</a>
            <a href="/sleep">Sleep Test</a>
        </div>
        
        <div class="info">
            <strong>Note:</strong> This is a single-threaded server, so it can only handle one request at a time.
            Try opening multiple tabs to <a href="/sleep">/sleep</a> to see the blocking behavior!
        </div>
        
        <h2>Server Features</h2>
        <ul>
            <li>✅ HTTP/1.1 request parsing</li>
            <li>✅ Basic routing</li>
            <li>✅ Static file serving</li>
            <li>✅ 404 error handling</li>
            <li>⚠️ Single-threaded (blocking)</li>
        </ul>
        
        <p><small>Built with ❤️ in Rust | Chapter 20.1</small></p>
    </div>
</body>
</html>"#;
    
    // Create about.html
    let about_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>About - Rust Web Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #f0f0f0; }
        .container { background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #d73502; }
        .nav { margin: 20px 0; }
        .nav a { margin-right: 15px; color: #d73502; text-decoration: none; }
        .nav a:hover { text-decoration: underline; }
        code { background: #f5f5f5; padding: 2px 6px; border-radius: 3px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>About This Server</h1>
        
        <div class="nav">
            <a href="/">Home</a>
            <a href="/about">About</a>
            <a href="/test">Test</a>
            <a href="/sleep">Sleep Test</a>
        </div>
        
        <p>This web server is a learning example from <strong>The Rust Programming Language</strong> book, Chapter 20.</p>
        
        <h2>Technical Details</h2>
        <ul>
            <li><strong>Language:</strong> Rust 🦀</li>
            <li><strong>Architecture:</strong> Single-threaded</li>
            <li><strong>Protocol:</strong> HTTP/1.1</li>
            <li><strong>Port:</strong> 7878</li>
            <li><strong>Concurrency:</strong> None (sequential processing)</li>
        </ul>
        
        <h2>Key Components</h2>
        <ul>
            <li><code>TcpListener</code> - Listens for incoming connections</li>
            <li><code>TcpStream</code> - Handles individual connections</li>
            <li>HTTP request parsing and response generation</li>
            <li>File serving with proper Content-Length headers</li>
        </ul>
        
        <h2>Limitations</h2>
        <p>Being single-threaded means this server can only handle one request at a time. 
        If one request takes a long time (like <a href="/sleep">/sleep</a>), all other requests must wait.</p>
        
        <p>The next chapters will show how to improve this with multithreading!</p>
    </div>
</body>
</html>"#;
    
    // Create test.html
    let test_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Test Page - Rust Web Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #f0f0f0; }
        .container { background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #d73502; }
        .nav { margin: 20px 0; }
        .nav a { margin-right: 15px; color: #d73502; text-decoration: none; }
        .nav a:hover { text-decoration: underline; }
        .test-section { background: #f9f9f9; padding: 15px; margin: 15px 0; border-radius: 5px; }
        button { background: #d73502; color: white; border: none; padding: 10px 20px; border-radius: 5px; cursor: pointer; }
        button:hover { background: #b52d02; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🧪 Server Test Page</h1>
        
        <div class="nav">
            <a href="/">Home</a>
            <a href="/about">About</a>
            <a href="/test">Test</a>
            <a href="/sleep">Sleep Test</a>
        </div>
        
        <div class="test-section">
            <h3>Concurrency Test</h3>
            <p>Open multiple tabs to <a href="/sleep" target="_blank">/sleep</a> to test single-threaded behavior.</p>
            <p>You'll notice that requests are processed one at a time!</p>
            <button onclick="window.open('/sleep', '_blank')">Open Sleep Test in New Tab</button>
        </div>
        
        <div class="test-section">
            <h3>Request Information</h3>
            <p>Check your terminal to see request logging and server behavior.</p>
            <p>Each request shows:</p>
            <ul>
                <li>Connection establishment</li>
                <li>Request line parsing</li>
                <li>Route handling</li>
                <li>Response status</li>
            </ul>
        </div>
        
        <div class="test-section">
            <h3>Error Testing</h3>
            <p>Try visiting a non-existent page: <a href="/nonexistent">/nonexistent</a></p>
            <p>This will demonstrate 404 error handling.</p>
        </div>
        
        <script>
            console.log('🦀 Rust web server test page loaded!');
            console.log('Server timestamp:', new Date().toISOString());
        </script>
    </div>
</body>
</html>"#;
    
    // Create 404.html
    let error_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>404 Not Found - Rust Web Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #f0f0f0; }
        .container { background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); text-align: center; }
        h1 { color: #d73502; font-size: 4em; margin: 0; }
        h2 { color: #666; }
        .nav { margin: 20px 0; }
        .nav a { margin-right: 15px; color: #d73502; text-decoration: none; }
        .nav a:hover { text-decoration: underline; }
        .error-info { background: #ffebee; padding: 20px; border-radius: 5px; margin: 20px 0; }
    </style>
</head>
<body>
    <div class="container">
        <h1>404</h1>
        <h2>Page Not Found</h2>
        
        <div class="error-info">
            <p>🚫 Sorry, the page you're looking for doesn't exist on this server.</p>
            <p>This single-threaded Rust web server only handles a few basic routes.</p>
        </div>
        
        <div class="nav">
            <a href="/">🏠 Home</a>
            <a href="/about">ℹ️ About</a>
            <a href="/test">🧪 Test</a>
            <a href="/sleep">😴 Sleep Test</a>
        </div>
        
        <p><small>Error handled by Rust web server | Chapter 20.1</small></p>
    </div>
</body>
</html>"#;
    
    // Write files, but don't overwrite if they exist
    write_file_if_not_exists("hello.html", hello_html);
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
    match filename {
        "404.html" => {
            r#"<html><body>
                <h1>404 Not Found</h1>
                <p>The page you requested could not be found.</p>
                <p><a href="/">Return to home</a></p>
            </body></html>"#.to_string()
        }
        _ => {
            format!(r#"<html><body>
                <h1>File Not Found</h1>
                <p>The file '{}' could not be found.</p>
                <p>This is fallback content from the Rust server.</p>
                <p><a href="/">Return to home</a></p>
            </body></html>"#, filename)
        }
    }
}