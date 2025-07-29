# Chapter 7.1: Packages and Crates

## Key Takeaways

### Module System Overview
- Rust's module system organizes code into packages, crates, modules, and paths
- Provides code organization, encapsulation, and reusability
- Enables building scalable applications with clear separation of concerns
- Supports both library and binary distributions

### Crate Fundamentals
- **Crate**: Smallest unit of code the Rust compiler processes at once
- **Crate Root**: Source file where compiler begins compilation
- **Binary Crate**: Executable programs that have a `main` function
- **Library Crate**: Shared functionality without a `main` function

### Package Structure
- **Package**: Bundle of one or more crates with a `Cargo.toml` file
- Describes how to build the crates it contains
- Can contain multiple binary crates but at most one library crate
- Must contain at least one crate (either library or binary)

### Cargo Conventions
- `src/main.rs` - Binary crate root (same name as package)
- `src/lib.rs` - Library crate root (same name as package)
- `src/bin/` - Directory for additional binary crates
- Each file in `src/bin/` becomes a separate binary crate

### Important Concepts

#### Compilation Units
- Compiler processes entire crates as single units
- Cross-crate dependencies resolved at link time
- Each crate compiled separately for parallel builds
- Crate boundaries define API surfaces

#### Package vs Crate Relationship
- Package = Collection of crates + metadata (Cargo.toml)
- Crate = Compilation unit with code
- One package can produce multiple executable binaries
- Library crates provide shared functionality

### Code Examples and Patterns

#### Basic Package Structure
```
my-project/
├── Cargo.toml
└── src/
    └── main.rs
```

**Cargo.toml:**
```toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2021"
```

**src/main.rs:**
```rust
fn main() {
    println!("Hello, world!");
}
```

#### Package with Library and Binary
```
my-project/
├── Cargo.toml
└── src/
    ├── lib.rs
    └── main.rs
```

**src/lib.rs:**
```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

**src/main.rs:**
```rust
use my_project::greet;

fn main() {
    println!("{}", greet("Rust"));
}
```

#### Multiple Binary Crates
```
my-project/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── main.rs
    └── bin/
        ├── client.rs
        └── server.rs
```

**src/bin/client.rs:**
```rust
use my_project::greet;

fn main() {
    println!("Client: {}", greet("Client"));
}
```

**src/bin/server.rs:**
```rust
use my_project::greet;

fn main() {
    println!("Server: {}", greet("Server"));
}
```

#### Building Different Targets
```bash
# Build default binary (src/main.rs)
cargo build

# Run default binary
cargo run

# Run specific binary
cargo run --bin client
cargo run --bin server

# Build all binaries
cargo build --bins
```

#### Library-Only Package
```
my-library/
├── Cargo.toml
└── src/
    └── lib.rs
```

**Cargo.toml:**
```toml
[package]
name = "my-library"
version = "0.1.0"
edition = "2021"

[lib]
name = "my_library"
```

#### Real-World Example: CLI Tool with Library
```
calculator/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── main.rs
    └── bin/
        └── calc-server.rs
```

**src/lib.rs:**
```rust
//! A simple calculator library

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
```

**src/main.rs:**
```rust
use calculator::{calculate, Operation};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 4 {
        println!("Usage: {} <number1> <operation> <number2>", args[0]);
        return;
    }
    
    let a: f64 = args[1].parse().expect("Invalid first number");
    let b: f64 = args[3].parse().expect("Invalid second number");
    
    let operation = match args[2].as_str() {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _ => {
            println!("Invalid operation. Use +, -, *, or /");
            return;
        }
    };
    
    match calculate(a, b, operation) {
        Ok(result) => println!("{} {} {} = {}", a, args[2], b, result),
        Err(error) => println!("Error: {}", error),
    }
}
```

### Practical Applications
- Organizing large applications into logical units
- Creating reusable libraries for distribution
- Building CLI tools with both library and binary components
- Separating core functionality from user interfaces
- Creating multiple executables from shared code

### Cargo Commands for Packages and Crates
```bash
# Create new binary package
cargo new my-binary

# Create new library package
cargo new --lib my-library

# Build library
cargo build --lib

# Run tests
cargo test

# Generate documentation
cargo doc --open

# Check without building
cargo check

# Build in release mode
cargo build --release
```

### Integration with Previous Chapters
- Uses functions and structs as building blocks within crates
- Applies ownership rules across crate boundaries
- Enables modular design with clear interfaces
- Supports testing with `#[cfg(test)]` modules

### Community Conventions and Idioms
- Use snake_case for package names in Cargo.toml
- Keep crate names short and descriptive
- Separate concerns between library and binary code
- Use semantic versioning for published crates
- Include comprehensive documentation for public APIs

### Package Configuration Options
```toml
[package]
name = "my-package"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
license = "MIT OR Apache-2.0"
description = "A short description of the package"
homepage = "https://example.com"
repository = "https://github.com/user/repo"
readme = "README.md"
keywords = ["cli", "tool"]
categories = ["command-line-utilities"]

[lib]
name = "my_package"
path = "src/lib.rs"

[[bin]]
name = "my-binary"
path = "src/bin/main.rs"
```

### Personal Notes
- Understanding packages and crates is fundamental to Rust project organization
- The separation between library and binary crates encourages good design
- Cargo's conventions make project structure predictable and discoverable
- Multiple binary crates enable creating tool suites from shared libraries
- This foundation is essential for understanding module system and visibility

Official Chapter: https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html

---
*Completed: ✓*