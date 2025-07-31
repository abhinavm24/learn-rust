//! Chapter 9.1: Unrecoverable Errors with panic!
//! 
//! This example demonstrates the panic! macro and Rust's approach to handling
//! unrecoverable errors. When something goes seriously wrong and the program
//! cannot continue safely, Rust provides the panic! macro to immediately
//! terminate execution and provide debugging information.
//!
//! Key concepts:
//! - When and why to use panic!
//! - Stack unwinding vs aborting
//! - Panic hooks and backtraces
//! - Catching panics with catch_unwind
//! - Using panics in testing

use std::panic;
use rust_book_examples::print_chapter_header;

#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    email: String,
}

impl User {
    fn new(name: String, age: i32, email: String) -> Self {
        // Validate input and panic on invalid data
        if name.is_empty() {
            panic!("User name cannot be empty");
        }
        if age < 0 || age > 150 {
            panic!("Invalid age: {}. Age must be between 0 and 150.", age);
        }
        if !email.contains('@') {
            panic!("Invalid email format: {}", email);
        }
        
        User { name, age, email }
    }
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("Cannot divide {} by zero!", a);
    }
    a / b
}

fn validate_age(age: i32) -> i32 {
    if age < 0 || age > 150 {
        panic!("Invalid age: {}. Age must be between 0 and 150.", age);
    }
    age
}

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

fn process_user(user: User) {
    if user.name.is_empty() {
        panic!("User has empty name: {:?}", user);
    }
    
    if user.age < 0 {
        panic!("User has invalid age: {:?}", user);
    }
    
    println!("Processing user: {:?}", user);
}

fn risky_operation(should_panic: bool) -> i32 {
    if should_panic {
        panic!("Something went wrong!");
    }
    42
}

fn deep_function() {
    even_deeper_function();
}

fn even_deeper_function() {
    panic!("Deep panic for backtrace demonstration");
}

fn demonstrate_basic_panic() {
    println!("\n=== Basic Panic Usage ===");
    
    println!("About to demonstrate a panic...");
    println!("Note: This would normally terminate the program!");
    
    // We'll use catch_unwind to prevent actual program termination
    let result = panic::catch_unwind(|| {
        println!("Hello, world!");
        panic!("crash and burn");
        println!("This will never be printed");
    });
    
    match result {
        Ok(_) => println!("Operation completed successfully"),
        Err(_) => println!("Panic was caught: 'crash and burn'"),
    }
}

fn demonstrate_panic_with_data() {
    println!("\n=== Panic with Formatted Messages ===");
    
    let test_cases = vec![
        (10.0, 2.0),
        (15.0, 3.0),
        (20.0, 0.0), // This will panic
    ];
    
    for (a, b) in test_cases {
        let result = panic::catch_unwind(|| divide(a, b));
        match result {
            Ok(result) => println!("{} / {} = {}", a, b, result),
            Err(_) => println!("Panic caught: Cannot divide {} by zero!", a),
        }
    }
}

fn demonstrate_out_of_bounds() {
    println!("\n=== Array Out-of-Bounds Panic ===");
    
    let v = vec![1, 2, 3];
    println!("Vector: {:?}", v);
    
    // Safe access
    println!("Element at index 1: {}", v[1]);
    
    // Demonstrate out-of-bounds access
    let result = panic::catch_unwind(|| {
        let _value = v[99]; // This will panic
    });
    
    match result {
        Ok(_) => println!("Access successful"),
        Err(_) => println!("Panic caught: Index 99 out of bounds for vector of length {}", v.len()),
    }
}

fn demonstrate_assertions() {
    println!("\n=== Using Assertions ===");
    
    let x = 5;
    let y = 10;
    
    // These assertions will pass
    println!("Testing assertions with x={}, y={}:", x, y);
    
    let result = panic::catch_unwind(|| {
        assert!(x < y);
        println!("✓ assert!(x < y) passed");
        
        assert_eq!(x + 5, y);
        println!("✓ assert_eq!(x + 5, y) passed");
        
        assert_ne!(x, y);
        println!("✓ assert_ne!(x, y) passed");
    });
    
    match result {
        Ok(_) => println!("All assertions passed"),
        Err(_) => println!("An assertion failed"),
    }
    
    // Demonstrate a failing assertion
    let result = panic::catch_unwind(|| {
        assert!(x > y, "x should be greater than y, but x={} and y={}", x, y);
    });
    
    match result {
        Ok(_) => println!("Assertion passed"),
        Err(_) => println!("Assertion failed: x should be greater than y"),
    }
}

