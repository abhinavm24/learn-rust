/*! 
# Rust Book Examples Library

This crate contains shared utilities and example functions for learning Rust concepts
from "The Rust Programming Language" book.

## Key Concepts Demonstrated

- **Ownership**: How Rust manages memory through ownership rules
- **Borrowing**: References that allow you to use values without taking ownership
- **Lifetimes**: Ensuring references are valid for as long as needed
- **Error Handling**: Using Result and Option types for safe error handling

## Usage

Run individual examples with:
```bash
cargo run --example ch02_guessing_game
cargo run --example ch03_01_variables
cargo run --example ch04_01_ownership
```
*/

// === UTILITY FUNCTIONS ===

/// Prints a formatted separator for organizing output
/// 
/// # Arguments
/// * `title` - The title to display in the separator
/// 
/// # Example
/// ```
/// use rust_book_examples::print_separator;
/// print_separator("Variables");
/// // Output: === Variables ===
/// ```
#[allow(dead_code)]
pub fn print_separator(title: &str) {
    println!("\n=== {} ===", title);
}

/// Prints a formatted chapter header with borders
/// 
/// This function demonstrates:
/// - String formatting with padding and centering
/// - The `format!` macro for string interpolation
/// 
/// # Arguments
/// * `chapter` - The chapter identifier (e.g., "Chapter 3.1")
/// * `title` - The chapter title
/// 
/// # Example
/// ```
/// use rust_book_examples::print_chapter_header;
/// print_chapter_header("Chapter 4.1", "What is Ownership?");
/// ```
#[allow(dead_code)]
pub fn print_chapter_header(chapter: &str, title: &str) {
    println!("\n{:=^50}", format!(" {} ", chapter));
    println!("{:^50}", title);
    println!("{:=^50}", "");
}

// === OWNERSHIP DEMONSTRATION FUNCTIONS ===

/// Module containing functions that demonstrate ownership concepts
/// 
/// These functions are designed to show how ownership, borrowing, and the Copy trait
/// work in Rust. They're used across multiple examples to maintain consistency.
#[allow(dead_code)]
pub mod ownership_examples {
    
    /// Takes ownership of a String and prints it
    /// 
    /// This function demonstrates:
    /// - Taking ownership of heap-allocated data (String)
    /// - What happens when a value is moved into a function
    /// - Automatic cleanup when the owner goes out of scope
    /// 
    /// # Arguments
    /// * `some_string` - A String that will be moved into this function
    /// 
    /// # Important Notes
    /// After calling this function, the original variable is no longer valid
    /// because ownership has been transferred.
    /// 
    /// # Example
    /// ```
    /// use rust_book_examples::ownership_examples::takes_ownership;
    /// 
    /// let s = String::from("hello");
    /// takes_ownership(s);
    /// // s is no longer valid here - it was moved
    /// ```
    pub fn takes_ownership(some_string: String) {
        println!("takes_ownership received: {}", some_string);
        // some_string goes out of scope here and `drop` is called
        // The memory is automatically freed
    }

    /// Takes a copy of an integer and prints it
    /// 
    /// This function demonstrates:
    /// - The Copy trait for stack-allocated data
    /// - Why simple types like i32 can be used after being passed to functions
    /// - The difference between Copy and Move semantics
    /// 
    /// # Arguments
    /// * `some_integer` - An i32 that will be copied (not moved)
    /// 
    /// # Example
    /// ```
    /// use rust_book_examples::ownership_examples::makes_copy;
    /// 
    /// let x = 5;
    /// makes_copy(x);
    /// println!("{}", x); // x is still valid because i32 implements Copy
    /// ```
    pub fn makes_copy(some_integer: i32) {
        println!("makes_copy received: {}", some_integer);
        // some_integer goes out of scope, but since i32 implements Copy,
        // no special cleanup is needed and the original variable remains valid
    }

    /// Creates a String and returns ownership to the caller
    /// 
    /// This function demonstrates:
    /// - How functions can create and transfer ownership
    /// - Return value ownership transfer
    /// - Creating heap-allocated data within a function
    /// 
    /// # Returns
    /// A String with ownership transferred to the caller
    /// 
    /// # Example
    /// ```
    /// use rust_book_examples::ownership_examples::gives_ownership;
    /// 
    /// let s = gives_ownership();
    /// println!("{}", s); // We now own the String
    /// ```
    pub fn gives_ownership() -> String {
        let some_string = String::from("yours"); // Create a new String
        some_string // Return ownership to the calling function
    }

    /// Takes ownership of a String and returns it back
    /// 
    /// This function demonstrates:
    /// - Taking ownership through parameters
    /// - Returning ownership through return values
    /// - The pattern of "taking and giving back" ownership
    /// 
    /// # Arguments
    /// * `a_string` - A String that will be moved in and moved back out
    /// 
    /// # Returns
    /// The same String, with ownership transferred back to caller
    /// 
    /// # Example
    /// ```
    /// use rust_book_examples::ownership_examples::takes_and_gives_back;
    /// 
    /// let s1 = String::from("hello");
    /// let s2 = takes_and_gives_back(s1);
    /// // s1 is no longer valid, but s2 contains the same value
    /// println!("{}", s2);
    /// ```
    pub fn takes_and_gives_back(a_string: String) -> String {
        println!("takes_and_gives_back received: {}", a_string);
        a_string // Return ownership to the calling function
    }
}