# Chapter 20: Final Project - Building a Multithreaded Web Server

## Key Takeaways

### Web Server Fundamentals
- **TCP Listener**: Accept incoming network connections
- **HTTP Protocol**: Handle HTTP requests and generate responses
- **Request Parsing**: Parse HTTP requests into structured data
- **Response Generation**: Create well-formed HTTP responses

### Concurrency Patterns
- **Single-threaded Server**: Simple but blocks on each request
- **Thread Pool**: Fixed number of worker threads for better resource management
- **Channel Communication**: Message passing between threads for coordination
- **Graceful Shutdown**: Proper cleanup and resource deallocation

### System Programming
- **Network Programming**: TCP sockets and network I/O
- **File System**: Serving static files from disk
- **Error Handling**: Robust error handling for network operations
- **Resource Management**: Memory and thread management

### Production Considerations
- **Performance**: Handling multiple concurrent connections
- **Scalability**: Design patterns for scaling web services
- **Security**: Basic security considerations for web servers
- **Monitoring**: Logging and observability for production systems

## Chapter Structure

### 20.1: Building a Single-Threaded Web Server
```rust
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

// Basic single-threaded server
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";
    
    if buffer.starts_with(get) {
        let contents = fs::read_to_string("hello.html").unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\n\r\n{}",
            contents
        );
        
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!(
            "{}\r\n\r\n{}",
            status_line,
            contents
        );
        
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

// Improved request parsing
struct HttpRequest {
    method: String,
    path: String,
    version: String,
    headers: std::collections::HashMap<String, String>,
}

impl HttpRequest {
    fn parse(buffer: &[u8]) -> Option<Self> {
        let request = String::from_utf8_lossy(buffer);
        let lines: Vec<&str> = request.lines().collect();
        
        if lines.is_empty() {
            return None;
        }
        
        // Parse request line
        let request_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
        if request_line_parts.len() != 3 {
            return None;
        }
        
        let method = request_line_parts[0].to_string();
        let path = request_line_parts[1].to_string();
        let version = request_line_parts[2].to_string();
        
        // Parse headers
        let mut headers = std::collections::HashMap::new();
        for line in lines.iter().skip(1) {
            if line.is_empty() {
                break;
            }
            
            if let Some(pos) = line.find(':') {
                let name = line[..pos].trim().to_string();
                let value = line[pos + 1..].trim().to_string();
                headers.insert(name, value);
            }
        }
        
        Some(HttpRequest {
            method,
            path,
            version,
            headers,
        })
    }
}

// HTTP Response builder
struct HttpResponse {
    status_code: u16,
    status_text: String,
    headers: std::collections::HashMap<String, String>,
    body: String,
}

impl HttpResponse {
    fn new(status_code: u16, status_text: &str) -> Self {
        HttpResponse {
            status_code,
            status_text: status_text.to_string(),
            headers: std::collections::HashMap::new(),
            body: String::new(),
        }
    }
    
    fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.insert(name.to_string(), value.to_string());
        self
    }
    
    fn body(mut self, body: String) -> Self {
        self.body = body;
        self.headers.insert(
            "Content-Length".to_string(),
            self.body.len().to_string(),
        );
        self
    }
    
    fn to_string(&self) -> String {
        let mut response = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status_code,
            self.status_text
        );
        
        for (name, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", name, value));
        }
        
        response.push_str("\r\n");
        response.push_str(&self.body);
        
        response
    }
}

// Route handling
fn route_request(request: &HttpRequest) -> HttpResponse {
    match (&request.method[..], &request.path[..]) {
        ("GET", "/") => {
            let contents = fs::read_to_string("hello.html")
                .unwrap_or_else(|_| "<h1>Hello, World!</h1>".to_string());
            
            HttpResponse::new(200, "OK")
                .header("Content-Type", "text/html")
                .body(contents)
        }
        ("GET", "/sleep") => {
            std::thread::sleep(std::time::Duration::from_secs(5));
            let contents = "<h1>Sleep completed</h1>";
            
            HttpResponse::new(200, "OK")
                .header("Content-Type", "text/html")
                .body(contents.to_string())
        }
        _ => {
            let contents = fs::read_to_string("404.html")
                .unwrap_or_else(|_| "<h1>404 Not Found</h1>".to_string());
            
            HttpResponse::new(404, "NOT FOUND")
                .header("Content-Type", "text/html")
                .body(contents)
        }
    }
}
```

