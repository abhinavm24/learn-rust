# Chapter 12.1: Accepting Command Line Arguments

## Key Takeaways

### Core Concepts
- **std::env::args()**: Function to access command line arguments as Unicode strings
- **Iterator Pattern**: Returns iterator over arguments for flexible processing
- **Program Name Access**: First argument (index 0) is always the program name
- **Argument Validation**: Essential for robust CLI applications
- **Unicode Safety**: Handles UTF-8 arguments with panic on invalid Unicode

### Important Syntax and Operators
- `std::env::args()` - Returns iterator over command line arguments
- `args.collect()` - Converts iterator to Vec<String>
- `&args[index]` - Access specific argument by index
- `args.len()` - Get total number of arguments
- `std::process::exit(code)` - Exit program with specific exit code
- `eprintln!()` - Print to stderr for error messages

### Programming Concepts Introduced
- **Command Line Interface (CLI)**: Programs that accept input via command line
- **Argument Parsing**: Converting string arguments to program parameters
- **Error Handling**: Graceful failure when arguments are missing or invalid
- **Exit Codes**: Standard way to indicate program success/failure to shell

## Code Examples and Patterns

### Basic Argument Reading
```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // args[0] is always the program name
    let query = &args[1];        // First user argument
    let file_path = &args[2];    // Second user argument
    
    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
```

### Safe Argument Access with Validation
```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <query> <file>", args[0]);
        std::process::exit(1);
    }
    
    let query = &args[1];
    let file_path = &args[2];
    
    println!("Searching for '{}' in file '{}'", query, file_path);
}
```

### Structured Configuration Pattern
```rust
use std::env;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config { query, file_path })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });
    
    println!("Searching for '{}' in file '{}'", config.query, config.file_path);
}
```

### Unicode-Safe Alternative
```rust
use std::env;
use std::ffi::OsString;

fn main() {
    let args_os: Vec<OsString> = env::args_os().collect();
    
    // Convert to strings with error handling
    let args: Result<Vec<String>, _> = args_os
        .iter()
        .map(|arg| arg.to_str().ok_or("Invalid Unicode"))
        .collect();
        
    match args {
        Ok(valid_args) => {
            // Process valid Unicode arguments
            if valid_args.len() >= 3 {
                println!("Query: {}, File: {}", valid_args[1], valid_args[2]);
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
```

## Practical Applications
- Building command-line tools and utilities
- Creating configurable programs that accept runtime parameters
- Implementing file processing tools (grep-like utilities)
- Developing deployment scripts and automation tools
- Building developer tools that integrate with shell workflows

## Integration with Previous Chapters
- **Prerequisites**: String handling (Chapter 4), error handling (Chapter 9), collections (Chapter 8)
- **Builds On**: Ownership concepts for handling argument strings
- **Connections**: Uses Vec<String> from collections, prepares for I/O operations in later sections

## Community Conventions and Idioms
- Use `clap` crate for complex argument parsing in real applications
- Follow Unix conventions: `--help`, `--version` flags
- Use `eprintln!` for error messages (stderr) vs `println!` for output (stdout)
- Exit with code 0 for success, non-zero for errors
- Provide clear usage messages when arguments are incorrect
- Consider using `structopt` or `clap` derive macros for type-safe argument parsing

## Personal Notes
- Raw argument parsing is tedious for complex CLIs - use dedicated crates
- Always validate arguments early to fail fast with clear error messages
- Remember that args[0] is the program name, user arguments start at index 1
- Unicode handling can be tricky - most apps can use `env::args()` safely
- Good error messages significantly improve user experience

Official Chapter: https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html

---
*Completed: ✓*