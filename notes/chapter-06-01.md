# Chapter 6.1: Defining an Enum

## Key Takeaways

### Enum Fundamentals
- Enums allow you to define types by enumerating possible variants
- Each variant can optionally hold data of different types
- Enums are more powerful than in many languages - variants can contain any type
- Create types that represent a fixed set of possibilities

### Enum vs Struct
- **Structs**: Group related data together (AND relationship)
- **Enums**: Choose between different possibilities (OR relationship)
- Enums can contain structs, and structs can contain enums
- Both can have methods through `impl` blocks

### Variant Data Types
- **Unit variants**: No associated data
- **Tuple variants**: Anonymous fields like tuple structs
- **Struct variants**: Named fields like regular structs
- **Mixed variants**: Different variants can have different data types

### Important Syntax and Operators

#### Enum Definition
```rust
enum EnumName {
    Variant1,
    Variant2(Type),
    Variant3 { field: Type },
}
```

#### Enum Instantiation
```rust
let instance1 = EnumName::Variant1;
let instance2 = EnumName::Variant2(value);
let instance3 = EnumName::Variant3 { field: value };
```

### Programming Concepts Introduced
- **Sum Types**: Representing "one of several possibilities"
- **Variant Pattern**: Different data for different cases
- **Type Safety**: Compiler ensures you handle all possibilities
- **Algebraic Data Types**: Mathematical foundation for type composition

### Code Examples and Patterns

#### Basic Enum Definition
```rust
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {
    // Function that works with any IP address kind
}
```

#### Enum with Associated Data
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
```

#### Complex Enum with Mixed Variant Types
```rust
enum Message {
    Quit,                       // Unit variant
    Move { x: i32, y: i32 },   // Struct variant
    Write(String),             // Tuple variant with one field
    ChangeColor(i32, i32, i32), // Tuple variant with three fields
}

fn main() {
    let quit = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };
    let write_msg = Message::Write(String::from("Hello"));
    let color_msg = Message::ChangeColor(255, 0, 0);
}
```

#### Equivalent Struct Approach (More Verbose)
```rust
// What the Message enum replaces:
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// With enum, all these are unified under one type
```

#### Enum with Methods
```rust
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    let msg = Message::Write(String::from("hello"));
    msg.call();
}
```

#### Real-World Example: HTTP Status
```rust
enum HttpStatus {
    Ok,
    NotFound,
    ServerError(String),
    Redirect { location: String, permanent: bool },
}

impl HttpStatus {
    fn is_success(&self) -> bool {
        match self {
            HttpStatus::Ok => true,
            _ => false,
        }
    }
    
    fn status_code(&self) -> u16 {
        match self {
            HttpStatus::Ok => 200,
            HttpStatus::NotFound => 404,
            HttpStatus::ServerError(_) => 500,
            HttpStatus::Redirect { permanent, .. } => {
                if *permanent { 301 } else { 302 }
            }
        }
    }
}

fn main() {
    let status = HttpStatus::ServerError(String::from("Database connection failed"));
    println!("Status code: {}", status.status_code());
    println!("Is success: {}", status.is_success());
}
```

#### Nested Enums and Structs
```rust
#[derive(Debug)]
enum Currency {
    USD,
    EUR,
    GBP,
    BTC,
}

#[derive(Debug)]
struct Money {
    amount: f64,
    currency: Currency,
}

#[derive(Debug)]
enum PaymentMethod {
    Cash(Money),
    Card { number: String, expiry: String },
    DigitalWallet(String), // wallet ID
}

fn main() {
    let cash_payment = PaymentMethod::Cash(Money {
        amount: 50.0,
        currency: Currency::USD,
    });
    
    let card_payment = PaymentMethod::Card {
        number: String::from("1234-5678-9012-3456"),
        expiry: String::from("12/25"),
    };
    
    println!("{:#?}", cash_payment);
    println!("{:#?}", card_payment);
}
```

#### The Option Enum (Standard Library)
```rust
// This is defined in the standard library
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    // Options are so common they're in the prelude
    // You can use Some and None directly without Option::
}
```

#### Enum with Different Data Complexities
```rust
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
enum Shape {
    Circle(f64),                    // radius
    Rectangle(f64, f64),           // width, height
    Triangle(Point, Point, Point), // three vertices
    Polygon(Vec<Point>),           // multiple vertices
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(_, _, _) => {
                // Complex triangle area calculation would go here
                0.0 // Placeholder
            }
            Shape::Polygon(_) => {
                // Complex polygon area calculation would go here
                0.0 // Placeholder
            }
        }
    }
}

fn main() {
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(10.0, 20.0);
    let triangle = Shape::Triangle(
        Point { x: 0.0, y: 0.0 },
        Point { x: 10.0, y: 0.0 },
        Point { x: 5.0, y: 10.0 },
    );
    
    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());
}
```

### Practical Applications
- State machines (different states with different data)
- Error handling (Result type with Ok/Err variants)
- Optional values (Option type with Some/None variants)
- Parser abstract syntax trees
- UI event systems
- API response types

### Memory Efficiency
- Enums are stored efficiently - only need space for the largest variant
- Tag field indicates which variant is active
- No wasted space for unused variants
- More memory efficient than separate struct types

### Integration with Previous Chapters
- Can contain structs as variant data
- Support methods like structs (Chapter 5.3)
- Use ownership and borrowing rules for variant data
- Can be used as function parameters and return types

### Community Conventions and Idioms
- Use PascalCase for enum names and variants
- Prefer descriptive variant names over abbreviations
- Consider using associated functions for complex construction
- Use derive macros like `#[derive(Debug)]` for development
- Group related constants as unit variants

### Personal Notes
- Enums are much more powerful than in languages like C/Java
- The ability to attach different data to variants is incredibly useful
- Pattern matching (next chapter) makes enums truly shine
- Understanding enums is essential for idiomatic Rust
- They enable type-safe state representation that eliminates many bugs

Official Chapter: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html

---
*Completed: ✓*