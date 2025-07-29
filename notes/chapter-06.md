# Chapter 6: Enums and Pattern Matching

## Key Takeaways

### Enum Fundamentals
- **Algebraic Data Types**: Enums represent data that can be one of several variants
- **Type Safety**: Compile-time guarantee of which variant is being used
- **Memory Efficiency**: Only stores data for the active variant
- **Pattern Matching**: Exhaustive matching ensures all cases are handled

### Enum Design Patterns
- **Option Type**: Null safety through Option<T>
- **Result Type**: Error handling through Result<T, E>
- **State Machines**: Model different states with different data
- **Message Passing**: Represent different types of messages/events

### Pattern Matching Power
- **match Expressions**: Exhaustive pattern matching
- **if let**: Convenient for single-pattern matching
- **Destructuring**: Extract data from enum variants
- **Guards**: Additional conditions in patterns

### Error Handling Philosophy
- **No Null**: Option<T> replaces null pointers
- **Explicit Errors**: Result<T, E> makes errors visible
- **Recoverable vs Unrecoverable**: Different strategies for different error types
- **Composable**: Error types can be combined and transformed

## Chapter Structure

### 6.1: Defining an Enum
```rust
// Basic enum
enum IpAddrKind {
    V4,
    V6,
}

// Enum with data
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Complex enum with different data types
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },   // Anonymous struct
    Write(String),              // Single value
    ChangeColor(i32, i32, i32), // Tuple
}

// Using enums
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

// Methods on enums
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to {}, {}", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: {}, {}, {}", r, g, b),
        }
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

### 6.2: The match Control Flow Construct
```rust
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... other states
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

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Catch-all patterns
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),  // Catch-all with value
}

// Ignore value with _
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),  // Ignore the value
}

// Do nothing
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),  // Unit value - do nothing
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(spaces: u8) {}
fn reroll() {}
```

### 6.3: Concise Control Flow with if let
```rust
// Verbose match
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}

// Concise if let
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}

// if let with else
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}

// Multiple if let patterns
let favorite_color: Option<&str> = None;
let is_tuesday = false;
let age: Result<u8, _> = "34".parse();

if let Some(color) = favorite_color {
    println!("Using your favorite color, {}, as the background", color);
} else if is_tuesday {
    println!("Tuesday is green day!");
} else if let Ok(age) = age {
    if age > 30 {
        println!("Using purple as the background color");
    } else {
        println!("Using orange as the background color");
    }
} else {
    println!("Using blue as the background color");
}
```

## Advanced Enum Patterns

### Recursive Enums
```rust
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn new() -> Self {
        Nil
    }
    
    fn prepend(self, elem: i32) -> Self {
        Cons(elem, Box::new(self))
    }
    
    fn len(&self) -> usize {
        match self {
            Cons(_, tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    
    fn stringify(&self) -> String {
        match self {
            Cons(head, tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let list = List::new().prepend(1).prepend(2).prepend(3);
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
```

### State Machine with Enums
```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(self) -> Self {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }
    
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 25,
        }
    }
    
    fn can_go(&self) -> bool {
        matches!(self, TrafficLight::Green)
    }
}

struct Intersection {
    light: TrafficLight,
    timer: u32,
}

impl Intersection {
    fn new() -> Self {
        Intersection {
            light: TrafficLight::Red,
            timer: 30,
        }
    }
    
    fn tick(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        } else {
            self.light = self.light.next();
            self.timer = self.light.duration();
        }
    }
}
```

### Complex Data Modeling
```rust
#[derive(Debug, Clone)]
enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Object(std::collections::HashMap<String, Value>),
    Null,
}

impl Value {
    fn type_name(&self) -> &'static str {
        match self {
            Value::Integer(_) => "integer",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Null => "null",
        }
    }
    
    fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Integer(0) => false,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Object(o) => !o.is_empty(),
            _ => true,
        }
    }
    
    fn get_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
}

use std::collections::HashMap;

fn json_example() {
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Value::String("Alice".to_string()));
    obj.insert("age".to_string(), Value::Integer(30));
    obj.insert("is_admin".to_string(), Value::Boolean(true));
    
    let data = Value::Object(obj);
    println!("Data type: {}", data.type_name());
    println!("Is truthy: {}", data.is_truthy());
}
```

## Option<T> Patterns

### Safe Null Handling
```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    let result = divide(10.0, 3.0);
    
    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot divide by zero"),
    }
    
    // Using unwrap_or
    let result = divide(10.0, 0.0).unwrap_or(0.0);
    println!("Result with default: {}", result);
    
    // Using map
    let result = divide(10.0, 3.0)
        .map(|x| x * 2.0)
        .unwrap_or(0.0);
    println!("Doubled result: {}", result);
}
```

### Option Combinators
```rust
fn add_one(x: Option<i32>) -> Option<i32> {
    x.map(|n| n + 1)
}

fn multiply(x: Option<i32>, y: Option<i32>) -> Option<i32> {
    match (x, y) {
        (Some(a), Some(b)) => Some(a * b),
        _ => None,
    }
}

// Or using and_then
fn multiply_alt(x: Option<i32>, y: Option<i32>) -> Option<i32> {
    x.and_then(|a| y.map(|b| a * b))
}

fn chain_operations() {
    let result = Some(5)
        .map(|x| x * 2)        // Some(10)
        .and_then(|x| {        // Chain another operation
            if x > 5 {
                Some(x - 1)
            } else {
                None
            }
        })                     // Some(9)
        .or(Some(0));          // Keep Some(9)
    
    println!("Chained result: {:?}", result);
}
```

## Result<T, E> Patterns

### Error Handling Strategies
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Multiple error types
#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
    Custom(String),
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::Io(error)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(error: std::num::ParseIntError) -> Self {
        MyError::Parse(error)
    }
}

fn complex_operation() -> Result<i32, MyError> {
    let content = std::fs::read_to_string("number.txt")?;
    let number: i32 = content.trim().parse()?;
    
    if number < 0 {
        return Err(MyError::Custom("Number cannot be negative".to_string()));
    }
    
    Ok(number * 2)
}
```

