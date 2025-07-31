//! Chapter 7.1: Packages and Crates
//! 
//! This example demonstrates Rust's module system fundamentals including packages,
//! crates, and how they work together to organize code. A package is a bundle of
//! one or more crates, and a crate is the smallest unit of code that the Rust
//! compiler considers at a time.
//!
//! Key concepts:
//! - Understanding packages vs crates
//! - Binary crates vs library crates
//! - Cargo conventions for project structure
//! - How the compiler processes crates
//! - Creating multiple binary targets

use rust_book_examples::print_chapter_header;

// This example demonstrates concepts that would typically be spread across
// multiple files in a real package structure. For educational purposes,
// we'll simulate various scenarios in a single file.

// === Simulating Library Crate Functionality ===

/// Basic arithmetic operations that might be in a library crate
pub mod calculator {
    /// Adds two numbers together
    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    /// Subtracts second number from first
    pub fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }

    /// Multiplies two numbers
    pub fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }

    /// Divides first number by second
    pub fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("Division by zero".to_string())
        } else {
            Ok(a / b)
        }
    }

    /// Operation enum for calculator
    #[derive(Debug, Clone)]
    pub enum Operation {
        Add,
        Subtract,
        Multiply,
        Divide,
    }

    /// Calculate result based on operation
    pub fn calculate(a: f64, b: f64, op: Operation) -> Result<f64, String> {
        match op {
            Operation::Add => Ok(add(a, b)),
            Operation::Subtract => Ok(subtract(a, b)),
            Operation::Multiply => Ok(multiply(a, b)),
            Operation::Divide => divide(a, b),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_add() {
            assert_eq!(add(2.0, 3.0), 5.0);
        }

        #[test]
        fn test_divide_by_zero() {
            assert!(divide(5.0, 0.0).is_err());
        }
    }
}

// === Simulating Multiple Binary Targets ===

/// This would typically be in src/bin/client.rs
mod client_simulation {
    use super::calculator::{calculate, Operation};

    pub fn run_client() {
        println!("=== Client Binary Simulation ===");
        println!("Client connecting to calculator service...");
        
        let result = calculate(10.0, 5.0, Operation::Add);
        match result {
            Ok(value) => println!("Client calculated: 10 + 5 = {}", value),
            Err(error) => println!("Client error: {}", error),
        }
        
        println!("Client disconnected.");
    }
}

/// This would typically be in src/bin/server.rs
mod server_simulation {
    use super::calculator::{calculate, Operation};

    pub fn run_server() {
        println!("=== Server Binary Simulation ===");
        println!("Server starting calculator service...");
        
        let operations = vec![
            (20.0, 4.0, Operation::Divide),
            (15.0, 3.0, Operation::Multiply),
            (100.0, 25.0, Operation::Subtract),
        ];
        
        for (a, b, op) in operations {
            match calculate(a, b, op.clone()) {
                Ok(result) => println!("Server processed: {} {:?} {} = {}", a, op, b, result),
                Err(error) => println!("Server error: {}", error),
            }
        }
        
        println!("Server stopped.");
    }
}

// === Demonstrating Package Structure Concepts ===

fn demonstrate_package_structure() {
    println!("\n=== Package Structure Concepts ===");
    
    println!("📦 Package Structure:");
    println!("my-project/");
    println!("├── Cargo.toml          # Package metadata and dependencies");
    println!("└── src/");
    println!("    ├── lib.rs          # Library crate root (optional)");
    println!("    ├── main.rs         # Default binary crate root");
    println!("    └── bin/            # Additional binary crates");
    println!("        ├── client.rs   # Binary: cargo run --bin client");
    println!("        └── server.rs   # Binary: cargo run --bin server");
    
    println!("\n🔧 Cargo Commands:");
    println!("cargo new my-project            # Create new package");
    println!("cargo new --lib my-library      # Create library-only package");
    println!("cargo build                     # Build default binary");
    println!("cargo run                       # Run default binary");
    println!("cargo run --bin client          # Run specific binary");
    println!("cargo build --lib               # Build library only");
    println!("cargo test                      # Run tests");
}

