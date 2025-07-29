# Chapter 9.1: Unrecoverable Errors with panic!

## Key Takeaways

### Panic Fundamentals
- **panic!**: Macro for unrecoverable errors when program cannot continue safely
- **Immediate Termination**: Stops program execution when called
- **Safety Mechanism**: Prevents undefined behavior and memory corruption
- **Two Trigger Methods**: Explicit `panic!` calls or conditions that cause panics

### Panic Behavior
- **Default**: Print error message, unwind stack, clean up memory, and exit
- **Stack Unwinding**: Walks back up the stack and cleans up data from functions
- **Abort Option**: Can be configured to abort immediately without cleanup
- **Protection**: Prevents continuing execution in undefined states

### When Panics Occur
- **Explicit Calls**: Using `panic!` macro directly
- **Runtime Errors**: Array out-of-bounds access, division by zero, etc.
- **Assertion Failures**: `assert!`, `assert_eq!`, `assert_ne!` macros
- **Library Functions**: Some standard library functions panic on invalid input

### Important Syntax and Operators

#### Explicit Panic
```rust
panic!("error message");
panic!("error with data: {}", value);
```

#### Panic Configuration
```toml
# In Cargo.toml
[profile.release]
panic = 'abort'
```

#### Environment Variables
```bash
RUST_BACKTRACE=1    # Enable backtrace
RUST_BACKTRACE=full # Enable full backtrace
```

### Programming Concepts Introduced
- **Fail-Fast Philosophy**: Stop immediately when errors occur
- **Stack Unwinding**: Systematic cleanup during error propagation
- **Debugging Information**: Backtraces for error investigation
- **Program Termination**: Controlled shutdown vs immediate abort

### Code Examples and Patterns

#### Basic Panic Usage
```rust
fn main() {
    println!("Hello, world!");
    
    panic!("crash and burn");
    
    println!("This will never be printed");
}

// Output:
// Hello, world!
// thread 'main' panicked at 'crash and burn', src/main.rs:4:5
```

#### Panic with Formatted Messages
```rust
fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("Cannot divide {} by zero!", a);
    }
    a / b
}

fn main() {
    let result = divide(10.0, 0.0);
    println!("Result: {}", result); // This won't execute
}
```

#### Array Out-of-Bounds Panic
```rust
fn main() {
    let v = vec![1, 2, 3];
    
    // This will panic at runtime
    v[99]; // Index out of bounds
}

// Output:
// thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99'
```

#### Using Assertions
```rust
fn main() {
    let x = 5;
    let y = 10;
    
    // These will panic if the condition is false
    assert!(x < y);
    assert_eq!(x + 5, y);
    assert_ne!(x, y);
    
    // Custom panic message
    assert!(x > y, "x should be greater than y, but x={} and y={}", x, y);
}
```

#### Conditional Panic Based on Logic
```rust
fn withdraw(balance: &mut i32, amount: i32) {
    if amount > *balance {
        panic!("Insufficient funds! Balance: {}, Withdrawal: {}", balance, amount);
    }
    
    if amount <= 0 {
        panic!("Invalid withdrawal amount: {}", amount);
    }
    
    *balance -= amount;
    println!("Withdrew {}. New balance: {}", amount, balance);
}

fn main() {
    let mut account_balance = 100;
    
    withdraw(&mut account_balance, 50);  // OK
    withdraw(&mut account_balance, 60);  // Will panic - insufficient funds
}
```

#### Panic in Different Contexts
```rust
// Panic in function
fn validate_age(age: i32) -> i32 {
    if age < 0 || age > 150 {
        panic!("Invalid age: {}. Age must be between 0 and 150.", age);
    }
    age
}

// Panic in match expression
fn process_grade(grade: char) -> &'static str {
    match grade {
        'A' => "Excellent",
        'B' => "Good", 
        'C' => "Average",
        'D' => "Below Average",
        'F' => "Fail",
        _ => panic!("Invalid grade: {}", grade),
    }
}

fn main() {
    let age = validate_age(25);     // OK
    let grade_desc = process_grade('A'); // OK
    
    // These will panic:
    // validate_age(-5);
    // process_grade('Z');
}
```

#### Panic vs Error Handling
```rust
use std::fs::File;

fn main() {
    // Method 1: Let it panic
    let f = File::open("hello.txt").unwrap(); // Panics if file doesn't exist
    
    // Method 2: Handle the error gracefully
    match File::open("hello.txt") {
        Ok(file) => println!("File opened successfully"),
        Err(error) => println!("Failed to open file: {}", error),
    }
    
    // Method 3: Panic with custom message
    let f = File::open("hello.txt")
        .expect("Failed to open hello.txt - file should exist");
}
```

#### Stack Unwinding Demonstration
```rust
fn function_c() {
    println!("In function C");
    panic!("Something went wrong in C!");
}

fn function_b() {
    println!("In function B");
    function_c();
    println!("Back in function B (won't print)");
}

fn function_a() {
    println!("In function A");
    function_b();
    println!("Back in function A (won't print)");
}

fn main() {
    println!("Starting main");
    function_a();
    println!("Back in main (won't print)");
}

// Output:
// Starting main
// In function A
// In function B  
// In function C
// thread 'main' panicked at 'Something went wrong in C!'
```

