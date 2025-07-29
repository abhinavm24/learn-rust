# Chapter 18.1: All the Places Patterns Can Be Used

## Key Takeaways
- **Pattern Matching**: Fundamental feature in Rust for destructuring and conditional logic
- **Match Arms**: Primary location for patterns in `match` expressions
- **Conditional if let**: Simplified pattern matching for single cases
- **while let**: Pattern matching in loops
- **Function Parameters**: Patterns can destructure function arguments
- **Variable Assignments**: Patterns in `let` statements for destructuring

## Match Arms

### Basic Match Expression
```rust
fn main() {
    let x = 1;
    
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
```

### Pattern Matching with Values
```rust
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
    // ... more states
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

### Pattern Matching with Option
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
    
    println!("{:?}, {:?}", six, none);  // Some(6), None
}
```

## Conditional if let Expressions

### Basic if let Usage
```rust
fn main() {
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
}
```

### if let with Complex Patterns
```rust
enum Message {
    Hello { id: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Hello { id: 5 };
    
    if let Message::Hello { id: id_variable @ 3..=7 } = msg {
        println!("Found an id in range: {}", id_variable);
    }
}
```

## while let Conditional Loops

### Basic while let Usage
```rust
fn main() {
    let mut stack = Vec::new();
    
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

### while let with Complex Patterns
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    
    let keys: Vec<_> = map.keys().cloned().collect();
    let mut iter = keys.into_iter();
    
    while let Some(key) = iter.next() {
        if let Some(value) = map.remove(key) {
            println!("Removed {}: {}", key, value);
        }
    }
}
```

## for Loops

### Destructuring in for Loops
```rust
fn main() {
    let v = vec!['a', 'b', 'c'];
    
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}
```

### Complex Destructuring in for Loops
```rust
fn main() {
    let points = vec![(0, 1), (2, 3), (4, 5)];
    
    for (x, y) in points {
        println!("Point: ({}, {})", x, y);
    }
}
```

### Nested Destructuring
```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let points = vec![
        Point { x: 0, y: 1 },
        Point { x: 2, y: 3 },
    ];
    
    for Point { x, y } in points {
        println!("Point: ({}, {})", x, y);
    }
}
```

## let Statements

### Basic Destructuring with let
```rust
fn main() {
    let point = (3, 5);
    let (x, y) = point;
    
    println!("x: {}, y: {}", x, y);
}
```

### Struct Destructuring
```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    
    // Shorthand when variable names match field names
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```

### Array and Slice Destructuring
```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let [first, second, ..] = arr;
    println!("First: {}, Second: {}", first, second);
    
    let [.., fourth, fifth] = arr;
    println!("Fourth: {}, Fifth: {}", fourth, fifth);
    
    let [first, .., last] = arr;
    println!("First: {}, Last: {}", first, last);
}
```

## Function Parameters

### Destructuring Function Parameters
```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

### Complex Parameter Destructuring
```rust
struct Point {
    x: i32,
    y: i32,
}

fn print_point(Point { x, y }: Point) {
    println!("Point coordinates: ({}, {})", x, y);
}

fn print_point_ref(&Point { x, y }: &Point) {
    println!("Point coordinates: ({}, {})", x, y);
}

fn main() {
    let p = Point { x: 5, y: 10 };
    print_point(p);
    
    let p2 = Point { x: 1, y: 2 };
    print_point_ref(&p2);
}
```

### Closure Parameter Patterns
```rust
fn main() {
    let points = vec![(0, 1), (2, 3), (4, 5)];
    
    let sum_of_squares: i32 = points
        .iter()
        .map(|(x, y)| x * x + y * y)
        .sum();
    
    println!("Sum of squares: {}", sum_of_squares);
}
```

## Nested Patterns

### Complex Nested Destructuring
```rust
struct Person {
    name: String,
    address: Address,
}

struct Address {
    street: String,
    city: String,
    country: String,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        address: Address {
            street: String::from("123 Main St"),
            city: String::from("Anytown"),
            country: String::from("USA"),
        },
    };
    
    let Person {
        name,
        address: Address { city, country, .. },
    } = person;
    
    println!("{} lives in {}, {}", name, city, country);
}
```

### Enum with Nested Patterns
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

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {}, green {}, blue {}", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {}, saturation {}, value {}", h, s, v);
        }
        _ => (),
    }
}
```

## Irrefutable vs Refutable Patterns

### Irrefutable Patterns (Always Match)
```rust
fn main() {
    // These patterns always match
    let x = 5;                    // Irrefutable
    let (a, b) = (1, 2);         // Irrefutable
    let Point { x, y } = Point { x: 0, y: 0 };  // Irrefutable
}
```

### Refutable Patterns (May Not Match)
```rust
fn main() {
    let some_option_value: Option<i32> = None;
    
    // This would cause a compile error because None doesn't match Some(x)
    // let Some(x) = some_option_value;  // ❌ Refutable pattern
    
    // Use if let for refutable patterns
    if let Some(x) = some_option_value {
        println!("{}", x);
    }
}
```

## Integration with Previous Concepts
- **Ownership**: Patterns can move, borrow, or copy values during destructuring
- **Borrowing**: Use `&` in patterns to borrow instead of move
- **Lifetimes**: Pattern matching preserves lifetime relationships
- **Error Handling**: Common with `Result` and `Option` types

## Best Practices
1. **Use Appropriate Pattern Location**: Choose the right construct for your use case
2. **Destructure Meaningfully**: Extract only the data you need
3. **Handle All Cases**: Ensure exhaustive matching when required
4. **Prefer Specific Patterns**: More specific patterns are more expressive
5. **Use `_` for Unused Values**: Clearly indicate intentionally ignored values

## Common Pattern Locations Summary
- **match expressions**: Primary pattern matching construct
- **if let expressions**: Single pattern matching with optional else
- **while let loops**: Pattern matching in iteration
- **for loops**: Destructuring in iteration
- **let statements**: Variable binding with destructuring
- **Function parameters**: Destructuring arguments
- **Closure parameters**: Destructuring in closures

Patterns provide a powerful and expressive way to work with data structures throughout Rust programs.