//! Chapter 1: Getting Started with Rust
//! 
//! This example demonstrates the very basics of Rust programming:
//! - Hello World program structure
//! - The main function as entry point
//! - Using println! macro for output
//! - Basic Rust syntax and conventions

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 1", "Getting Started with Rust");
    
    // The classic Hello, World! program
    println!("Hello, world!");
    
    // Demonstrate various println! capabilities
    basic_output_demo();
    formatting_demo();
    rust_info_demo();
    development_workflow_demo();
}

/// Demonstrates basic output capabilities
fn basic_output_demo() {
    println!("\n=== Basic Output ===");
    
    // Simple string output
    println!("This is a simple println! statement");
    
    // Multiple println statements
    println!("Rust is a systems programming language");
    println!("that runs blazingly fast,");
    println!("prevents segfaults,");
    println!("and guarantees thread safety.");
    
    // Using print! (without newline)
    print!("This ");
    print!("is ");
    print!("all ");
    print!("on ");
    println!("one line!");
}

/// Demonstrates string formatting capabilities
fn formatting_demo() {
    println!("\n=== String Formatting ===");
    
    // Variable substitution
    let name = "Rustacean";
    let language = "Rust";
    println!("Hello, {}! Welcome to {}!", name, language);
    
    // Positional arguments
    println!("{0} is learning {1}. {0} loves {1}!", "Alice", "Rust");
    println!("{1} was created by {0}", "Mozilla Research", "Rust");
    
    // Named arguments
    println!(
        "{language} emphasizes {safety}, {speed}, and {concurrency}",
        language = "Rust",
        safety = "memory safety",
        speed = "zero-cost abstractions", 
        concurrency = "fearless concurrency"
    );
    
    // Numbers and formatting
    let number = 42;
    let pi = 3.14159;
    println!("The answer is {}", number);
    println!("Pi is approximately {:.2}", pi);
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", number, number, number);
}

/// Demonstrates Rust toolchain information
fn rust_info_demo() {
    println!("\n=== Rust Environment Info ===");
    
    // Show compilation information (using available environment variables)
    println!("Debug mode: {}", cfg!(debug_assertions));
    println!("Optimization level: {}", if cfg!(debug_assertions) { "Debug" } else { "Release" });
    
    // Show target information if available at compile time
    if let Some(target) = option_env!("TARGET") {
        println!("Target triple: {}", target);
    } else {
        println!("Target triple: {}", std::env::consts::ARCH);
    }
    
    // Show file information
    println!("Source file: {}", file!());
    println!("Current line would be: {}", line!() + 1);
    println!("Module path: {}", module_path!());
    
    // Show cargo information if available
    if let Some(pkg_name) = option_env!("CARGO_PKG_NAME") {
        println!("Package name: {}", pkg_name);
    }
    if let Some(pkg_version) = option_env!("CARGO_PKG_VERSION") {
        println!("Package version: {}", pkg_version);
    }
}

/// Demonstrates key concepts from Chapter 1
fn development_workflow_demo() {
    println!("\n=== Development Workflow Concepts ===");
    
    // Compilation model
    println!("🔧 Rust is ahead-of-time compiled");
    println!("   - Source code (.rs) → Compiler (rustc) → Executable binary");
    println!("   - No interpreter needed at runtime");
    println!("   - Standalone executables");
    
    // Cargo features
    println!("\n📦 Cargo Build System:");
    println!("   - cargo new: Create new project");
    println!("   - cargo build: Compile project");
    println!("   - cargo run: Compile and run");
    println!("   - cargo check: Check without building");
    println!("   - cargo build --release: Optimized build");
    
    // Project structure
    println!("\n📁 Project Structure:");
    println!("   - Cargo.toml: Project configuration");
    println!("   - src/: Source code directory");
    println!("   - target/: Build artifacts");
    println!("   - Cargo.lock: Dependency lock file");
    
    // Language features
    println!("\n⚡ Rust Core Principles:");
    println!("   - Memory safety without garbage collection");
    println!("   - Zero-cost abstractions");
    println!("   - Fearless concurrency");
    println!("   - Speed and reliability");
    
    // Macros
    println!("\n🔮 Macros (note the '!' syntax):");
    println!("   - println! is a macro, not a function");
    println!("   - Macros generate code at compile time");
    println!("   - Distinguished by '!' at the end");
    
    // Community
    println!("\n🦀 Community:");
    println!("   - Rustaceans: Members of the Rust community");
    println!("   - Cargo crates: Shared libraries");
    println!("   - rustup: Toolchain installer and updater");
    
    // Documentation
    println!("\n📚 Documentation:");
    println!("   - rustup doc: Offline documentation");
    println!("   - Built-in documentation system");
    println!("   - Example-driven learning");
    
    println!("\n🎉 Welcome to your Rust journey!");
    println!("You've successfully run your first Rust program!");
}

// Additional examples of Rust syntax basics
#[allow(dead_code)]
fn syntax_examples() {
    // This function demonstrates basic Rust syntax
    // (marked with #[allow(dead_code)] so it doesn't warn about being unused)
    
    // Variables (immutable by default)
    let message = "Hello, Rust!";
    println!("{}", message);
    
    // Mutable variables (must be explicit)
    let mut counter = 0;
    counter += 1;
    println!("Counter: {}", counter);
    
    // Function calls
    greet_language("Rust");
    
    // Comments
    // This is a single-line comment
    /* This is a 
       multi-line comment */
    
    // Documentation comments (these would be used for documenting functions/structs)
    // /// This is a documentation comment
    // /// It can be used to generate documentation
    
    // Blocks and expressions
    let result = {
        let x = 3;
        let y = 4;
        x + y  // Note: no semicolon, this is an expression
    };
    println!("Result: {}", result);
}

#[allow(dead_code)]
fn greet_language(_language: &str) {
    // Helper function to demonstrate function calls
    // The underscore prefix prevents "unused parameter" warnings
}

/// Demonstrates the difference between statements and expressions
#[allow(dead_code)]
fn statements_vs_expressions() {
    // Statements perform actions but don't return values
    let x = 5; // This is a statement
    
    // Expressions evaluate to values
    let y = {
        let x = 3;
        x + 1  // This expression becomes the value of y
    };
    
    println!("x = {}, y = {}", x, y);
    
    // Function calls are expressions
    let z = add_one(5);
    println!("z = {}", z);
}

#[allow(dead_code)]
fn add_one(x: i32) -> i32 {
    x + 1  // Expression (no semicolon)
}