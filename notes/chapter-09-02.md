# Chapter 9.2: Recoverable Errors with Result

## Key Takeaways

### Result Type Fundamentals
- **Result<T, E>**: Enum for operations that might fail but can be recovered from
- **Two Variants**: `Ok(T)` for success value, `Err(E)` for error value
- **Explicit Error Handling**: Forces consideration of error cases at compile time
- **Type Safety**: Both success and error types are known at compile time

### Result vs Panic Philosophy
- **Result**: For expected, recoverable errors (file not found, network timeout)
- **Panic**: For unexpected programming errors (array out of bounds, logic bugs)
- **Graceful Degradation**: Result allows programs to continue running
- **User Experience**: Result enables better error messages and recovery options

### Error Propagation
- **Manual Propagation**: Using match expressions to handle and forward errors
- **? Operator**: Concise syntax for early return on errors
- **Error Compatibility**: Error types must be convertible for propagation
- **Chain of Responsibility**: Errors can bubble up through call stack

### Important Syntax and Operators

#### Result Pattern Matching
```rust
match result {
    Ok(value) => // handle success,
    Err(error) => // handle error,
}
```

#### Error Propagation with ?
```rust
let value = operation()?;  // Return early if error, continue if Ok
```

#### Convenience Methods
```rust
result.unwrap()           // Get value or panic
result.expect("message")  // Get value or panic with message
result.unwrap_or(default) // Get value or return default
```

### Programming Concepts Introduced
- **Explicit Error Handling**: Making error cases visible in type system
- **Error Propagation**: Passing errors up the call stack
- **Early Return**: Exiting functions early when errors occur
- **Composable Error Handling**: Building complex error handling from simple parts

### Code Examples and Patterns

#### Basic Result Usage
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");
    
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            println!("Problem opening the file: {}", error);
            return;
        }
    };
    
    println!("File opened successfully");
}
```

#### Handling Different Error Types
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");
    
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```

#### Using unwrap and expect
```rust
use std::fs::File;

fn main() {
    // unwrap: panics with default message on error
    let greeting_file = File::open("hello.txt").unwrap();
    
    // expect: panics with custom message on error
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

#### Error Propagation with Manual Matching
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut username = String::new();
    
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn main() {
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(error) => println!("Error reading username: {}", error),
    }
}
```

#### Error Propagation with ? Operator
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Even more concise
fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Using standard library function
fn read_username_from_file_v3() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}
```

#### Using ? with main Function
```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    
    println!("File opened successfully");
    Ok(())
}
```

#### Real-World Example: Configuration Parser
```rust
use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
enum ConfigError {
    FileNotFound,
    ParseError(String),
    MissingField(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound => write!(f, "Configuration file not found"),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::MissingField(field) => write!(f, "Missing required field: {}", field),
        }
    }
}

impl std::error::Error for ConfigError {}

struct Config {
    host: String,
    port: u16,
    database_url: String,
}

impl Config {
    fn from_file(filename: &str) -> Result<Config, ConfigError> {
        // Read file
        let contents = fs::read_to_string(filename)
            .map_err(|_| ConfigError::FileNotFound)?;
        
        // Parse key-value pairs
        let mut settings = HashMap::new();
        for line in contents.lines() {
            if let Some((key, value)) = line.split_once('=') {
                settings.insert(key.trim(), value.trim());
            }
        }
        
        // Extract required fields
        let host = settings
            .get("host")
            .ok_or_else(|| ConfigError::MissingField("host".to_string()))?
            .to_string();
        
        let port_str = settings
            .get("port")
            .ok_or_else(|| ConfigError::MissingField("port".to_string()))?;
        
        let port = port_str
            .parse::<u16>()
            .map_err(|_| ConfigError::ParseError(format!("Invalid port: {}", port_str)))?;
        
        let database_url = settings
            .get("database_url")
            .ok_or_else(|| ConfigError::MissingField("database_url".to_string()))?
            .to_string();
        
        Ok(Config {
            host,
            port,
            database_url,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_file("config.txt")?;
    
    println!("Loaded config:");
    println!("  Host: {}", config.host);
    println!("  Port: {}", config.port);
    println!("  Database: {}", config.database_url);
    
    Ok(())
}
```

#### Result with Different Error Types
```rust
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    
    Ok(first_number * second_number)
}

fn main() {
    match multiply("10", "2") {
        Ok(result) => println!("10 * 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match multiply("ten", "2") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Parse error: {}", e),
    }
}
```

#### Combining Results with Different Error Types
```rust
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(ParseIntError),
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::Parse(error)
    }
}

fn read_number_from_file(filename: &str) -> Result<i32, AppError> {
    let mut file = File::open(filename)?;  // io::Error -> AppError
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;   // io::Error -> AppError
    
    let number = contents.trim().parse()?; // ParseIntError -> AppError
    Ok(number)
}