### 20.2: Turning Our Single-Threaded Server into a Multithreaded Server
```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Thread pool implementation
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool { workers, sender }
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            
            println!("Worker {} got a job; executing.", id);
            
            job();
        });
        
        Worker { id, thread }
    }
}

// Using the thread pool
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    if let Some(request) = HttpRequest::parse(&buffer) {
        let response = route_request(&request);
        let response_string = response.to_string();
        
        stream.write(response_string.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
```

### 20.3: Graceful Shutdown and Cleanup
```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Enhanced ThreadPool with graceful shutdown
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        
        if let Some(ref sender) = self.sender {
            sender.send(Message::NewJob(job)).unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        
        for _ in &self.workers {
            if let Some(ref sender) = self.sender {
                sender.send(Message::Terminate).unwrap();
            }
        }
        
        println!("Shutting down all workers.");
        
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
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

// Server with signal handling for graceful shutdown
use std::sync::atomic::{AtomicBool, Ordering};

static SHUTDOWN: AtomicBool = AtomicBool::new(false);

fn main() {
    // Set up signal handler (simplified - in real code use signal-hook crate)
    ctrlc::set_handler(move || {
        println!("Received Ctrl+C! Shutting down gracefully...");
        SHUTDOWN.store(true, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.set_nonblocking(true).unwrap();
    
    let pool = ThreadPool::new(4);
    
    println!("Server running on http://127.0.0.1:7878");
    
    for stream in listener.incoming() {
        if SHUTDOWN.load(Ordering::SeqCst) {
            println!("Shutdown signal received, stopping server...");
            break;
        }
        
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Non-blocking socket would block, continue loop
                std::thread::sleep(std::time::Duration::from_millis(10));
                continue;
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
    
    println!("Server shutdown complete.");
}
```

## Advanced Web Server Features

### Static File Serving
```rust
use std::path::{Path, PathBuf};
use std::fs;

struct StaticFileHandler {
    root_dir: PathBuf,
}

impl StaticFileHandler {
    fn new<P: AsRef<Path>>(root_dir: P) -> Self {
        StaticFileHandler {
            root_dir: root_dir.as_ref().to_path_buf(),
        }
    }
    
    fn serve_file(&self, path: &str) -> HttpResponse {
        // Security: prevent directory traversal
        let sanitized_path = path.trim_start_matches('/');
        let file_path = self.root_dir.join(sanitized_path);
        
        // Ensure the resolved path is still within root_dir
        if !file_path.starts_with(&self.root_dir) {
            return self.forbidden_response();
        }
        
        match fs::read(&file_path) {
            Ok(contents) => {
                let content_type = self.guess_content_type(&file_path);
                
                HttpResponse::new(200, "OK")
                    .header("Content-Type", &content_type)
                    .header("Cache-Control", "public, max-age=3600")
                    .body(String::from_utf8_lossy(&contents).to_string())
            }
            Err(_) => self.not_found_response(),
        }
    }
    
    fn guess_content_type(&self, path: &Path) -> String {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("html") | Some("htm") => "text/html".to_string(),
            Some("css") => "text/css".to_string(),
            Some("js") => "application/javascript".to_string(),
            Some("json") => "application/json".to_string(),
            Some("png") => "image/png".to_string(),
            Some("jpg") | Some("jpeg") => "image/jpeg".to_string(),
            Some("gif") => "image/gif".to_string(),
            Some("svg") => "image/svg+xml".to_string(),
            Some("txt") => "text/plain".to_string(),
            _ => "application/octet-stream".to_string(),
        }
    }
    
    fn not_found_response(&self) -> HttpResponse {
        HttpResponse::new(404, "NOT FOUND")
            .header("Content-Type", "text/html")
            .body("<h1>404 - File Not Found</h1>".to_string())
    }
    
    fn forbidden_response(&self) -> HttpResponse {
        HttpResponse::new(403, "FORBIDDEN")
            .header("Content-Type", "text/html")
            .body("<h1>403 - Forbidden</h1>".to_string())
    }
}
```

