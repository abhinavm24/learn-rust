//! # Chapter 9.2: Recoverable Errors with Result
//! 
//! This example demonstrates:
//! - The Result<T, E> enum for handling recoverable errors
//! - Matching on Result to handle Ok and Err cases
//! - Shortcuts for panic on error: unwrap and expect
//! - Propagating errors with the ? operator
//! - Where to use Result vs panic!

use rust_book_examples::print_chapter_header;
use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    print_chapter_header("Chapter 9.2", "Recoverable Errors with Result");

    println!("=== Basic Result Handling ===");
    basic_result_handling();
    
    println!("\n=== Matching Different Error Types ===");
    matching_error_types();
    
    println!("\n=== Shortcuts: unwrap and expect ===");
    unwrap_and_expect_examples();
    
    println!("\n=== Propagating Errors ===");
    error_propagation_examples();
    
    println!("\n=== Custom Error Types ===");
    custom_error_examples();
    
    println!("\n=== Best Practices ===");
    best_practices_examples();
}

fn basic_result_handling() {
    // Result is an enum: Result<T, E> where T is success type, E is error type
    let result = File::open("hello.txt");
    
    match result {
        Ok(file) => {
            println!("File opened successfully: {:?}", file);
        },
        Err(error) => {
            println!("Failed to open file: {:?}", error);
        }
    }
    
    // Using if let for simpler error handling
    if let Err(error) = File::open("nonexistent.txt") {
        println!("Expected error for nonexistent file: {:?}", error);
    }
}

