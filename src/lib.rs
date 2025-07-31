//! # Rust Book Examples Library
//!
//! This crate contains shared utilities for the Rust Book examples.
//! Each example is self-contained with its own functions and explanations.
//!
//! ## Organization
//!
//! - **examples/**: Individual chapter examples with comprehensive explanations
//! - **src/lib.rs**: Shared utility functions used across multiple examples
//!
//! ## Key Concepts Covered
//!
//! - **Chapter 1**: Getting Started with Rust
//! - **Chapter 2**: Programming a Guessing Game
//! - **Chapter 3**: Variables, Data Types, Functions, Comments, Control Flow  
//! - **Chapter 4**: Ownership, References & Borrowing, Slices
//! - **Chapter 5**: Using Structs to Structure Related Data
//! - **Chapter 6**: Enums and Pattern Matching
//! - **Chapter 7**: Managing Growing Projects with Packages, Crates, and Modules
//! - **Chapter 8**: Common Collections (Vectors, Strings, Hash Maps)
//! - **Chapter 9**: Error Handling
//! - **Chapter 10**: Generic Types, Traits, and Lifetimes
//! - **Chapter 11**: Writing Automated Tests
//! - **Chapter 12**: I/O Project: Building a Command Line Program
//! - **Chapter 13**: Functional Language Features: Iterators and Closures
//! - **Chapter 14**: More about Cargo and Crates.io
//! - **Chapter 15**: Smart Pointers
//! - **Chapter 16**: Fearless Concurrency
//! - **Chapter 17**: Object Oriented Programming Features
//! - **Chapter 18**: Patterns and Matching
//! - **Chapter 19**: Advanced Features
//! - **Chapter 20**: Final Project: Building a Multithreaded Web Server
//!
//! ## Usage
//!
//! Run individual examples with:
//! ```bash
//! # Chapter 1: Getting Started
//! cargo run --example ch01_hello_world
//!
//! # Chapter 2: Programming a Guessing Game
//! cargo run --example ch02_guessing_game
//!
//! # Chapter 3: Common Programming Concepts
//! cargo run --example ch03_01_variables      # Variables and Mutability
//! cargo run --example ch03_02_data_types     # Data Types
//! cargo run --example ch03_03_functions      # Functions
//! cargo run --example ch03_04_comments       # Comments
//! cargo run --example ch03_05_control_flow   # Control Flow
//!
//! # Chapter 4: Understanding Ownership
//! cargo run --example ch04_01_ownership      # What is Ownership?
//! cargo run --example ch04_02_references_borrowing # References and Borrowing
//! cargo run --example ch04_03_slices         # The Slice Type
//!
//! # Chapter 5: Using Structs to Structure Related Data
//! cargo run --example ch05_01_defining_structs    # Defining and Instantiating Structs
//! cargo run --example ch05_02_example_structs     # An Example Program Using Structs
//! cargo run --example ch05_03_method_syntax       # Method Syntax
//!
//! # Chapter 6: Enums and Pattern Matching
//! cargo run --example ch06_01_defining_enums      # Defining an Enum
//! cargo run --example ch06_02_match_control_flow  # The match Control Flow Construct
//! cargo run --example ch06_03_if_let              # Concise Control Flow with if let
//!
//! # Chapter 7: Managing Growing Projects
//! cargo run --example ch07_01_packages_crates     # Packages and Crates
//!
//! # Chapter 8: Common Collections
//! cargo run --example ch08_01_vectors             # Storing Lists of Values with Vectors
//! cargo run --example ch08_02_strings             # Storing UTF-8 Encoded Text with Strings
//! cargo run --example ch08_03_hash_maps           # Storing Keys with Associated Values in Hash Maps
//!
//! # Chapter 7: Managing Growing Projects with Packages, Crates, and Modules
//! cargo run --example ch07_01_packages_crates     # Packages and Crates
//! cargo run --example ch07_02_defining_modules    # Defining Modules to Control Scope and Privacy
//! cargo run --example ch07_03_module_paths        # Paths for Referring to an Item in the Module Tree
//! cargo run --example ch07_04_bringing_paths_into_scope # Bringing Paths into Scope with the use Keyword
//! cargo run --example ch07_05_separating_modules  # Separating Modules into Different Files
//!
//! # Chapter 9: Error Handling
//! cargo run --example ch09_01_unrecoverable_errors_panic # Unrecoverable Errors with panic!
//! cargo run --example ch09_02_recoverable_errors        # Recoverable Errors with Result
//! cargo run --example ch09_03_to_panic_or_not          # To panic! or Not to panic!
//!
//! # Chapter 10: Generic Types, Traits, and Lifetimes
//! cargo run --example ch10_01_generic_data_types       # Generic Data Types
//! cargo run --example ch10_02_traits                   # Traits: Defining Shared Behavior
//! cargo run --example ch10_03_lifetime_syntax          # Validating References with Lifetimes
//!
//! # Chapter 11: Writing Automated Tests
//! cargo run --example ch11_01_writing_tests            # How to Write Tests
//! cargo run --example ch11_02_controlling_tests        # Controlling How Tests Are Run
//!
//! # Chapter 12: An I/O Project: Building a Command Line Program
//! cargo run --example ch12_01_accepting_cli_args       # Accepting Command Line Arguments
//!
//! # Chapter 13: Functional Language Features: Iterators and Closures
//! cargo run --example ch13_01_closures                 # Closures: Anonymous Functions that Capture Their Environment
//!
//! # Chapter 15: Smart Pointers
//! cargo run --example ch15_01_smart_pointers           # Using Box<T>, Rc<T>, and RefCell<T>
//! cargo run --example ch15_02_deref_trait              # Treating Smart Pointers Like Regular References with Deref
//! cargo run --example ch15_03_drop_trait               # Running Code on Cleanup with Drop
//! cargo run --example ch15_04_rc_smart_pointer         # Rc<T>, the Reference Counted Smart Pointer
//! cargo run --example ch15_05_refcell_interior_mutability # RefCell<T> and the Interior Mutability Pattern
//! cargo run --example ch15_06_reference_cycles         # Reference Cycles Can Leak Memory
//!
//! # Chapter 16: Fearless Concurrency
//! cargo run --example ch16_01_threads                  # Using Threads to Run Code Simultaneously
//! cargo run --example ch16_02_message_passing          # Using Message Passing to Transfer Data Between Threads
//! cargo run --example ch16_03_shared_state             # Shared-State Concurrency
//!
//! # Chapter 17: Object Oriented Programming Features of Rust
//! cargo run --example ch17_01_oop_characteristics      # Characteristics of Object-Oriented Languages
//! cargo run --example ch17_02_trait_objects            # Using Trait Objects That Allow for Values of Different Types
//! cargo run --example ch17_03_oop_implementations      # Implementing an Object-Oriented Design Pattern
//!
//! # Chapter 18: Patterns and Matching
//! cargo run --example ch18_01_pattern_locations        # All the Places Patterns Can Be Used
//! cargo run --example ch18_02_pattern_refutability     # Refutability: Whether a Pattern Might Fail to Match
//! cargo run --example ch18_03_pattern_syntax           # Pattern Syntax
//!
//! # Chapter 19: Advanced Features
//! cargo run --example ch19_01_unsafe_rust              # Unsafe Rust
//! cargo run --example ch19_02_advanced_traits          # Advanced Traits
//! cargo run --example ch19_03_advanced_types           # Advanced Types
//! cargo run --example ch19_04_advanced_functions       # Advanced Functions and Closures
//! cargo run --example ch19_05_macros                   # Macros
//!
//! # Chapter 20: Final Project: Building a Multithreaded Web Server
//! cargo run --example ch20_01_single_threaded          # Building a Single-Threaded Web Server
//! cargo run --example ch20_02_multithreaded            # Turning Our Single-Threaded Server into a Multithreaded Server
//! cargo run --example ch20_03_graceful_shutdown        # Graceful Shutdown and Cleanup
//! ```

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