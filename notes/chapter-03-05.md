# Chapter 3.5: Control Flow

## Key Takeaways

### If Expressions
- Use `if` for conditional branching based on boolean expressions
- Conditions must be `bool` type (no automatic conversion from other types)
- `if` is an expression - can return values for variable assignment
- Support `else if` for multiple conditions and `else` for default case
- All branches must return same type when used as expression

### Loop Types
- **`loop`**: Infinite loop with explicit `break` to exit
- **`while`**: Conditional loop that runs while condition is true
- **`for`**: Iterator-based loop for collections and ranges (most common)

### Loop Control
- `break` exits current loop (can return values from `loop`)
- `continue` skips to next iteration
- Loop labels (`'label:`) for controlling nested loops
- Loops can return values using `break value`

### Important Syntax and Operators

#### Conditional Syntax
- `if condition { }` - basic conditional
- `else if condition { }` - additional conditions
- `else { }` - default case
- No parentheses required around conditions

#### Loop Syntax
- `loop { }` - infinite loop
- `while condition { }` - conditional loop
- `for item in collection { }` - iterator loop
- `break` - exit loop keyword
- `continue` - skip iteration keyword
- `'label:` - loop label prefix

#### Range Syntax
- `1..4` - range from 1 to 3 (exclusive end)
- `1..=4` - range from 1 to 4 (inclusive end)
- `.rev()` - reverse range/iterator method

### Programming Concepts Introduced
- **Conditional Execution**: Branching program flow based on conditions
- **Iteration**: Repeating code blocks with different control mechanisms
- **Expression-based Conditionals**: Using `if` as value-producing expression
- **Iterator Pattern**: Safe, efficient collection traversal
- **Loop Labels**: Managing complex nested loop structures
- **Early Exit**: Using `break` and `continue` for flow control

### Code Examples and Patterns

#### Basic If Expressions
```rust
fn main() {
    let number = 3;
    
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    
    // Multiple conditions
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }
}
```

#### If as Expression
```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
    
    // All branches must return same type
    // let number = if condition { 5 } else { "six" }; // Error!
}
```

#### Loop - Infinite Loop
```rust
fn main() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2; // Return value from loop
        }
    };
    
    println!("The result is {result}"); // 20
}
```

#### While Loop
```rust
fn main() {
    let mut number = 3;
    
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    
    println!("LIFTOFF!!!");
}
```

#### For Loop with Collections
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    
    // Safe iteration over array
    for element in a {
        println!("the value is: {element}");
    }
    
    // Range-based loop
    for number in 1..4 {
        println!("{number}!");
    }
    
    // Reverse range
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
```

#### Loop Labels and Nested Loops
```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // Break outer loop
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    println!("End count = {count}");
}
```

#### Break and Continue
```rust
fn main() {
    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // Skip even numbers
        }
        if i > 7 {
            break; // Stop at 7
        }
        println!("{i}");
    }
    // Prints: 1, 3, 5, 7
}
```

### Practical Applications
- User input validation loops
- Menu systems with conditional branching
- Data processing with iterator patterns
- Game loops and event handling
- Configuration-based program behavior
- Error handling with retry logic
- Collection processing and transformation

### Integration with Previous Chapters
- Uses boolean types from Chapter 3.2 for conditions
- Combines with functions from Chapter 3.3 for structured control
- Works with variables and mutability from Chapter 3.1
- Extends guessing game patterns from Chapter 2
- Prepares for complex data structure iteration in later chapters

### Community Conventions and Idioms
- Prefer `for` loops over `while` with manual indexing
- Use `loop` for infinite loops rather than `while true`
- Use ranges (`1..10`) instead of manual counter variables
- Prefer iterator methods over manual loop indexing
- Use early returns and `continue` to reduce nesting
- Use descriptive loop labels for complex nested structures
- Favor `if let` patterns for simple option handling (covered later)

### Personal Notes
- Expression-based `if` statements feel more functional than traditional languages
- `for` loops with ranges are much safer than C-style index loops
- Loop labels provide elegant solution to complex nested loop control
- The type consistency requirement for `if` expressions enforces clarity
- Rust's approach prevents many common loop-related bugs through type safety

Official Chapter: https://doc.rust-lang.org/book/ch03-05-control-flow.html

---
*Completed: ✓*