### Request Router
```rust
use std::collections::HashMap;

type RouteHandler = Box<dyn Fn(&HttpRequest) -> HttpResponse + Send + Sync>;

struct Router {
    routes: HashMap<(String, String), RouteHandler>,  // (method, path) -> handler
    static_handler: Option<StaticFileHandler>,
}

impl Router {
    fn new() -> Self {
        Router {
            routes: HashMap::new(),
            static_handler: None,
        }
    }
    
    fn route<F>(mut self, method: &str, path: &str, handler: F) -> Self
    where
        F: Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    {
        self.routes.insert(
            (method.to_uppercase(), path.to_string()),
            Box::new(handler),
        );
        self
    }
    
    fn static_files<P: AsRef<Path>>(mut self, root_dir: P) -> Self {
        self.static_handler = Some(StaticFileHandler::new(root_dir));
        self
    }
    
    fn handle_request(&self, request: &HttpRequest) -> HttpResponse {
        // Try exact route match first
        if let Some(handler) = self.routes.get(&(request.method.clone(), request.path.clone())) {
            return handler(request);
        }
        
        // Try static file serving
        if let Some(ref static_handler) = self.static_handler {
            if request.method == "GET" {
                return static_handler.serve_file(&request.path);
            }
        }
        
        // Default 404 response
        HttpResponse::new(404, "NOT FOUND")
            .header("Content-Type", "text/html")
            .body("<h1>404 - Page Not Found</h1>".to_string())
    }
}

// Usage example
fn create_router() -> Router {
    Router::new()
        .route("GET", "/", |_req| {
            HttpResponse::new(200, "OK")
                .header("Content-Type", "text/html")
                .body("<h1>Welcome to Rust Web Server!</h1>".to_string())
        })
        .route("GET", "/api/health", |_req| {
            HttpResponse::new(200, "OK")
                .header("Content-Type", "application/json")
                .body(r#"{"status": "healthy", "timestamp": "2024-01-01T00:00:00Z"}"#.to_string())
        })
        .route("POST", "/api/echo", |req| {
            HttpResponse::new(200, "OK")
                .header("Content-Type", "application/json")
                .body(format!(r#"{{"method": "{}", "path": "{}"}}"#, req.method, req.path))
        })
        .static_files("./public")
}
```

### Middleware System
```rust
trait Middleware: Send + Sync {
    fn process(&self, request: &mut HttpRequest, response: &mut HttpResponse) -> bool;
}

struct LoggingMiddleware;

impl Middleware for LoggingMiddleware {
    fn process(&self, request: &mut HttpRequest, _response: &mut HttpResponse) -> bool {
        println!(
            "[{}] {} {} - {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            request.method,
            request.path,
            request.headers.get("User-Agent").unwrap_or(&"Unknown".to_string())
        );
        true  // Continue processing
    }
}

struct CorsMiddleware {
    allowed_origins: Vec<String>,
}

impl CorsMiddleware {
    fn new(origins: Vec<String>) -> Self {
        CorsMiddleware {
            allowed_origins: origins,
        }
    }
}

impl Middleware for CorsMiddleware {
    fn process(&self, request: &mut HttpRequest, response: &mut HttpResponse) -> bool {
        if let Some(origin) = request.headers.get("Origin") {
            if self.allowed_origins.contains(&"*".to_string()) 
                || self.allowed_origins.contains(origin) {
                response.headers.insert("Access-Control-Allow-Origin".to_string(), origin.clone());
                response.headers.insert("Access-Control-Allow-Methods".to_string(), "GET, POST, PUT, DELETE".to_string());
                response.headers.insert("Access-Control-Allow-Headers".to_string(), "Content-Type, Authorization".to_string());
            }
        }
        true
    }
}

struct MiddlewareStack {
    middlewares: Vec<Box<dyn Middleware>>,
}

impl MiddlewareStack {
    fn new() -> Self {
        MiddlewareStack {
            middlewares: Vec::new(),
        }
    }
    
    fn add<M: Middleware + 'static>(mut self, middleware: M) -> Self {
        self.middlewares.push(Box::new(middleware));
        self
    }
    
    fn process(&self, request: &mut HttpRequest, response: &mut HttpResponse) -> bool {
        for middleware in &self.middlewares {
            if !middleware.process(request, response) {
                return false;  // Stop processing if middleware returns false
            }
        }
        true
    }
}
```