fn matching_error_types() {
    let result = File::open("hello.txt");
    
    match result {
        Ok(file) => println!("File opened: {:?}", file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found, attempting to create...");
                match File::create("hello.txt") {
                    Ok(fc) => println!("File created: {:?}", fc),
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                }
            },
            ErrorKind::PermissionDenied => {
                println!("Permission denied when trying to open file");
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    }
}

fn unwrap_and_expect_examples() {
    println!("=== unwrap and expect shortcuts ===");
    
    // unwrap(): panics if Result is Err, returns value if Ok
    // Only use when you're sure it won't fail or in examples/prototypes
    
    // This is safe because we're creating a valid result
    let success_result: Result<i32, &str> = Ok(42);
    let value = success_result.unwrap();
    println!("Unwrapped value: {}", value);
    
    // expect() is like unwrap() but lets you specify the panic message
    let another_success: Result<String, &str> = Ok(String::from("Success!"));
    let message = another_success.expect("This should never fail");
    println!("Expected value: {}", message);
    
    // Demonstrating why unwrap/expect should be used carefully
    println!("Note: In real code, avoid unwrap/expect unless you're certain of success");
    
    // Better alternatives:
    let risky_operation: Result<i32, &str> = Err("Something went wrong");
    
    // Using unwrap_or for providing default values
    let safe_value = risky_operation.unwrap_or(0);
    println!("Safe default value: {}", safe_value);
    
    // Using unwrap_or_else for computed defaults
    let computed_default = risky_operation.unwrap_or_else(|err| {
        println!("Error occurred: {}, using default", err);
        -1
    });
    println!("Computed default: {}", computed_default);
}

fn error_propagation_examples() {
    println!("=== Error Propagation Patterns ===");
    
    // Old way: manual error propagation
    match read_username_from_file_v1() {
        Ok(username) => println!("Username (v1): {}", username),
        Err(e) => println!("Error reading username (v1): {:?}", e),
    }
    
    // New way: using ? operator
    match read_username_from_file_v2() {
        Ok(username) => println!("Username (v2): {}", username),
        Err(e) => println!("Error reading username (v2): {:?}", e),
    }
    
    // Even more concise
    match read_username_from_file_v3() {
        Ok(username) => println!("Username (v3): {}", username),
        Err(e) => println!("Error reading username (v3): {:?}", e),
    }
    
    // Using ? with Option
    match get_last_char_of_first_line("hello.txt") {
        Some(ch) => println!("Last character: {}", ch),
        None => println!("No last character found"),
    }
}

// Manual error propagation (verbose)
fn read_username_from_file_v1() -> Result<String, io::Error> {
    let f = File::open("username.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Using ? operator for error propagation
fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Even more concise with chaining
fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// Using ? with Option
fn get_last_char_of_first_line(filename: &str) -> Option<char> {
    let mut s = String::new();
    File::open(filename).ok()?.read_to_string(&mut s).ok()?;
    s.lines().next()?.chars().last()
}

fn custom_error_examples() {
    // Custom error type example
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NegativeSquareRoot,
    }
    
    fn divide(a: f64, b: f64) -> Result<f64, MathError> {
        if b == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
    
    fn square_root(x: f64) -> Result<f64, MathError> {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }
    
    // Using our custom error types
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Division error: {:?}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("This shouldn't print: {}", result),
        Err(e) => println!("Expected division error: {:?}", e),
    }
    
    match square_root(-4.0) {
        Ok(result) => println!("This shouldn't print: {}", result),
        Err(e) => println!("Expected square root error: {:?}", e),
    }
}

fn best_practices_examples() {
    println!("=== Result Best Practices ===");
    
    // 1. Use Result for recoverable errors
    fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.parse()
    }
    
    match parse_number("42") {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Parse error: {}", e),
    }
    
    // 2. Chain operations with and_then
    let result = parse_number("42")
        .and_then(|n| {
            if n > 0 {
                Ok(n * 2)
            } else {
                Err("Number must be positive".parse::<i32>().unwrap_err())
            }
        });
    
    match result {
        Ok(n) => println!("Doubled positive number: {}", n),
        Err(e) => println!("Error in chain: {}", e),
    }
    
    // 3. Map operations for transforming success values
    let mapped_result = parse_number("123")
        .map(|n| n + 1)
        .map(|n| format!("Number: {}", n));
    
    match mapped_result {
        Ok(s) => println!("Mapped result: {}", s),
        Err(e) => println!("Mapping error: {}", e),
    }
    
    // 4. Use map_err for transforming error values
    let transformed_error = parse_number("not_a_number")
        .map_err(|_| "Failed to parse as number");
    
    match transformed_error {
        Ok(n) => println!("Number: {}", n),
        Err(msg) => println!("Custom error message: {}", msg),
    }
}

// Demonstration of when to use Result vs panic
fn when_to_use_result_vs_panic() {
    println!("=== When to use Result vs panic! ===");
    
    // Use Result for:
    // - Expected failure conditions
    // - Recoverable errors
    // - Library code where caller should decide how to handle errors
    
    fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(a / b)
        }
    }
    
    // Use panic! for:
    // - Unrecoverable errors
    // - Programming errors (bugs)
    // - When continuing would be unsafe or invalid
    
    fn array_access_safe(arr: &[i32], index: usize) -> Option<i32> {
        arr.get(index).copied()
    }
    
    match safe_divide(10, 0) {
        Ok(result) => println!("Division result: {}", result),
        Err(msg) => println!("Division failed: {}", msg),
    }
    
    let numbers = [1, 2, 3, 4, 5];
    match array_access_safe(&numbers, 10) {
        Some(value) => println!("Array value: {}", value),
        None => println!("Index out of bounds"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_ok() {
        let result: Result<i32, &str> = Ok(42);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_result_err() {
        let result: Result<i32, &str> = Err("error");
        assert!(result.is_err());
    }

    #[test]
    fn test_result_mapping() {
        let result = Ok(5).map(|x| x * 2);
        assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn test_error_propagation() {
        // This function would propagate any IO errors
        fn test_function() -> Result<(), io::Error> {
            File::open("nonexistent.txt")?;
            Ok(())
        }
        
        assert!(test_function().is_err());
    }
}