fn demonstrate_crate_types() {
    println!("\n=== Crate Types ===");
    
    println!("🗃️ Binary Crate:");
    println!("• Contains a main() function");
    println!("• Compiles to an executable");
    println!("• Entry point: src/main.rs or src/bin/*.rs");
    println!("• Purpose: Applications that can be run");
    
    println!("\n📚 Library Crate:");
    println!("• No main() function");
    println!("• Provides functionality for other programs");
    println!("• Entry point: src/lib.rs");
    println!("• Purpose: Shared code and reusable components");
    
    println!("\n⚖️ Package Rules:");
    println!("• Must contain at least one crate (library or binary)");
    println!("• Can contain at most one library crate");
    println!("• Can contain any number of binary crates");
    println!("• All crates in package share the same Cargo.toml");
}

fn demonstrate_compilation_units() {
    println!("\n=== Compilation Units ===");
    
    println!("🔄 How Rust Compiles Crates:");
    println!("1. Each crate is compiled as a separate unit");
    println!("2. Compiler starts from the crate root (main.rs or lib.rs)");
    println!("3. Dependencies are resolved at link time");
    println!("4. Multiple crates can be compiled in parallel");
    
    println!("\n🎯 Crate Boundaries:");
    println!("• Define API surfaces between compilation units");
    println!("• pub keyword controls visibility across crate boundaries");
    println!("• Use statements bring external crate items into scope");
    println!("• Each crate has its own namespace");
}

fn demonstrate_cargo_toml_structure() {
    println!("\n=== Cargo.toml Configuration ===");
    
    println!("📋 Basic Package Configuration:");
    println!("[package]");
    println!("name = \"my-project\"");
    println!("version = \"0.1.0\"");
    println!("edition = \"2021\"");
    println!();
    println!("[dependencies]");
    println!("serde = \"1.0\"");
    println!();
    println!("# Binary target definitions");
    println!("[[bin]]");
    println!("name = \"client\"");
    println!("path = \"src/bin/client.rs\"");
    println!();
    println!("[[bin]]");
    println!("name = \"server\"");
    println!("path = \"src/bin/server.rs\"");
}

fn demonstrate_real_world_usage() {
    println!("\n=== Real-World Applications ===");
    
    println!("🏗️ Common Package Patterns:");
    println!("• CLI Tool: Binary crate that uses library crate for core logic");
    println!("• Web Service: Multiple binaries (server, client, admin) sharing library");
    println!("• Game Engine: Library crate with example binaries for demos");
    println!("• Utility Suite: Multiple related tools sharing common functionality");
    
    println!("\n📊 Examples in the Rust Ecosystem:");
    println!("• cargo: Package with multiple binary targets");
    println!("• serde: Pure library crate for serialization");
    println!("• tokio: Library with optional binary utilities");
    println!("• ripgrep: Binary crate with library for reusable search logic");
}

fn main() {
    print_chapter_header("Chapter 7.1", "Packages and Crates");

    println!("This example demonstrates Rust's module system fundamentals.");
    println!("In a real project, the code would be organized across multiple files.");

    demonstrate_package_structure();
    demonstrate_crate_types();
    demonstrate_compilation_units();
    demonstrate_cargo_toml_structure();
    demonstrate_real_world_usage();

    // === Simulating Multiple Binary Execution ===
    println!("\n{}", "=".repeat(50));
    println!("SIMULATING MULTIPLE BINARY TARGETS");
    println!("{}", "=".repeat(50));

    // These would normally be separate executables
    client_simulation::run_client();
    println!();
    server_simulation::run_server();

    // === Demonstrating Library Usage ===
    println!("\n=== Using Library Functionality ===");
    
    use calculator::{calculate, Operation};
    
    let calculations = vec![
        (5.0, 3.0, Operation::Add),
        (10.0, 4.0, Operation::Subtract),
        (7.0, 2.0, Operation::Multiply),
        (15.0, 3.0, Operation::Divide),
        (10.0, 0.0, Operation::Divide), // This will error
    ];
    
    for (a, b, op) in calculations {
        match calculate(a, b, op.clone()) {
            Ok(result) => println!("{} {:?} {} = {}", a, op, b, result),
            Err(error) => println!("{} {:?} {} -> Error: {}", a, op, b, error),
        }
    }

    println!("\n=== Key Takeaways ===");
    println!("• Package = Collection of crates + Cargo.toml");
    println!("• Crate = Smallest compilation unit");
    println!("• Binary crates have main(), library crates don't");
    println!("• Cargo follows conventions for file organization");
    println!("• Multiple targets allow building different executables from shared code");
    println!("• Understanding this foundation is essential for larger Rust projects");
}