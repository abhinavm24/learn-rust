# Chapter 12.3: Refactoring to Improve Modularity and Error Handling

## Key Takeaways

### Separation of Concerns
- **Extract Functions**: Move logic out of main()
- **Configuration Parsing**: Separate argument parsing logic
- **Error Handling**: Centralize error handling patterns
- **Modularity**: Create reusable, testable components

### Configuration Structure
```rust
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config { query, file_path })
    }
}
```

### Main Function Refactoring
```rust
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    println!("With text:\n{contents}");
    
    Ok(())
}
```

### Error Handling Improvements
- **Result Types**: Use Result for recoverable errors
- **Error Propagation**: Use ? operator for cleaner code
- **Custom Error Messages**: Provide meaningful error descriptions
- **Exit Codes**: Use appropriate exit codes for different errors

### Modular Design Benefits
- **Testability**: Separate functions can be unit tested
- **Reusability**: Functions can be used in different contexts
- **Maintainability**: Changes isolated to specific functions
- **Readability**: Clear separation of responsibilities

### Library vs Binary Separation
- **src/lib.rs**: Library code with business logic
- **src/main.rs**: Binary entry point with minimal logic
- **Public API**: Expose necessary functions and types
- **Testing**: Library code is easier to test

### Error Type Considerations
- **Box<dyn Error>**: Trait object for any error type
- **Custom Error Types**: More specific error handling
- **Error Conversion**: Automatic conversion with From trait
- **Error Chains**: Preserve error context through layers

Official Chapter: https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html

---
*Completed: ✓*