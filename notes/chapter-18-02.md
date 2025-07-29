# Chapter 18.2: Refutability: Whether a Pattern Might Fail to Match

## Key Takeaways
- **Refutability**: Property that determines if a pattern can fail to match
- **Irrefutable Patterns**: Always match, cannot fail
- **Refutable Patterns**: Can fail to match, need handling
- **Context Matters**: Some contexts require irrefutable patterns
- **Compiler Enforcement**: Prevents misuse of refutable patterns

## Understanding Refutability

### Irrefutable Patterns
Patterns that will always match for any possible value passed.

```rust
fn main() {
    let x = 5;  // x is irrefutable - any value will match
    
    let (a, b) = (1, 2);  // tuple destructuring is irrefutable
    
    let Point { x, y } = Point { x: 0, y: 0 };  // struct destructuring is irrefutable
}

struct Point {
    x: i32,
    y: i32,
}
```

### Refutable Patterns
Patterns that can fail to match for some possible values.

```rust
fn main() {
    let some_option_value: Option<i32> = None;
    
    // This would fail to compile - Some(x) is refutable
    // let Some(x) = some_option_value;  // ❌ Compiler error
    
    // Correct usage with if let
    if let Some(x) = some_option_value {
        println!("{}", x);
    }
}
```

## Contexts Requiring Irrefutable Patterns

### let Statements
Must use irrefutable patterns because there's no alternative to handle failure.

```rust
fn main() {
    // ✅ Valid - irrefutable patterns
    let x = 5;
    let (a, b) = (1, 2);
    let Point { x, y } = Point { x: 1, y: 2 };
    
    // ❌ Invalid - refutable pattern in let
    // let Some(x) = Some(5);  // Compiler error
}

struct Point {
    x: i32,
    y: i32,
}
```

### Function Parameters
Must be irrefutable patterns.

```rust
// ✅ Valid - irrefutable parameter pattern
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

// ✅ Valid - irrefutable struct destructuring
fn print_point(Point { x, y }: Point) {
    println!("Point: ({}, {})", x, y);
}

// ❌ Invalid - refutable pattern in parameter
// fn handle_option(Some(x): Option<i32>) {  // Compiler error
//     println!("Value: {}", x);
// }

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
    
    let p = Point { x: 1, y: 2 };
    print_point(p);
}
```

### for Loop Variables
Must be irrefutable patterns.

```rust
fn main() {
    let v = vec![(1, 2), (3, 4), (5, 6)];
    
    // ✅ Valid - tuple destructuring is irrefutable
    for (x, y) in v {
        println!("({}, {})", x, y);
    }
    
    let points = vec![
        Point { x: 0, y: 1 },
        Point { x: 2, y: 3 },
    ];
    
    // ✅ Valid - struct destructuring is irrefutable
    for Point { x, y } in points {
        println!("Point: ({}, {})", x, y);
    }
}

struct Point {
    x: i32,
    y: i32,
}
```

## Contexts Accepting Refutable Patterns

### if let Expressions
Designed to handle refutable patterns with optional else branch.

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    
    // ✅ Valid - if let handles refutable patterns
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

### while let Loops
Handle refutable patterns, loop continues while pattern matches.

```rust
fn main() {
    let mut stack = Vec::new();
    
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    // ✅ Valid - while let handles refutable patterns
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    
    // When stack.pop() returns None, loop exits
}
```