### Error Handling and Logging
```rust
use std::fmt;

#[derive(Debug)]
enum ServerError {
    IoError(std::io::Error),
    ParseError(String),
    InternalError(String),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerError::IoError(e) => write!(f, "IO error: {}", e),
            ServerError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ServerError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for ServerError {}

impl From<std::io::Error> for ServerError {
    fn from(error: std::io::Error) -> Self {
        ServerError::IoError(error)
    }
}

// Enhanced connection handler with error handling
fn handle_connection_safe(mut stream: TcpStream) -> Result<(), ServerError> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    
    if bytes_read == 0 {
        return Ok(());  // Client closed connection
    }
    
    let request = match HttpRequest::parse(&buffer[..bytes_read]) {
        Some(req) => req,
        None => {
            let response = HttpResponse::new(400, "BAD REQUEST")
                .header("Content-Type", "text/html")
                .body("<h1>400 - Bad Request</h1>".to_string());
            
            stream.write(response.to_string().as_bytes())?;
            stream.flush()?;
            return Ok(());
        }
    };
    
    let router = create_router();
    let mut response = router.handle_request(&request);
    
    // Add server header
    response.headers.insert("Server".to_string(), "Rust-WebServer/1.0".to_string());
    response.headers.insert("Date".to_string(), chrono::Utc::now().to_rfc2822());
    
    stream.write(response.to_string().as_bytes())?;
    stream.flush()?;
    
    Ok(())
}

// Main server loop with error handling
fn run_server() -> Result<(), ServerError> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let pool = ThreadPool::new(4);
    
    println!("Server running on http://127.0.0.1:7878");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(move || {
                    if let Err(e) = handle_connection_safe(stream) {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
    
    Ok(())
}
```

### Configuration Management
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ServerConfig {
    #[serde(default = "default_host")]
    host: String,
    
    #[serde(default = "default_port")]
    port: u16,
    
    #[serde(default = "default_worker_count")]
    worker_count: usize,
    
    #[serde(default = "default_static_dir")]
    static_dir: String,
    
    #[serde(default)]
    cors_origins: Vec<String>,
    
    #[serde(default = "default_request_timeout")]
    request_timeout_seconds: u64,
}

fn default_host() -> String { "127.0.0.1".to_string() }
fn default_port() -> u16 { 7878 }
fn default_worker_count() -> usize { 4 }
fn default_static_dir() -> String { "./public".to_string() }
fn default_request_timeout() -> u64 { 30 }

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: default_host(),
            port: default_port(),
            worker_count: default_worker_count(),
            static_dir: default_static_dir(),
            cors_origins: vec!["*".to_string()],
            request_timeout_seconds: default_request_timeout(),
        }
    }
}

impl ServerConfig {
    fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: ServerConfig = toml::from_str(&content)?;
        Ok(config)
    }
    
    fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

