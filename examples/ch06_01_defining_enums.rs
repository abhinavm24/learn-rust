use rust_book_examples::print_chapter_header;

// === BASIC ENUM DEFINITION ===
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// === ENUM WITH ASSOCIATED DATA ===
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// === COMPLEX ENUM WITH MIXED VARIANT TYPES ===
#[derive(Debug)]
enum Message {
    Quit,                       // Unit variant - no data
    Move { x: i32, y: i32 },   // Struct variant - named fields
    Write(String),             // Tuple variant - one field
    ChangeColor(i32, i32, i32), // Tuple variant - three fields
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message received"),
            Message::Move { x, y } => println!("Move to coordinates ({}, {})", x, y),
            Message::Write(text) => println!("Write message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

// === ENUM REPRESENTING DIFFERENT SHAPES ===
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}

// === ENUM FOR WEB EVENTS ===
#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

// === ENUM VS STRUCT COMPARISON ===
// Using separate structs (more verbose approach)
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// === ENUM FOR ERROR HANDLING (PREVIEW) ===
#[derive(Debug)]
enum HttpResponse {
    Success(String),
    NotFound,
    ServerError { code: u16, message: String },
    Redirect(String),
}

fn main() {
    print_chapter_header("Chapter 6.1", "Defining an Enum");
    
    // === BASIC ENUM USAGE ===
    println!("\n=== Basic Enum Usage ===");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    println!("IPv4: {:?}", four);
    println!("IPv6: {:?}", six);
    
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    
    // === ENUMS WITH DATA ===
    println!("\n=== Enums with Associated Data ===");
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("Home address: {:?}", home);
    println!("Loopback address: {:?}", loopback);
    
    // === COMPLEX ENUM VARIANTS ===
    println!("\n=== Complex Enum Variants ===");
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello, World!")),
        Message::ChangeColor(255, 128, 0),
    ];
    
    for message in messages {
        println!("Processing message: {:?}", message);
        message.call();
    }
    
    // === SHAPE ENUM EXAMPLE ===
    println!("\n=== Shape Enum Example ===");
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 10.0, height: 20.0 },
        Shape::Triangle { base: 8.0, height: 12.0 },
    ];
    
    for shape in shapes {
        println!("Shape: {:?}", shape);
        println!("Area: {:.2}", shape.area());
        println!();
    }
    
    // === WEB EVENT ENUM ===
    println!("\n=== Web Event Enum ===");
    let events = vec![
        WebEvent::PageLoad,
        WebEvent::PageUnload,
        WebEvent::KeyPress('a'),
        WebEvent::Paste(String::from("Hello from clipboard")),
        WebEvent::Click { x: 320, y: 240 },
    ];
    
    for event in events {
        handle_web_event(event);
    }
    
    // === HTTP RESPONSE ENUM ===
    println!("\n=== HTTP Response Enum ===");
    let responses = vec![
        HttpResponse::Success(String::from("Data retrieved successfully")),
        HttpResponse::NotFound,
        HttpResponse::ServerError { 
            code: 500, 
            message: String::from("Internal server error") 
        },
        HttpResponse::Redirect(String::from("https://example.com")),
    ];
    
    for response in responses {
        handle_http_response(response);
    }
    
    // === ENUM VS MULTIPLE STRUCTS ===
    println!("\n=== Enum vs Multiple Structs ===");
    // With enum, we can store different message types in the same collection
    let mixed_messages = vec![
        Message::Quit,
        Message::Write(String::from("Enum approach")),
        Message::Move { x: 100, y: 200 },
    ];
    
    println!("Mixed messages in one vector:");
    for msg in mixed_messages {
        msg.call();
    }
    
    // With separate structs, we'd need separate collections or trait objects
    let quit_msg = QuitMessage;
    let write_msg = WriteMessage(String::from("Struct approach"));
    // Can't easily put these in the same vector without additional complexity
    
    println!("Separate structs require separate handling");
}

fn route(ip_kind: IpAddrKind) {
    println!("Routing for IP version: {:?}", ip_kind);
}

fn handle_web_event(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("Key pressed: '{}'", c),
        WebEvent::Paste(s) => println!("Pasted text: '{}'", s),
        WebEvent::Click { x, y } => println!("Clicked at coordinates ({}, {})", x, y),
    }
}

fn handle_http_response(response: HttpResponse) {
    match response {
        HttpResponse::Success(data) => println!("✓ Success: {}", data),
        HttpResponse::NotFound => println!("✗ 404 Not Found"),
        HttpResponse::ServerError { code, message } => {
            println!("✗ Server Error {}: {}", code, message);
        }
        HttpResponse::Redirect(url) => println!("↳ Redirecting to: {}", url),
    }
}