### match Arms
Each arm can have refutable patterns, but together they must be exhaustive.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));
    
    // ✅ Valid - match arms can be refutable, but must be exhaustive
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {}, green {}, blue {}", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {}, saturation {}, value {}", h, s, v);
        }
        _ => (),  // Handle remaining cases
    }
}
```

## Compiler Error Examples

### Refutable Pattern in let
```rust
fn main() {
    let some_option_value: Option<i32> = None;
    
    // ❌ This causes a compile error:
    // let Some(x) = some_option_value;
    
    /*
    Error:
    refutable pattern in local binding: `None` not covered
    help: you might want to use `if let` to ignore the variant that isn't matched
    */
}
```

### Irrefutable Pattern in if let
```rust
fn main() {
    // ❌ This causes a warning (and is pointless):
    // if let x = 5 {
    //     println!("{}", x);
    // }
    
    /*
    Warning:
    irrefutable `if let` pattern
    help: consider using a `let` binding instead
    */
}
```

## Working with Refutability

### Converting Refutable to Irrefutable Context
```rust
fn main() {
    let some_option_value: Option<i32> = Some(5);
    
    // Instead of let Some(x) = some_option_value;
    // Use pattern matching to handle both cases
    match some_option_value {
        Some(x) => println!("Got a value: {}", x),
        None => println!("Got None"),
    }
    
    // Or use if let for conditional handling
    if let Some(x) = some_option_value {
        println!("Got a value: {}", x);
    }
    
    // Or unwrap if you're certain it's Some (unsafe in general)
    let x = some_option_value.unwrap();
    println!("Value: {}", x);
}
```

### Handling Multiple Refutable Patterns
```rust
fn main() {
    let config_max: Option<usize> = Some(3);
    let setting_value: Result<u8, _> = "4".parse();
    
    // Chain multiple refutable patterns
    if let Some(max) = config_max {
        if let Ok(value) = setting_value {
            if value <= max as u8 {
                println!("Value {} is within limit {}", value, max);
            } else {
                println!("Value {} exceeds limit {}", value, max);
            }
        } else {
            println!("Couldn't parse setting value");
        }
    } else {
        println!("No maximum configured");
    }
}
```

### Nested Pattern Refutability
```rust
enum Message {
    Hello { id: i32 },
    Goodbye,
}

fn main() {
    let messages = vec![
        Message::Hello { id: 1 },
        Message::Hello { id: 2 },
        Message::Goodbye,
    ];
    
    for msg in messages {
        // Each arm is refutable, but together they're exhaustive
        match msg {
            Message::Hello { id } if id > 1 => {
                println!("Hello with high id: {}", id);
            }
            Message::Hello { id } => {
                println!("Hello with id: {}", id);
            }
            Message::Goodbye => {
                println!("Goodbye!");
            }
        }
    }
}
```

## Practical Refutability Patterns

### Option Handling
```rust
fn process_config(config: Option<&str>) {
    // Refutable pattern - config might be None
    if let Some(config_str) = config {
        println!("Processing config: {}", config_str);
        
        // Further refutable pattern matching
        match config_str {
            "debug" => println!("Debug mode enabled"),
            "release" => println!("Release mode enabled"),
            _ => println!("Unknown config: {}", config_str),
        }
    } else {
        println!("No config provided, using defaults");
    }
}

fn main() {
    process_config(Some("debug"));
    process_config(None);
}
```

### Result Handling
```rust
use std::fs;

fn read_config() -> Result<String, std::io::Error> {
    fs::read_to_string("config.txt")
}

fn main() {
    // Refutable pattern matching with Result
    match read_config() {
        Ok(contents) => {
            println!("Config contents: {}", contents);
            
            // Nested refutable patterns
            if let Some(first_line) = contents.lines().next() {
                println!("First line: {}", first_line);
            }
        }
        Err(error) => {
            println!("Failed to read config: {}", error);
        }
    }
}
```

## Refutable vs Irrefutable Summary

### Irrefutable Patterns (Always Match)
- Variable bindings: `let x = 5;`
- Tuple destructuring: `let (x, y) = (1, 2);`
- Struct destructuring: `let Point { x, y } = point;`
- Function parameters: `fn foo(x: i32) {}`
- Array destructuring: `let [a, b, c] = [1, 2, 3];`

### Refutable Patterns (May Not Match)
- Option patterns: `Some(x)`, `None`
- Result patterns: `Ok(x)`, `Err(e)`
- Enum variant patterns: `Message::Quit`
- Range patterns: `1..=5`
- Guard patterns: `Some(x) if x > 5`
- Literal patterns: `42`, `"hello"`

## Best Practices
1. **Use Appropriate Context**: Match refutability requirements with context
2. **Handle All Cases**: Ensure exhaustive matching for refutable patterns
3. **Prefer if let**: Use `if let` for simple refutable pattern matching
4. **Use match for Complex Cases**: Use `match` for multiple refutable patterns
5. **Compiler Guidance**: Trust compiler errors and warnings about refutability

## Integration with Previous Concepts
- **Error Handling**: Essential for `Result` and `Option` types
- **Enums**: All enum variants create refutable patterns except single-variant enums
- **Ownership**: Refutability doesn't affect ownership semantics
- **Control Flow**: Enables safe, exhaustive control flow

Understanding refutability helps write more robust Rust code by ensuring patterns are used in appropriate contexts and all cases are properly handled.