// Main function with configuration
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig::load_from_file("server.toml")
        .unwrap_or_else(|_| {
            println!("Could not load config file, using defaults");
            ServerConfig::default()
        });
    
    println!("Starting server with config: {:#?}", config);
    
    let listener = TcpListener::bind(config.bind_address())?;
    let pool = ThreadPool::new(config.worker_count);
    
    let middleware_stack = MiddlewareStack::new()
        .add(LoggingMiddleware)
        .add(CorsMiddleware::new(config.cors_origins.clone()));
    
    let router = create_router().static_files(&config.static_dir);
    
    println!("Server running on http://{}", config.bind_address());
    
    for stream in listener.incoming() {
        let stream = stream?;
        
        pool.execute(move || {
            if let Err(e) = handle_connection_safe(stream) {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
    
    Ok(())
}
```

## Testing the Web Server

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_http_request_parsing() {
        let request_data = b"GET /test HTTP/1.1\r\nHost: localhost\r\nUser-Agent: test\r\n\r\n";
        let request = HttpRequest::parse(request_data).unwrap();
        
        assert_eq!(request.method, "GET");
        assert_eq!(request.path, "/test");
        assert_eq!(request.version, "HTTP/1.1");
        assert_eq!(request.headers.get("Host"), Some(&"localhost".to_string()));
    }
    
    #[test]
    fn test_http_response_formatting() {
        let response = HttpResponse::new(200, "OK")
            .header("Content-Type", "text/html")
            .body("<h1>Test</h1>".to_string());
        
        let response_string = response.to_string();
        assert!(response_string.contains("HTTP/1.1 200 OK"));
        assert!(response_string.contains("Content-Type: text/html"));
        assert!(response_string.contains("<h1>Test</h1>"));
    }
    
    #[test]
    fn test_thread_pool() {
        let pool = ThreadPool::new(4);
        let (tx, rx) = std::sync::mpsc::channel();
        
        for i in 0..10 {
            let tx = tx.clone();
            pool.execute(move || {
                tx.send(i).unwrap();
            });
        }
        
        drop(tx);  // Close the channel
        
        let mut results = Vec::new();
        while let Ok(result) = rx.recv() {
            results.push(result);
        }
        
        assert_eq!(results.len(), 10);
        results.sort();
        assert_eq!(results, (0..10).collect::<Vec<_>>());
    }
    
    // Integration test (requires running server)
    #[test]
    #[ignore]  // Ignored by default, run with --ignored
    fn test_server_integration() {
        // Start server in background thread
        thread::spawn(|| {
            run_server().unwrap();
        });
        
        // Give server time to start
        thread::sleep(Duration::from_millis(100));
        
        // Make HTTP request
        let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
        stream.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
        
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        
        assert!(response.contains("HTTP/1.1 200 OK"));
    }
}
```

## Performance and Production Considerations

### Benchmarking
```rust
use std::time::Instant;

fn benchmark_request_handling() {
    let request_data = b"GET / HTTP/1.1\r\nHost: localhost\r\nUser-Agent: benchmark\r\n\r\n";
    let router = create_router();
    
    let start = Instant::now();
    let iterations = 10_000;
    
    for _ in 0..iterations {
        if let Some(request) = HttpRequest::parse(request_data) {
            let _response = router.handle_request(&request);
        }
    }
    
    let duration = start.elapsed();
    println!("Processed {} requests in {:?}", iterations, duration);
    println!("Average: {:?} per request", duration / iterations);
}
```

### Memory Management
```rust
// Connection pooling for better resource management
struct ConnectionPool {
    max_connections: usize,
    active_connections: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl ConnectionPool {
    fn new(max_connections: usize) -> Self {
        ConnectionPool {
            max_connections,
            active_connections: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }
    
    fn try_acquire(&self) -> Option<ConnectionGuard> {
        let current = self.active_connections.load(std::sync::atomic::Ordering::SeqCst);
        if current < self.max_connections {
            self.active_connections.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Some(ConnectionGuard {
                pool: std::sync::Arc::clone(&self.active_connections),
            })
        } else {
            None
        }
    }
}

struct ConnectionGuard {
    pool: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        self.pool.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    }
}
```

### Security Considerations
```rust
// Basic security headers middleware
struct SecurityHeadersMiddleware;

impl Middleware for SecurityHeadersMiddleware {
    fn process(&self, _request: &mut HttpRequest, response: &mut HttpResponse) -> bool {
        response.headers.insert("X-Content-Type-Options".to_string(), "nosniff".to_string());
        response.headers.insert("X-Frame-Options".to_string(), "DENY".to_string());
        response.headers.insert("X-XSS-Protection".to_string(), "1; mode=block".to_string());
        response.headers.insert("Strict-Transport-Security".to_string(), "max-age=31536000".to_string());
        true
    }
}

// Request size limiting
fn validate_request_size(buffer: &[u8]) -> Result<(), ServerError> {
    const MAX_REQUEST_SIZE: usize = 8192;  // 8KB limit
    
    if buffer.len() > MAX_REQUEST_SIZE {
        return Err(ServerError::ParseError("Request too large".to_string()));
    }
    
    Ok(())
}
```

## Best Practices Summary

### Architecture
- ✅ **Separation of Concerns**: Router, middleware, handlers in separate modules
- ✅ **Error Handling**: Proper error types and recovery strategies
- ✅ **Configuration**: External configuration files for deployment flexibility
- ✅ **Logging**: Comprehensive logging for debugging and monitoring

### Performance
- ✅ **Thread Pool**: Fixed number of workers to avoid thread explosion
- ✅ **Connection Limits**: Protect against resource exhaustion
- ✅ **Static File Caching**: Appropriate cache headers for static content
- ✅ **Request Timeout**: Prevent hanging connections

### Security
- ✅ **Input Validation**: Validate and sanitize all input
- ✅ **Path Traversal Prevention**: Secure static file serving
- ✅ **Security Headers**: Basic security headers for web browsers
- ✅ **Request Size Limits**: Prevent memory exhaustion attacks

### Production Readiness
- ✅ **Graceful Shutdown**: Clean resource cleanup on termination
- ✅ **Error Recovery**: Resilient error handling without crashing
- ✅ **Monitoring**: Health checks and metrics endpoints
- ✅ **Documentation**: Clear API documentation and deployment guides

Official Chapter: https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html

---
*Completed: ✓*