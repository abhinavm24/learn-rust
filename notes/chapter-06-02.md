# Chapter 6.2: The match Control Flow Construct

## Key Takeaways

### Match Fundamentals
- `match` is a powerful control flow construct for pattern matching
- Compares a value against patterns and executes code for the first match
- Must be exhaustive - all possible values must be handled
- Patterns can extract values from enums, structs, tuples, and other types

### Pattern Matching Power
- More powerful than switch statements in other languages
- Can destructure complex data types
- Compiler ensures all cases are covered
- Arms are checked in order, first match wins

### Match Syntax
- Each arm has a pattern and code: `pattern => code,`
- Use `{}` for multi-line code blocks
- Use `_` as catch-all pattern (like default in switch)
- Arms must return the same type (if match returns a value)

### Important Syntax and Operators

#### Basic Match Syntax
```rust
match value {
    pattern1 => code1,
    pattern2 => code2,
    pattern3 => {
        // multi-line code
        result
    },
    _ => default_code,
}
```

### Programming Concepts Introduced
- **Pattern Matching**: Structural comparison and deconstruction
- **Exhaustiveness**: Compiler-enforced completeness
- **Control Flow**: Decision making based on data structure
- **Value Extraction**: Getting data out of complex types

### Code Examples and Patterns

#### Basic Enum Matching
```rust
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Quarter;
    println!("Value: {} cents", value_in_cents(coin));
}
```

#### Matching with Data Extraction
```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Texas,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value: {} cents", value_in_cents(coin));
}
```

#### Matching with Option<T>
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("{:?}, {:?}", six, none); // Some(6), None
}
```

#### Complex Pattern Matching
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
    }
}

fn main() {
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 30 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(200, 255, 255),
    ];
    
    for msg in messages {
        process_message(msg);
    }
}
```

#### Catch-All Patterns
```rust
fn dice_roll(roll: u8) -> String {
    match roll {
        1 => String::from("Critical failure!"),
        6 => String::from("Critical success!"),
        2 | 3 | 4 | 5 => String::from("Normal roll"),
        _ => String::from("Invalid dice roll"),
    }
}

// Alternative: using a variable for catch-all
fn dice_roll_with_value(roll: u8) -> String {
    match roll {
        1 => String::from("Critical failure!"),
        6 => String::from("Critical success!"),
        other => format!("You rolled a {}", other),
    }
}

fn main() {
    println!("{}", dice_roll(1));
    println!("{}", dice_roll(4));
    println!("{}", dice_roll_with_value(3));
}
```

#### Matching Tuples
```rust
fn describe_point(point: (i32, i32)) -> String {
    match point {
        (0, 0) => String::from("Origin"),
        (0, y) => format!("On the Y axis at {}", y),
        (x, 0) => format!("On the X axis at {}", x),
        (x, y) => format!("Point at ({}, {})", x, y),
    }
}

fn main() {
    let points = vec![(0, 0), (0, 5), (3, 0), (2, 4)];
    
    for point in points {
        println!("{}", describe_point(point));
    }
}
```

#### Matching with Guards (Conditions)
```rust
fn check_number(x: Option<i32>) -> String {
    match x {
        Some(n) if n < 0 => String::from("Negative number"),
        Some(n) if n == 0 => String::from("Zero"),
        Some(n) if n > 100 => String::from("Large positive number"),
        Some(n) => format!("Positive number: {}", n),
        None => String::from("No number"),
    }
}

fn main() {
    let numbers = vec![Some(-5), Some(0), Some(42), Some(150), None];
    
    for num in numbers {
        println!("{}", check_number(num));
    }
}
```

#### Exhaustiveness Example (Won't Compile)
```rust
fn incomplete_match(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Missing Coin::Quarter - compiler error!
    }
}
```

#### Ignoring Parts of Values
```rust
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let point = Point3D { x: 1, y: 2, z: 3 };
    
    match point {
        Point3D { x, y: 0, z } => println!("On x-z plane: x={}, z={}", x, z),
        Point3D { x: 0, y, z: 0 } => println!("On y axis: y={}", y),
        Point3D { x, y, z: _ } => println!("Point at x={}, y={}, ignoring z", x, y),
    }
}
```

#### Matching References
```rust
fn analyze_reference(value: &Option<i32>) -> String {
    match value {
        Some(n) => format!("Got a number: {}", n),
        None => String::from("Got nothing"),
    }
}

fn analyze_owned(value: Option<i32>) -> String {
    match value {
        Some(n) => format!("Got a number: {}", n),
        None => String::from("Got nothing"),
    }
}

fn main() {
    let x = Some(42);
    
    // Match against reference
    println!("{}", analyze_reference(&x));
    
    // Match against owned value
    println!("{}", analyze_owned(x));
}
```

#### Real-World HTTP Status Matching
```rust
#[derive(Debug)]
enum HttpStatus {
    Ok,
    NotFound,
    ServerError(String),
    Redirect { location: String, permanent: bool },
}

fn handle_response(status: HttpStatus) -> String {
    match status {
        HttpStatus::Ok => {
            String::from("Request successful")
        }
        HttpStatus::NotFound => {
            String::from("Page not found")
        }
        HttpStatus::ServerError(error) => {
            format!("Server error occurred: {}", error)
        }
        HttpStatus::Redirect { location, permanent } => {
            let redirect_type = if permanent { "permanent" } else { "temporary" };
            format!("Redirecting to {} ({})", location, redirect_type)
        }
    }
}

fn main() {
    let responses = vec![
        HttpStatus::Ok,
        HttpStatus::NotFound,
        HttpStatus::ServerError(String::from("Database connection failed")),
        HttpStatus::Redirect {
            location: String::from("https://example.com"),
            permanent: true,
        },
    ];
    
    for response in responses {
        println!("{}", handle_response(response));
    }
}
```

### Practical Applications
- State machine implementations
- Parser and interpreter implementations
- Error handling with Result types
- API response processing
- Command processing systems
- Configuration validation

### Integration with Previous Chapters
- Works seamlessly with enums from Chapter 6.1
- Uses borrowing concepts for pattern matching references
- Builds on control flow from Chapter 3.5
- Enables safe data extraction from complex types

### Community Conventions and Idioms
- Always handle all enum variants explicitly
- Use `_` sparingly - prefer explicit matching when possible
- Group related patterns when appropriate
- Use guards for additional conditions
- Consider creating helper functions for complex match arms

### Performance Notes
- `match` compiles to efficient jump tables when possible
- No runtime overhead compared to if/else chains
- Pattern matching is zero-cost abstraction
- Compiler optimizations can make matches very fast

### Personal Notes
- `match` is one of Rust's most powerful features
- Exhaustiveness checking prevents many bugs at compile time
- Pattern matching makes complex data handling elegant
- The combination of enums and match is incredibly expressive
- Learning to think in patterns transforms how you structure data

Official Chapter: https://doc.rust-lang.org/book/ch06-02-match.html

---
*Completed: ✓*