# Chapter 14.5: Extending Cargo with Custom Commands

## Key Takeaways

### Custom Cargo Commands
- **Subcommand Extensions**: Add custom commands to cargo
- **Naming Convention**: Executables named `cargo-commandname`
- **PATH Integration**: Must be in PATH to be discoverable
- **Standard Interface**: Behave like built-in cargo commands

### Creating Custom Commands
```bash
# Create a new binary crate for custom command
cargo new cargo-hello --name cargo-hello

# The binary name determines the command name
# cargo-hello -> cargo hello
```

### Basic Custom Command Implementation
```rust
// src/main.rs
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // First arg is program name: cargo-hello
    // Second arg is the subcommand: hello
    // Remaining args are the actual arguments
    
    if args.len() > 2 {
        println!("Hello, {}!", args[2]);
    } else {
        println!("Hello, world!");
    }
}
```

### Installing Custom Commands
```bash
# Install from local development
cargo install --path .

# Now you can use it
cargo hello           # Prints "Hello, world!"
cargo hello Rust      # Prints "Hello, Rust!"
```

### Advanced Custom Command Example
```rust
// cargo-count/src/main.rs
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Skip "cargo-count" and "count"
    let target_dir = if args.len() > 2 {
        &args[2]
    } else {
        "."
    };
    
    match count_rust_files(target_dir) {
        Ok(count) => println!("Found {} Rust files", count),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn count_rust_files(dir: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut count = 0;
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "rs" {
                    count += 1;
                }
            }
        } else if path.is_dir() {
            // Recursively count in subdirectories
            count += count_rust_files(path.to_str().unwrap())?;
        }
    }
    
    Ok(count)
}
```

### Command with Cargo Integration
```rust
// cargo-check-deps/src/main.rs
use std::env;
use std::process::Command;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Check if we're in a Cargo project
    if !Path::new("Cargo.toml").exists() {
        eprintln!("Error: Not in a Cargo project directory");
        std::process::exit(1);
    }
    
    // Read Cargo.toml
    let cargo_toml = match fs::read_to_string("Cargo.toml") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading Cargo.toml: {}", e);
            std::process::exit(1);
        }
    };
    
    // Simple dependency counting
    let dep_count = cargo_toml.matches("[dependencies]").count() 
                  + cargo_toml.matches("[dev-dependencies]").count()
                  + cargo_toml.matches("[build-dependencies]").count();
    
    println!("This project has {} dependency sections", dep_count);
    
    // Run cargo tree to show dependency tree
    if args.len() > 2 && args[2] == "--tree" {
        let output = Command::new("cargo")
            .arg("tree")
            .output()
            .expect("Failed to run cargo tree");
        
        println!("Dependency tree:");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
```

### Command with Configuration
```rust
// cargo-template/src/main.rs
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: cargo template <template-name>");
        std::process::exit(1);
    }
    
    let template_name = &args[2];
    
    match template_name.as_str() {
        "lib" => create_library_template(),
        "cli" => create_cli_template(),
        "web" => create_web_template(),
        _ => {
            eprintln!("Unknown template: {}", template_name);
            eprintln!("Available templates: lib, cli, web");
            std::process::exit(1);
        }
    }
}

fn create_library_template() {
    println!("Creating library template...");
    
    // Create directory structure
    fs::create_dir_all("src").unwrap();
    fs::create_dir_all("tests").unwrap();
    
    // Create lib.rs
    fs::write("src/lib.rs", r#"//! # My Library
//!
//! A useful library for...

/// Adds one to the input.
///
/// # Examples
///
/// ```
/// let result = my_lib::add_one(5);
/// assert_eq!(result, 6);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_one(5), 6);
    }
}
"#).unwrap();
    
    // Create Cargo.toml
    fs::write("Cargo.toml", r#"[package]
name = "my-lib"
version = "0.1.0"
edition = "2021"

[dependencies]
"#).unwrap();
    
    println!("Library template created successfully!");
}

fn create_cli_template() {
    println!("Creating CLI template...");
    // Similar implementation for CLI template
}

fn create_web_template() {
    println!("Creating web template...");
    // Similar implementation for web template
}
```

### Argument Parsing for Custom Commands
```toml
# Cargo.toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
```

```rust
// Using clap for better argument parsing
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    MyCommand(MyCommandArgs),
}

#[derive(clap::Args)]
#[command(author, version, about = "My custom cargo command")]
struct MyCommandArgs {
    #[arg(short, long)]
    verbose: bool,
    
    #[arg(short, long, default_value = ".")]
    path: String,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Count { pattern: String },
    List { filter: Option<String> },
}

fn main() {
    let CargoCli::MyCommand(args) = CargoCli::parse();
    
    if args.verbose {
        println!("Running in verbose mode");
    }
    
    match args.command {
        Some(Commands::Count { pattern }) => {
            println!("Counting files matching: {}", pattern);
        }
        Some(Commands::List { filter }) => {
            println!("Listing files with filter: {:?}", filter);
        }
        None => {
            println!("No subcommand specified");
        }
    }
}
```

### Publishing Custom Commands
```toml
# Cargo.toml for publishable custom command
[package]
name = "cargo-mycmd"
version = "0.1.0"
edition = "2021"
description = "A useful cargo extension"
license = "MIT OR Apache-2.0"
repository = "https://github.com/user/cargo-mycmd"
keywords = ["cargo", "cli", "tools"]
categories = ["development-tools::cargo-plugins"]

[[bin]]
name = "cargo-mycmd"
path = "src/main.rs"

[dependencies]
clap = "4.0"
# other dependencies
```

### Example: Useful Custom Commands
```bash
# Some ideas for custom cargo commands:

# cargo outdated - check for outdated dependencies
# cargo audit - security audit
# cargo bloat - analyze binary size
# cargo expand - show macro expansions  
# cargo watch - rebuild on file changes
# cargo edit - add/remove dependencies from CLI
# cargo generate - generate projects from templates
# cargo release - automate release process
```

### Installation and Distribution
```bash
# Install from crates.io
cargo install cargo-mycmd

# Users can then run
cargo mycmd --help

# Install from source
git clone https://github.com/user/cargo-mycmd
cd cargo-mycmd
cargo install --path .
```

### Best Practices for Custom Commands
- **Follow Conventions**: Use standard cargo-like interfaces
- **Good Help Text**: Provide comprehensive help and examples
- **Error Handling**: Handle errors gracefully with helpful messages
- **Integration**: Work well with existing cargo workflows
- **Documentation**: Document installation and usage clearly
- **Testing**: Include tests for command functionality

Official Chapter: https://doc.rust-lang.org/book/ch14-05-extending-cargo.html

---
*Completed: ✓*