fn demonstrate_conditional_panic() {
    println!("\n=== Conditional Panic Based on Logic ===");
    
    let mut account_balance = 100;
    println!("Starting balance: {}", account_balance);
    
    // Successful withdrawal
    let result = panic::catch_unwind(|| {
        let mut balance = account_balance;
        withdraw(&mut balance, 50);
        balance
    });
    
    match result {
        Ok(new_balance) => {
            account_balance = new_balance;
            println!("Withdrawal successful");
        },
        Err(_) => println!("Withdrawal failed"),
    }
    
    // Withdrawal that would cause panic (insufficient funds)
    let result = panic::catch_unwind(|| {
        let mut balance = account_balance;
        withdraw(&mut balance, 60);
        balance
    });
    
    match result {
        Ok(_) => println!("Withdrawal successful"),
        Err(_) => println!("Panic caught: Insufficient funds"),
    }
}

fn demonstrate_different_contexts() {
    println!("\n=== Panic in Different Contexts ===");
    
    // Panic in validation function
    let ages = vec![25, -5, 30, 200];
    
    println!("Validating ages: {:?}", ages);
    for age in ages {
        let result = panic::catch_unwind(|| validate_age(age));
        match result {
            Ok(valid_age) => println!("Valid age: {}", valid_age),
            Err(_) => println!("Invalid age caught: {}", age),
        }
    }
    
    // Panic in match expression
    let grades = vec!['A', 'B', 'Z', 'C'];
    
    println!("Processing grades: {:?}", grades);
    for grade in grades {
        let result = panic::catch_unwind(|| process_grade(grade));
        match result {
            Ok(description) => println!("Grade {}: {}", grade, description),
            Err(_) => println!("Invalid grade caught: {}", grade),
        }
    }
}

fn demonstrate_stack_unwinding() {
    println!("\n=== Stack Unwinding Demonstration ===");
    
    println!("Demonstrating function call chain and panic propagation:");
    
    let result = panic::catch_unwind(|| {
        println!("Starting main");
        function_a();
        println!("Back in main (won't print)");
    });
    
    match result {
        Ok(_) => println!("All functions completed successfully"),
        Err(_) => println!("Panic was caught and unwound through the stack"),
    }
    
    println!("Program continues after catching the panic");
}

fn demonstrate_panic_with_debug_info() {
    println!("\n=== Panic with Debug Information ===");
    
    let users = vec![
        User {
            name: "Alice".to_string(),
            age: 30,
            email: "alice@example.com".to_string(),
        },
        User {
            name: String::new(), // This will cause panic
            age: 25,
            email: "empty@example.com".to_string(),
        },
    ];
    
    for (i, user) in users.into_iter().enumerate() {
        let result = panic::catch_unwind(|| process_user(user));
        match result {
            Ok(_) => println!("User {} processed successfully", i),
            Err(_) => println!("User {} processing failed (empty name)", i),
        }
    }
}

fn demonstrate_custom_panic_hook() {
    println!("\n=== Custom Panic Hook ===");
    
    // Save the original panic hook
    let original_hook = panic::take_hook();
    
    // Set a custom panic hook
    panic::set_hook(Box::new(|panic_info| {
        println!("🚨 Custom panic handler activated!");
        
        if let Some(location) = panic_info.location() {
            println!("📍 Panic location: {}:{}:{}", 
                     location.file(), 
                     location.line(), 
                     location.column());
        }
        
        if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
            println!("💬 Panic message: {}", message);
        }
        
        println!("🔧 This is where you might log the error or send notifications");
    }));
    
    // Trigger a panic to show the custom hook
    let result = panic::catch_unwind(|| {
        panic!("This is a custom panic for demonstration!");
    });
    
    match result {
        Ok(_) => println!("No panic occurred"),
        Err(_) => println!("Panic was handled by custom hook"),
    }
    
    // Restore the original panic hook
    panic::set_hook(original_hook);
    println!("Original panic hook restored");
}

