# Chapter 20.1: Building a Multithreaded Web Server - Single-Threaded Server

## Key Takeaways

### Core Concepts
- **HTTP Protocol**: Stateless request-response communication over TCP
- **TCP Socket Programming**: Low-level network connections using std::net
- **Single-Threaded Architecture**: Sequential request processing model
- **Web Server Basics**: Listening, accepting connections, parsing requests, sending responses
- **Performance Bottlenecks**: Understanding limitations before optimization

### Important Syntax and Operators
- `TcpListener::bind("127.0.0.1:7878")` - Create server socket bound to address
- `listener.incoming()` - Iterator over incoming TCP connections
- `stream.read(&mut buffer)` - Read data from TCP stream into buffer
- `stream.write(response.as_bytes())` - Write HTTP response to stream
- `stream.flush()` - Ensure all data is sent before closing
- `String::from_utf8_lossy(&buffer[..])` - Convert bytes to UTF-8 string

### Programming Concepts Introduced
- **Network Programming**: Building applications that communicate over networks
- **Protocol Implementation**: Implementing HTTP request/response protocol
- **I/O Operations**: Reading from and writing to network streams
- **Request Routing**: Mapping URLs to different response handlers
- **Static File Serving**: Delivering HTML, CSS, and other static content

## Code Examples and Patterns

### Basic TCP Server Setup
```rust
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {
    // Bind to localhost on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running on http://127.0.0.1:7878");

    // Handle incoming connections sequentially
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
```

### HTTP Request Processing
```rust
fn handle_connection(mut stream: TcpStream) {
    // Read request into buffer
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    // Convert bytes to string for parsing
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request);
    
    // Simple HTTP response
    let response = "HTTP/1.1 200 OK\r\n\r\n<html><body>Hello, World!</body></html>";
    
    // Send response and flush
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

### Request Routing and Static File Serving
```rust
use std::fs;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    // Parse first line of request
    let request_line = String::from_utf8_lossy(&buffer[..])
        .lines()
        .next()
        .unwrap_or("");
    
    // Route based on request path
    let (status_line, filename) = match request_line {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            // Simulate slow request
            std::thread::sleep(std::time::Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    
    // Read file contents
    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        String::from("<html><body><h1>File not found</h1></body></html>")
    });
    
    // Build complete HTTP response
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

### Error Handling and Robustness
```rust
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    
    // Handle read errors gracefully
    match stream.read(&mut buffer) {
        Ok(0) => {
            println!("Client disconnected");
            return Ok(());
        }
        Ok(_) => {
            // Process request
            let response = "HTTP/1.1 200 OK\r\n\r\n<html><body>Hello</body></html>";
            stream.write_all(response.as_bytes())?;
            stream.flush()?;
        }
        Err(e) => {
            eprintln!("Failed to read from connection: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}
```

## Practical Applications
- Building simple web servers for development and testing
- Understanding network protocol implementation
- Creating HTTP API endpoints for microservices
- Implementing custom protocol servers (not just HTTP)
- Learning foundation for web frameworks and reverse proxies
- Building monitoring tools that expose HTTP metrics

## Integration with Previous Chapters
- **Prerequisites**: I/O operations (Chapter 12), error handling (Chapter 9), ownership for managing connections
- **Builds On**: String manipulation from early chapters, pattern matching for request routing
- **Connections**: Uses Vec and collections for buffering, sets up need for concurrency (Chapter 16)

## Community Conventions and Idioms
- Use `hyper` crate for production HTTP servers instead of raw TCP
- Follow HTTP/1.1 specification for proper header handling
- Use `tokio` for async networking in real applications
- Implement proper error handling rather than `unwrap()` everywhere
- Use structured logging (e.g., `tracing` crate) for production servers
- Consider security: input validation, HTTPS, rate limiting

## Personal Notes
- Single-threaded servers are educational but impractical for production
- Raw TCP socket programming helps understand HTTP framework internals
- The blocking nature clearly demonstrates why concurrency is needed
- Good stepping stone to understanding async programming concepts
- Real web servers handle many more HTTP features (keep-alive, chunked encoding, etc.)
- Performance testing reveals the need for multithreading or async I/O

Official Chapter: https://doc.rust-lang.org/book/ch20-01-single-threaded.html

---
*Completed: ✓*