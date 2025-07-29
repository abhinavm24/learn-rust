# Chapter 18: Patterns and Matching

## Key Takeaways

### Core Concepts
- **Pattern Matching**: Destructuring data types to extract values and control program flow
- **Exhaustiveness Checking**: Compiler ensures all possible cases are handled in match expressions
- **Refutability**: Understanding when patterns can fail to match vs always matching
- **Destructuring**: Breaking apart complex data structures into their component parts
- **Pattern Guards**: Adding additional conditions to pattern matches for fine-grained control
- **Multiple Pattern Context**: Using patterns in various contexts beyond just match expressions

### Important Syntax and Operators
- `match value { patterns => expressions }` - Primary pattern matching construct
- `if let pattern = value` - Conditional pattern matching for single case
- `while let pattern = value` - Loop with pattern matching condition
- `let (a, b) = tuple` - Destructuring assignment in let statements
- `|` - Multiple pattern alternatives (OR patterns)
- `..` - Range patterns and ignoring remaining fields
- `_` - Wildcard pattern to ignore values
- `@` - Binding captured values while pattern matching

### Programming Concepts Introduced
- **Pattern-Based Control Flow**: Using data structure shapes to determine program behavior
- **Destructuring Assignment**: Extracting multiple values from complex types in single operation
- **Exhaustive Analysis**: Compiler-enforced completeness of pattern coverage
- **Guard Conditions**: Combining pattern matching with additional boolean logic
- **Variable Binding**: Capturing values during pattern matching process

## Code Examples and Patterns

### Basic Pattern Matching Fundamentals
```rust
fn describe_number(x: i32) {
    match x {
        // Literal patterns
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        
        // Multiple patterns with OR
        4 | 5 | 6 => println!("Four, five, or six"),
        
        // Range patterns
        7..=10 => println!("Seven through ten"),
        
        // Wildcard pattern
        _ => println!("Something else"),
    }
}

// Using patterns in different contexts
fn pattern_contexts() {
    let point = (3, 5);
    
    // Pattern in let statement (irrefutable)
    let (x, y) = point;
    println!("Point coordinates: ({}, {})", x, y);
    
    // Pattern in if let (refutable)
    let some_option_value = Some(5);
    if let Some(value) = some_option_value {
        println!("Got a value: {}", value);
    }
    
    // Pattern in while let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}
```

### Advanced Destructuring Patterns
```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn advanced_destructuring() {
    // Struct destructuring
    let p = Point { x: 0, y: 7 };
    
    // Complete destructuring
    let Point { x, y } = p;
    println!("Point at ({}, {})", x, y);
    
    // Partial destructuring with renaming
    let Point { x: a, y: b } = p;
    println!("Renamed coordinates: ({}, {})", a, b);
    
    // Partial destructuring with shorthand
    match p {
        Point { x: 0, y } => println!("On the y-axis at {}", y),
        Point { x, y: 0 } => println!("On the x-axis at {}", x),
        Point { x, y } => println!("At ({}, {})", x, y),
    }
    
    // Enum destructuring
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write("Hello".to_string()),
        Message::ChangeColor(255, 0, 0),
    ];
    
    for msg in messages {
        match msg {
            Message::Quit => println!("Quit message received"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change color to RGB({}, {}, {})", r, g, b)
            }
        }
    }
}
```

### Pattern Guards and @ Bindings
```rust
fn pattern_guards_and_bindings() {
    let num = Some(4);
    
    // Pattern guards with if conditions
    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) if x % 2 == 0 => println!("Even number: {}", x),
        Some(x) => println!("Other number: {}", x),
        None => println!("No number"),
    }
    
    // @ bindings - capture value while pattern matching
    enum MessageWithId {
        Hello { id: i32 },
        Goodbye { id: i32 },
    }
    
    let msg = MessageWithId::Hello { id: 5 };
    
    match msg {
        // Capture id value in id_variable while checking range
        MessageWithId::Hello { id: id_variable @ 3..=7 } => {
            println!("Found Hello with id in range 3-7: {}", id_variable)
        }
        MessageWithId::Hello { id: id_variable @ 10..=12 } => {
            println!("Found Hello with id in range 10-12: {}", id_variable)
        }
        MessageWithId::Hello { id } => {
            println!("Found Hello with other id: {}", id)
        }
        MessageWithId::Goodbye { id } => {
            println!("Found Goodbye with id: {}", id)
        }
    }
}
```

### Complex Nested Pattern Matching
```rust
fn nested_patterns() {
    let data = ((0, 1), (2, 3));
    
    // Nested destructuring
    let ((a, b), (c, d)) = data;
    println!("Nested values: {}, {}, {}, {}", a, b, c, d);
    
    // Pattern matching with nested structures
    match data {
        ((0, y), (x, 3)) => println!("Special case: y={}, x={}", y, x),
        ((a, b), (c, d)) if a + c > b + d => {
            println!("Sum of firsts > sum of seconds")
        }
        _ => println!("Other pattern"),
    }
    
    // Vector/slice patterns
    let vec = vec![1, 2, 3, 4, 5];
    
    match vec.as_slice() {
        [] => println!("Empty vector"),
        [single] => println!("Single element: {}", single),
        [first, second] => println!("Two elements: {}, {}", first, second),
        [first, .., last] => println!("First: {}, Last: {}", first, last),
        _ => println!("Other patterns"),
    }
}
```

### Function Parameter Patterns
```rust
// Patterns in function parameters
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

// Pattern matching in closures
fn closure_patterns() {
    let points = vec![(0, 1), (2, 3), (4, 5)];
    
    let sum_of_squares: i32 = points
        .iter()
        .map(|(x, y)| x * x + y * y)  // Pattern in closure parameter
        .sum();
    
    println!("Sum of squares: {}", sum_of_squares);
}
```

## Practical Applications
- Implementing finite state machines with clear state transitions
- Parsing structured data formats (JSON, XML, configuration files)
- Error handling with comprehensive case coverage
- Command-line argument processing with pattern-based routing
- Data validation and transformation pipelines
- Building domain-specific languages with pattern-based interpreters

## Integration with Previous Chapters
- **Prerequisites**: Enums (Chapter 6), structs (Chapter 5), ownership concepts (Chapter 4)
- **Builds On**: Option and Result types (Chapter 6), collections (Chapter 8) for complex destructuring
- **Connections**: Essential for error handling patterns, enables functional programming style

## Community Conventions and Idioms
- Always handle all enum variants explicitly rather than using catch-all patterns
- Use `if let` for simple single-case matches instead of full match expressions
- Prefer pattern matching over nested if-else statements for clarity
- Name captured variables descriptively, especially in complex patterns
- Use `_` prefix for unused variables in patterns to avoid warnings
- Group related patterns together and order from specific to general

## Personal Notes
- Pattern matching makes Rust code incredibly expressive and safe
- The exhaustiveness checker prevents many runtime errors by ensuring all cases are handled
- @ bindings are particularly useful when you need both the whole value and parts of it
- Pattern guards should be used sparingly - consider refactoring complex guards into separate functions
- Learning to think in patterns transforms how you approach data processing problems
- The combination of pattern matching and Rust's type system creates very robust code

Official Chapter: https://doc.rust-lang.org/book/ch18-00-patterns.html

---
*Completed: ✓*