#### Panic with Debug Information
```rust
#[derive(Debug)]
struct User {
    name: String,
    age: i32,
}

fn process_user(user: User) {
    if user.name.is_empty() {
        panic!("User has empty name: {:?}", user);
    }
    
    if user.age < 0 {
        panic!("User has invalid age: {:?}", user);
    }
    
    println!("Processing user: {:?}", user);
}

fn main() {
    let user1 = User {
        name: "Alice".to_string(),
        age: 30,
    };
    
    let user2 = User {
        name: String::new(),
        age: 25,
    };
    
    process_user(user1); // OK
    process_user(user2); // Will panic with debug info
}
```

#### Cargo.toml Panic Configuration
```toml
[package]
name = "panic_example"
version = "0.1.0"
edition = "2021"

# Panic configuration for different profiles
[profile.dev]
panic = "unwind"  # Default: unwind stack and clean up

[profile.release]
panic = "abort"   # Abort immediately, smaller binary size
```

#### Using Environment Variables for Debugging
```rust
// Run with: RUST_BACKTRACE=1 cargo run
fn deep_function() {
    even_deeper_function();
}

fn even_deeper_function() {
    panic!("Deep panic for backtrace demonstration");
}

fn main() {
    println!("This program will demonstrate backtraces");
    deep_function();
}

// With RUST_BACKTRACE=1, you'll see:
// - The panic message
// - Stack trace showing function call chain
// - File names and line numbers
```

#### Custom Panic Hook
```rust
use std::panic;

fn main() {
    // Set a custom panic hook
    panic::set_hook(Box::new(|panic_info| {
        println!("Custom panic handler!");
        println!("Panic occurred: {}", panic_info);
        
        if let Some(location) = panic_info.location() {
            println!("Panic location: {}:{}:{}", 
                     location.file(), 
                     location.line(), 
                     location.column());
        }
        
        if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
            println!("Panic message: {}", message);
        }
    }));
    
    // This will use our custom panic handler
    panic!("This is a custom panic!");
}
```

#### Catching Panics with std::panic::catch_unwind
```rust
use std::panic;

fn risky_operation(should_panic: bool) -> i32 {
    if should_panic {
        panic!("Something went wrong!");
    }
    42
}

fn main() {
    // Attempt to catch a panic
    let result = panic::catch_unwind(|| {
        risky_operation(true)
    });
    
    match result {
        Ok(value) => println!("Operation succeeded: {}", value),
        Err(_) => println!("Operation panicked and was caught"),
    }
    
    // Normal execution continues
    println!("Program continues running");
    
    // This works with closures that don't panic too
    let result2 = panic::catch_unwind(|| {
        risky_operation(false)
    });
    
    match result2 {
        Ok(value) => println!("Second operation succeeded: {}", value),
        Err(_) => println!("Second operation panicked"),
    }
}
```

#### Panic in Tests
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_normal_case() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    #[should_panic]
    fn test_panic_expected() {
        panic!("This test expects a panic");
    }
    
    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_specific_panic() {
        let x = 1;
        let y = 0;
        let _result = x / y; // This should panic with "division by zero"
    }
}

// Run with: cargo test
```

### Practical Applications
- Input validation with strict requirements
- Defensive programming for impossible states
- Development and debugging aids
- Test assertions and invariant checking
- Library API contract enforcement

### When to Use panic!
- **Programming Errors**: Bugs that should never happen in correct code
- **Invariant Violations**: When program state becomes invalid
- **Unrecoverable Conditions**: When continuing would be unsafe
- **Development/Testing**: For debugging and test assertions
- **Library Boundaries**: When caller violates API contracts

### When NOT to Use panic!
- **Expected Errors**: File not found, network errors, user input errors
- **Recoverable Conditions**: When the program can handle the error gracefully
- **User-Facing Applications**: Where crashes create poor user experience
- **Library Code**: Where callers should decide how to handle errors

### Panic vs Result
```rust
// Use panic! for programming errors
fn get_element(arr: &[i32], index: usize) -> i32 {
    if index >= arr.len() {
        panic!("Index {} out of bounds for array of length {}", index, arr.len());
    }
    arr[index]
}

// Use Result for expected errors  
fn safe_get_element(arr: &[i32], index: usize) -> Result<i32, String> {
    if index >= arr.len() {
        Err(format!("Index {} out of bounds", index))
    } else {
        Ok(arr[index])
    }
}
```

### Integration with Previous Chapters
- Uses string formatting from Chapter 8.2
- Applies to vector and HashMap operations from Chapter 8
- Builds foundation for Result type in next chapters
- Demonstrates Rust's safety-first approach to error handling

### Community Conventions and Idioms
- Use descriptive panic messages with relevant data
- Prefer `expect()` over `unwrap()` for better error messages
- Use `debug_assert!` for performance-sensitive code
- Document panic conditions in function documentation
- Consider `Result` types for recoverable errors

### Performance Considerations
- **Unwinding**: Has overhead for cleanup during stack unwinding
- **Abort**: Faster termination but no cleanup
- **Debug Assertions**: Zero cost in release builds
- **Release Optimization**: Panic paths are optimized as cold paths

### Personal Notes
- panic! is a last resort for unrecoverable errors
- Good panic messages save significant debugging time
- Backtraces are invaluable for understanding panic chains
- Understanding when to panic vs return errors is crucial
- Rust's panic system prevents undefined behavior common in other languages

Official Chapter: https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html

---
*Completed: ✓*