### Result Combinators
```rust
fn process_number(input: &str) -> Result<i32, String> {
    input.trim()
        .parse::<i32>()
        .map_err(|_| "Invalid number format".to_string())
        .and_then(|n| {
            if n >= 0 {
                Ok(n)
            } else {
                Err("Number must be non-negative".to_string())
            }
        })
        .map(|n| n * 2)
}

fn main() {
    let inputs = ["42", "-5", "abc", "  10  "];
    
    for input in &inputs {
        match process_number(input) {
            Ok(result) => println!("'{}' -> {}", input, result),
            Err(e) => println!("'{}' -> Error: {}", input, e),
        }
    }
}
```

## Advanced Pattern Matching

### Pattern Guards
```rust
fn categorize_number(x: Option<i32>) -> String {
    match x {
        Some(n) if n < 0 => "Negative".to_string(),
        Some(n) if n == 0 => "Zero".to_string(),
        Some(n) if n > 0 && n <= 10 => "Small positive".to_string(),
        Some(n) if n > 10 => "Large positive".to_string(),
        None => "No value".to_string(),
        _ => unreachable!(),
    }
}

// Multiple patterns with guards
fn analyze_tuple(pair: (i32, i32)) -> String {
    match pair {
        (x, y) if x == y => "Equal".to_string(),
        (x, y) if x > y => "First is larger".to_string(),
        (x, y) if x < y => "Second is larger".to_string(),
        _ => unreachable!(),
    }
}
```

### Nested Destructuring
```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn process_message(msg: Message) {
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {}, green {}, blue {}", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {}, saturation {}, value {}", h, s, v);
        }
        Message::Move { x, y } => {
            println!("Move to coordinates ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::Quit => {
            println!("Quit message received");
        }
    }
}
```

### @ Bindings
```rust
enum Message {
    Hello { id: i32 },
    Goodbye { id: i32 },
}

fn process_message_with_binding(msg: Message) {
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range 3-7: {}", id_variable);
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range");
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id);
        }
        Message::Goodbye { id } => {
            println!("Goodbye with id: {}", id);
        }
    }
}
```

## Performance and Memory

### Enum Size Optimization
```rust
// Large enum - size determined by largest variant
enum LargeEnum {
    Small(u8),                    // 1 byte + discriminant
    Large([u8; 1000]),           // 1000 bytes + discriminant
}

// Better: Box large variants
enum OptimizedEnum {
    Small(u8),
    Large(Box<[u8; 1000]>),      // 8 bytes (pointer) + discriminant
}

// Use Option for nullable pointers
struct Node {
    value: i32,
    next: Option<Box<Node>>,     // Null pointer optimization
}
```

### Pattern Matching Performance
```rust
// Compiler optimizes match to jump table when possible
fn fast_match(x: u8) -> &'static str {
    match x {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    }
}

// Complex patterns may be less optimized
fn complex_match(data: &[i32]) -> String {
    match data {
        [] => "empty".to_string(),
        [x] => format!("single: {}", x),
        [x, y] => format!("pair: {}, {}", x, y),
        _ => format!("multiple: {} items", data.len()),
    }
}
```

## Testing Enums
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_traffic_light_transitions() {
        let light = TrafficLight::Red;
        let light = light.next();
        assert!(matches!(light, TrafficLight::Green));
        
        let light = light.next();
        assert!(matches!(light, TrafficLight::Yellow));
    }
    
    #[test]
    fn test_option_operations() {
        assert_eq!(add_one(Some(5)), Some(6));
        assert_eq!(add_one(None), None);
        
        assert_eq!(multiply(Some(3), Some(4)), Some(12));
        assert_eq!(multiply(Some(3), None), None);
    }
    
    #[test]
    fn test_result_operations() {
        assert!(process_number("42").is_ok());
        assert!(process_number("-5").is_err());
        assert!(process_number("abc").is_err());
    }
}
```

## Best Practices

### Enum Design
```rust
// ✅ Good: Descriptive variant names
enum HttpStatus {
    Ok,
    NotFound,
    InternalServerError,
}

// ❌ Bad: Ambiguous names
enum Status {
    A,
    B,
    C,
}

// ✅ Good: Use associated data meaningfully
enum Event {
    KeyPress(char),
    MouseMove { x: i32, y: i32 },
    MouseClick { button: MouseButton, x: i32, y: i32 },
}

// ✅ Good: Implement common traits
#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}
```

### Error Handling Best Practices
```rust
// ✅ Good: Specific error types
#[derive(Debug)]
enum ValidationError {
    TooShort { min_length: usize },
    TooLong { max_length: usize },
    InvalidCharacter { character: char },
}

// ✅ Good: Use Result for recoverable errors
fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.len() < 3 {
        return Err(ValidationError::TooShort { min_length: 3 });
    }
    if username.len() > 20 {
        return Err(ValidationError::TooLong { max_length: 20 });
    }
    Ok(())
}

// ✅ Good: Use panic! for unrecoverable errors
fn divide_by_constant(x: f64) -> f64 {
    const DIVISOR: f64 = 2.0;
    if DIVISOR == 0.0 {
        panic!("Division by zero in divide_by_constant");
    }
    x / DIVISOR
}
```

Official Chapter: https://doc.rust-lang.org/book/ch06-00-enums.html

---
*Completed: ✓*