fn demonstrate_catching_panics() {
    println!("\n=== Catching Panics with catch_unwind ===");
    
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
    
    // This works with operations that don't panic too
    let result2 = panic::catch_unwind(|| {
        risky_operation(false)
    });
    
    match result2 {
        Ok(value) => println!("Second operation succeeded: {}", value),
        Err(_) => println!("Second operation panicked"),
    }
    
    // Demonstrate with closures that capture environment
    let multiplier = 3;
    let result3 = panic::catch_unwind(|| {
        let value = risky_operation(false);
        value * multiplier
    });
    
    match result3 {
        Ok(value) => println!("Calculated result: {}", value),
        Err(_) => println!("Calculation panicked"),
    }
}

fn demonstrate_when_to_panic() {
    println!("\n=== When to Use panic! vs When Not To ===");
    
    println!("✅ Good uses of panic!:");
    println!("  • Programming errors that should never happen");
    println!("  • Invalid states that indicate bugs");
    println!("  • Prototype code and examples");
    println!("  • Test assertions");
    println!("  • When continuing would be unsafe");
    
    println!("\n❌ Avoid panic! for:");
    println!("  • Expected errors (file not found, network issues)");
    println!("  • User input validation in production");
    println!("  • Library code where callers should handle errors");
    println!("  • Recoverable conditions");
    
    // Example of good panic usage
    println!("\nExample of appropriate panic usage:");
    let result = panic::catch_unwind(|| {
        let arr = [1, 2, 3, 4, 5];
        let index = 10; // This is clearly a programming error
        if index >= arr.len() {
            panic!("Programming error: index {} out of bounds for array of length {}", 
                   index, arr.len());
        }
        arr[index]
    });
    
    match result {
        Ok(value) => println!("Array access successful: {}", value),
        Err(_) => println!("Caught programming error: index out of bounds"),
    }
}

fn demonstrate_backtrace_info() {
    println!("\n=== Backtrace Information ===");
    
    println!("In a real program, you would run with:");
    println!("RUST_BACKTRACE=1 cargo run --example ch09_01_unrecoverable_errors_panic");
    println!("to see detailed backtraces when panics occur.");
    
    println!("\nDemonstrating deep function call stack:");
    let result = panic::catch_unwind(|| {
        deep_function();
    });
    
    match result {
        Ok(_) => println!("Deep function completed successfully"),
        Err(_) => {
            println!("Panic caught from deep in the call stack");
            println!("With RUST_BACKTRACE=1, you would see:");
            println!("  - The panic message");
            println!("  - Function call chain leading to the panic");
            println!("  - File names and line numbers");
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
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
        divide(1.0, 0.0);
    }
    
    #[test]
    #[should_panic(expected = "Invalid grade")]
    fn test_invalid_grade_panics() {
        process_grade('Z');
    }
}

fn main() {
    print_chapter_header("Chapter 9.1", "Unrecoverable Errors with panic!");

    println!("The panic! macro is used for unrecoverable errors when the program");
    println!("cannot continue safely. This example demonstrates various panic scenarios");
    println!("using catch_unwind to prevent actual program termination.");

    demonstrate_basic_panic();
    demonstrate_panic_with_data();
    demonstrate_out_of_bounds();
    demonstrate_assertions();
    demonstrate_conditional_panic();
    demonstrate_different_contexts();
    demonstrate_stack_unwinding();
    demonstrate_panic_with_debug_info();
    demonstrate_custom_panic_hook();
    demonstrate_catching_panics();
    demonstrate_when_to_panic();
    demonstrate_backtrace_info();

    println!("\n=== Key Takeaways ===");
    println!("• panic! is for unrecoverable errors and programming bugs");
    println!("• Use descriptive panic messages with relevant data");
    println!("• Stack unwinding cleans up memory before termination");
    println!("• RUST_BACKTRACE=1 provides debugging information");
    println!("• catch_unwind can recover from panics in some cases");
    println!("• Prefer Result<T, E> for recoverable errors");
    println!("• panic! prevents undefined behavior and memory corruption");
    
    println!("\n🔧 Run this example with RUST_BACKTRACE=1 to see detailed backtraces!");
    println!("📖 Run tests with: cargo test --example ch09_01_unrecoverable_errors_panic");
}