fn main() {
    match read_number_from_file("number.txt") {
        Ok(number) => println!("Number from file: {}", number),
        Err(AppError::Io(e)) => println!("IO error: {}", e),
        Err(AppError::Parse(e)) => println!("Parse error: {}", e),
    }
}
```

#### Result Combinators
```rust
fn main() {
    let result: Result<i32, &str> = Ok(10);
    
    // map: transform the Ok value
    let doubled = result.map(|x| x * 2);
    println!("Doubled: {:?}", doubled); // Ok(20)
    
    // map_err: transform the Err value
    let result: Result<i32, &str> = Err("error");
    let mapped_error = result.map_err(|e| format!("Failed: {}", e));
    println!("Mapped error: {:?}", mapped_error); // Err("Failed: error")
    
    // and_then: chain operations that return Results
    let result = Ok(10);
    let chained = result.and_then(|x| {
        if x > 5 {
            Ok(x * 2)
        } else {
            Err("too small")
        }
    });
    println!("Chained: {:?}", chained); // Ok(20)
    
    // unwrap_or: provide default value on error
    let result: Result<i32, &str> = Err("error");
    let value = result.unwrap_or(42);
    println!("With default: {}", value); // 42
    
    // unwrap_or_else: compute default value on error
    let result: Result<i32, &str> = Err("error");
    let value = result.unwrap_or_else(|_| 100);
    println!("With computed default: {}", value); // 100
}
```

#### Early Return Patterns
```rust
use std::fs::File;
use std::io::{self, Read};

fn process_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    // Process contents
    let processed = contents.to_uppercase();
    
    Ok(processed)
}

// Without ? operator (more verbose)
fn process_file_verbose(filename: &str) -> Result<String, io::Error> {
    let file_result = File::open(filename);
    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    
    let mut contents = String::new();
    let read_result = file.read_to_string(&mut contents);
    match read_result {
        Ok(_) => {},
        Err(e) => return Err(e),
    }
    
    let processed = contents.to_uppercase();
    Ok(processed)
}
```

#### Error Handling in Loops
```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn process_lines(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    let mut processed_lines = Vec::new();
    
    for line_result in reader.lines() {
        let line = line_result?; // Propagate any IO error
        
        if !line.trim().is_empty() {
            processed_lines.push(line.to_uppercase());
        }
    }
    
    Ok(processed_lines)
}

// Alternative: collect all results
fn process_lines_collect(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    reader
        .lines()
        .map(|line_result| {
            line_result.map(|line| {
                if line.trim().is_empty() {
                    None
                } else {
                    Some(line.to_uppercase())
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|opts| opts.into_iter().flatten().collect())
}
```

#### Testing Error Cases
```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_divide_success() {
        let result = divide(10.0, 2.0);
        assert_eq!(result, Ok(5.0));
    }
    
    #[test]
    fn test_divide_by_zero() {
        let result = divide(10.0, 0.0);
        assert_eq!(result, Err("Division by zero".to_string()));
    }
    
    #[test]
    fn test_divide_unwrap() {
        let result = divide(10.0, 2.0).unwrap();
        assert_eq!(result, 5.0);
    }
    
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_divide_panic() {
        divide(10.0, 0.0).expect("Division by zero");
    }
}
```

### Practical Applications
- File and network I/O operations
- Parsing user input and configuration files
- Database operations and queries
- API calls and HTTP requests
- Mathematical operations that can fail
- Resource allocation and cleanup

### Common Result Methods
- `is_ok()` / `is_err()` - Check variant without consuming
- `ok()` / `err()` - Convert to Option, discarding other variant
- `map(f)` / `map_err(f)` - Transform Ok or Err values
- `and_then(f)` - Chain operations that return Result
- `or_else(f)` - Provide alternative on error
- `unwrap_or(default)` - Get value or provide default
- `unwrap_or_else(f)` - Get value or compute default

### Error Handling Strategies

#### When to Use Different Approaches
```rust
// Use ? for propagating errors up
fn operation() -> Result<String, io::Error> {
    let contents = std::fs::read_to_string("file.txt")?;
    Ok(contents.to_uppercase())
}

// Use unwrap_or for providing defaults
fn get_config_value() -> String {
    std::env::var("CONFIG_PATH").unwrap_or_else(|_| "default.conf".to_string())
}

// Use match for complex error handling
fn handle_complex_error() {
    match risky_operation() {
        Ok(value) => println!("Success: {}", value),
        Err(ErrorType::NetworkError) => println!("Network issue, retrying..."),
        Err(ErrorType::ParseError(msg)) => println!("Data format error: {}", msg),
        Err(ErrorType::PermissionError) => println!("Access denied"),
    }
}
```

### Integration with Previous Chapters
- Uses enums and pattern matching from Chapter 6
- Applies to file I/O and collections from Chapter 8
- Builds on panic handling from Chapter 9.1
- Enables robust error handling in real applications

### Community Conventions and Idioms
- Prefer `Result` over panics for recoverable errors
- Use `?` operator for clean error propagation
- Implement `From` traits for error type conversions
- Use `expect` with descriptive messages instead of `unwrap`
- Consider using `anyhow` or `thiserror` crates for complex error handling

### Performance Considerations
- Result is zero-cost abstraction - no runtime overhead
- Error propagation with `?` is as fast as manual checking
- Early returns prevent unnecessary computation
- Result enables compiler optimizations for error paths

### Personal Notes
- Result forces explicit consideration of error cases
- The `?` operator makes error handling concise and readable
- Proper error types make debugging much easier
- Understanding Result is essential for idiomatic Rust
- The type system prevents forgetting to handle errors

Official Chapter: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html

---
*Completed: ✓*