/*! 
# Rust Book Examples Library

This crate contains shared utilities for the Rust Book examples.
Each example is self-contained with its own functions and explanations.

## Organization

- **examples/**: Individual chapter examples with comprehensive explanations
- **src/lib.rs**: Shared utility functions used across multiple examples

## Key Concepts Covered

- **Chapter 2**: Programming a Guessing Game
- **Chapter 3**: Variables, Data Types, Functions, Comments, Control Flow  
- **Chapter 4**: Ownership, References & Borrowing, Slices

## Usage

Run individual examples with:
```bash
# Chapter 2: Programming a Guessing Game
cargo run --example ch02_guessing_game

# Chapter 3: Common Programming Concepts
cargo run --example ch03_01_variables      # Variables and Mutability
cargo run --example ch03_02_data_types     # Data Types
cargo run --example ch03_03_functions      # Functions
cargo run --example ch03_04_comments       # Comments
cargo run --example ch03_05_control_flow   # Control Flow

# Chapter 4: Understanding Ownership
cargo run --example ch04_01_ownership      # What is Ownership?
cargo run --example ch04_02_references_borrowing # References and Borrowing
cargo run --example ch04_03_slices         # The Slice Type
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

// === ADDITIONAL UTILITY FUNCTIONS ===
// Add more general-purpose utilities here as needed