# Chapter 9: Error Handling

## Key Takeaways

### Error Philosophy
- **Explicit Error Handling**: Errors are part of the type system
- **No Null Pointer Exceptions**: Option<T> eliminates null pointer dereferences
- **No Exceptions**: Result<T, E> for recoverable errors, panic! for unrecoverable
- **Fail Fast**: Panic when program state is invalid and cannot continue

### Error Types
- **Recoverable Errors**: Use Result<T, E> for expected failures (file not found, parse error)
- **Unrecoverable Errors**: Use panic! for programming bugs (array out of bounds, assertion failures)
- **Option<T>**: Represents absence of value safely
- **Custom Error Types**: Create domain-specific error types for better error handling

### Error Propagation
- **? Operator**: Automatic error propagation and early return
- **Error Chain**: Errors can be chained and transformed
- **Error Context**: Add context to errors as they propagate up
- **Error Conversion**: Automatic conversion between compatible error types

### Best Practices
- **Use Result for recoverable errors**: File I/O, network requests, parsing
- **Use panic! for programming errors**: Invalid indices, broken invariants
- **Provide good error messages**: Help users understand what went wrong
- **Handle errors at appropriate level**: Don't ignore errors, handle where it makes sense

## Chapter Structure

### 9.1: Unrecoverable Errors with panic!
```rust
// Basic panic
fn main() {
    panic!("crash and burn");
}

// Panic with formatting
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero: {} / {}", a, b);
    }
    a / b
}

// Panic from invalid access
fn main() {
    let v = vec![1, 2, 3];
    v[99];  // This will panic with index out of bounds
}

// Setting panic behavior
// In Cargo.toml:
// [profile.release]
// panic = 'abort'  // Don't unwind, just abort

// Using panic hooks for custom behavior
use std::panic;

fn main() {
    panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            println!("Custom panic handler: {}", s);
        }
        
        if let Some(location) = panic_info.location() {
            println!("Panic occurred at {}:{}", location.file(), location.line());
        }
    }));
    
    panic!("Something went wrong!");
}

// Catching panics (advanced usage)
use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {
        panic!("Oops!");
    });
    
    match result {
        Ok(_) => println!("No panic occurred"),
        Err(_) => println!("A panic occurred"),
    }
}
```

### 9.2: Recoverable Errors with Result
```rust
use std::fs::File;
use std::io::ErrorKind;

// Basic Result handling
fn main() {
    let f = File::open("hello.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };
}

// Different error handling strategies
fn main() {
    let f = File::open("hello.txt");
    
    let f = match f {
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

// Using unwrap and expect
fn main() {
    // unwrap: panic on error
    let f = File::open("hello.txt").unwrap();
    
    // expect: panic with custom message
    let f = File::open("hello.txt")
        .expect("Failed to open hello.txt");
}

// Propagating errors manually
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),  // Early return on error
    };
    
    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Using the ? operator for error propagation
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;  // Propagate error automatically
    let mut s = String::new();
    f.read_to_string(&mut s)?;  // Propagate error automatically
    Ok(s)
}

// Even more concise
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// Or using fs::read_to_string
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

### 9.3: To panic! or Not to panic!
```rust
// When to use panic!
// 1. Test code and example code
#[cfg(test)]
mod tests {
    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);  // Will panic if assertion fails
    }
}

// 2. When you have more information than compiler
use std::net::IpAddr;

fn main() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();  // We know this is valid
}

// 3. Programming errors (bugs)
fn get_element(slice: &[i32], index: usize) -> i32 {
    if index >= slice.len() {
        panic!("Index {} is out of bounds for slice of length {}", index, slice.len());
    }
    slice[index]
}

// When to use Result
// 1. File operations
fn read_config() -> Result<String, std::io::Error> {
    std::fs::read_to_string("config.toml")
}

// 2. Network operations
fn fetch_data(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Implementation would go here
    Ok("data".to_string())
}

// 3. User input validation
fn parse_age(input: &str) -> Result<u8, std::num::ParseIntError> {
    input.trim().parse()
}

// Creating custom types for validation
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        
        Guess { value }
    }
    
    pub fn value(&self) -> i32 {
        self.value
    }
}

// Better: Return Result for validation
impl Guess {
    pub fn try_new(value: i32) -> Result<Guess, String> {
        if value < 1 || value > 100 {
            Err(format!("Guess value must be between 1 and 100, got {}.", value))
        } else {
            Ok(Guess { value })
        }
    }
}
```

## Advanced Error Handling Patterns

### Custom Error Types
```rust
use std::fmt;

// Simple custom error
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Division by zero"),
            MathError::NegativeSquareRoot => write!(f, "Square root of negative number"),
            MathError::Overflow => write!(f, "Arithmetic overflow"),
        }
    }
}

impl std::error::Error for MathError {}

// Using custom error
fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

// Complex error with data
#[derive(Debug)]
struct ValidationError {
    field: String,
    message: String,
    code: u32,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Validation error in field '{}': {} (code: {})", 
               self.field, self.message, self.code)
    }
}

impl std::error::Error for ValidationError {}

fn validate_email(email: &str) -> Result<(), ValidationError> {
    if !email.contains('@') {
        return Err(ValidationError {
            field: "email".to_string(),
            message: "Must contain @ symbol".to_string(),
            code: 1001,
        });
    }
    
    if email.len() < 5 {
        return Err(ValidationError {
            field: "email".to_string(),
            message: "Must be at least 5 characters".to_string(),
            code: 1002,
        });
    }
    
    Ok(())
}
```

### Error Conversion and Chaining
```rust
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;
use std::io;

// Multiple error types
#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
    Custom(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::Custom(msg) => write!(f, "Application error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            AppError::Parse(e) => Some(e),
            AppError::Custom(_) => None,
        }
    }
}

// From trait implementations for automatic conversion
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::Parse(error)
    }
}

// Function using multiple error types with ? operator
fn read_number_from_file(filename: &str) -> Result<i32, AppError> {
    let mut file = File::open(filename)?;  // io::Error automatically converted
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;   // io::Error automatically converted
    
    let number: i32 = contents.trim().parse()?;  // ParseIntError automatically converted
    
    if number < 0 {
        return Err(AppError::Custom("Number must be positive".to_string()));
    }
    
    Ok(number)
}

// Using the anyhow crate for easier error handling
// Cargo.toml: anyhow = "1.0"
use anyhow::{Context, Result};

fn read_user_from_file(filename: &str) -> Result<String> {
    std::fs::read_to_string(filename)
        .with_context(|| format!("Failed to read user data from {}", filename))
}

fn parse_user_age(age_str: &str) -> Result<u32> {
    age_str.parse()
        .with_context(|| format!("Failed to parse age: '{}'", age_str))
}
```

### Error Handling Strategies
```rust
// Strategy 1: Fail fast and bubble up
fn process_data(filename: &str) -> Result<Vec<i32>, AppError> {
    let content = std::fs::read_to_string(filename)?;
    let numbers: Result<Vec<i32>, _> = content
        .lines()
        .map(|line| line.trim().parse())
        .collect();
    
    Ok(numbers?)
}

// Strategy 2: Collect errors and continue processing
fn process_data_collect_errors(filename: &str) -> (Vec<i32>, Vec<String>) {
    let content = match std::fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => return (Vec::new(), vec![format!("Failed to read file: {}", e)]),
    };
    
    let mut numbers = Vec::new();
    let mut errors = Vec::new();
    
    for (line_num, line) in content.lines().enumerate() {
        match line.trim().parse::<i32>() {
            Ok(number) => numbers.push(number),
            Err(e) => errors.push(format!("Line {}: {}", line_num + 1, e)),
        }
    }
    
    (numbers, errors)
}

// Strategy 3: Default values for recoverable errors
fn get_config_value(key: &str) -> String {
    std::env::var(key)
        .unwrap_or_else(|_| "default_value".to_string())
}

// Strategy 4: Retry with backoff
use std::time::Duration;
use std::thread;

fn retry_operation<F, T, E>(mut operation: F, max_attempts: u32) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut attempts = 0;
    
    loop {
        attempts += 1;
        
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempts >= max_attempts {
                    return Err(e);
                }
                
                // Exponential backoff
                let delay = Duration::from_millis(100 * 2_u64.pow(attempts - 1));
                thread::sleep(delay);
            }
        }
    }
}

// Usage
fn unreliable_network_call() -> Result<String, io::Error> {
    // Simulated network call that might fail
    Ok("success".to_string())
}

fn main() {
    let result = retry_operation(|| unreliable_network_call(), 3);
    match result {
        Ok(data) => println!("Success: {}", data),
        Err(e) => println!("Failed after retries: {}", e),
    }
}
```

### Working with Option<T>
```rust
// Option basics
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some("Alice".to_string())
    } else {
        None
    }
}

// Option combinators
fn process_user_id(id_str: &str) -> Option<String> {
    id_str.parse::<u32>()     // Result<u32, ParseIntError>
        .ok()                 // Option<u32>
        .and_then(find_user)  // Option<String>
}

// Chaining operations with Option
fn get_user_initial(id_str: &str) -> Option<char> {
    id_str.parse::<u32>()
        .ok()
        .and_then(find_user)
        .and_then(|name| name.chars().next())
}

// Using unwrap_or for defaults
fn get_username_or_default(id: u32) -> String {
    find_user(id).unwrap_or_else(|| "Guest".to_string())
}

// Question mark operator with Option
fn get_first_char(text: Option<String>) -> Option<char> {
    let text = text?;  // Early return if None
    text.chars().next()
}

// Converting between Option and Result
fn option_to_result<T>(opt: Option<T>) -> Result<T, String> {
    opt.ok_or_else(|| "Value was None".to_string())
}

fn result_to_option<T, E>(res: Result<T, E>) -> Option<T> {
    res.ok()
}
```

### Testing Error Conditions
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_divide_by_zero() {
        let result = divide(10.0, 0.0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), MathError::DivisionByZero));
    }
    
    #[test]
    fn test_negative_sqrt() {
        let result = sqrt(-4.0);
        assert!(result.is_err());
        match result {
            Err(MathError::NegativeSquareRoot) => (),
            _ => panic!("Expected NegativeSquareRoot error"),
        }
    }
    
    #[test]
    fn test_valid_operations() {
        assert_eq!(divide(10.0, 2.0).unwrap(), 5.0);
        assert_eq!(sqrt(9.0).unwrap(), 3.0);
    }
    
    #[test]
    #[should_panic(expected = "Index 5 is out of bounds")]
    fn test_panic_condition() {
        let slice = &[1, 2, 3];
        get_element(slice, 5);  // This should panic
    }
    
    #[test]
    fn test_error_conversion() {
        let result = read_number_from_file("nonexistent.txt");
        assert!(result.is_err());
        
        match result.unwrap_err() {
            AppError::Io(_) => (),  // Expected
            _ => panic!("Expected IO error"),
        }
    }
}
```

### Real-World Error Handling
```rust
// Configuration loading with detailed errors
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    database_url: String,
    port: u16,
    max_connections: u32,
}

#[derive(Debug)]
enum ConfigError {
    FileNotFound(String),
    ParseError(String),
    ValidationError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::FileNotFound(path) => write!(f, "Config file not found: {}", path),
            ConfigError::ParseError(msg) => write!(f, "Failed to parse config: {}", msg),
            ConfigError::ValidationError(msg) => write!(f, "Invalid config: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
    let path = path.as_ref();
    let path_str = path.to_string_lossy();
    
    // Read file
    let content = std::fs::read_to_string(path)
        .map_err(|_| ConfigError::FileNotFound(path_str.to_string()))?;
    
    // Parse TOML
    let config: Config = toml::from_str(&content)
        .map_err(|e| ConfigError::ParseError(e.to_string()))?;
    
    // Validate
    if config.port == 0 {
        return Err(ConfigError::ValidationError("Port cannot be 0".to_string()));
    }
    
    if config.max_connections == 0 {
        return Err(ConfigError::ValidationError("Max connections cannot be 0".to_string()));
    }
    
    Ok(config)
}

// Database connection with error handling
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
enum DatabaseError {
    ConnectionFailed(String),
    QueryFailed(String),
    TransactionFailed(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            DatabaseError::QueryFailed(msg) => write!(f, "Query failed: {}", msg),
            DatabaseError::TransactionFailed(msg) => write!(f, "Transaction failed: {}", msg),
        }
    }
}

impl std::error::Error for DatabaseError {}

struct Database {
    connection_pool: Arc<Mutex<Vec<String>>>,  // Simplified
}

impl Database {
    fn new(url: &str) -> Result<Self, DatabaseError> {
        // Simulate connection
        if url.is_empty() {
            return Err(DatabaseError::ConnectionFailed("Empty URL".to_string()));
        }
        
        Ok(Database {
            connection_pool: Arc::new(Mutex::new(vec!["connection1".to_string()])),
        })
    }
    
    fn execute_query(&self, query: &str) -> Result<Vec<String>, DatabaseError> {
        if query.is_empty() {
            return Err(DatabaseError::QueryFailed("Empty query".to_string()));
        }
        
        // Simulate query execution
        Ok(vec!["result1".to_string(), "result2".to_string()])
    }
    
    fn execute_transaction<F>(&self, transaction: F) -> Result<(), DatabaseError>
    where
        F: FnOnce() -> Result<(), DatabaseError>,
    {
        // Begin transaction
        println!("Beginning transaction");
        
        match transaction() {
            Ok(()) => {
                println!("Committing transaction");
                Ok(())
            }
            Err(e) => {
                println!("Rolling back transaction");
                Err(DatabaseError::TransactionFailed(format!("Transaction rolled back: {}", e)))
            }
        }
    }
}

// Application combining all error types
#[derive(Debug)]
enum AppError {
    Config(ConfigError),
    Database(DatabaseError),
    Validation(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Config(e) => write!(f, "Configuration error: {}", e),
            AppError::Database(e) => write!(f, "Database error: {}", e),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Config(e) => Some(e),
            AppError::Database(e) => Some(e),
            AppError::Validation(_) => None,
        }
    }
}

impl From<ConfigError> for AppError {
    fn from(error: ConfigError) -> Self {
        AppError::Config(error)
    }
}

impl From<DatabaseError> for AppError {
    fn from(error: DatabaseError) -> Self {
        AppError::Database(error)
    }
}

// Main application function
fn run_application() -> Result<(), AppError> {
    // Load configuration
    let config = load_config("config.toml")?;
    
    // Connect to database
    let db = Database::new(&config.database_url)?;
    
    // Execute some queries
    let results = db.execute_query("SELECT * FROM users")?;
    println!("Query results: {:?}", results);
    
    // Execute transaction
    db.execute_transaction(|| {
        db.execute_query("INSERT INTO users VALUES (...)")?;
        db.execute_query("UPDATE users SET ...")?;
        Ok(())
    })?;
    
    Ok(())
}

fn main() {
    match run_application() {
        Ok(()) => println!("Application completed successfully"),
        Err(e) => {
            eprintln!("Application error: {}", e);
            
            // Print error chain
            let mut source = e.source();
            while let Some(err) = source {
                eprintln!("Caused by: {}", err);
                source = err.source();
            }
            
            std::process::exit(1);
        }
    }
}
```

## Best Practices Summary

### Error Type Design
```rust
// ✅ Good: Specific error types
#[derive(Debug)]
enum UserError {
    NotFound { id: u32 },
    InvalidEmail { email: String },
    PasswordTooWeak { requirements: String },
}

// ❌ Bad: Generic error
#[derive(Debug)]
enum Error {
    SomethingWentWrong,
}

// ✅ Good: Implement standard traits
impl fmt::Display for UserError { /* ... */ }
impl std::error::Error for UserError { /* ... */ }

// ✅ Good: Provide From implementations
impl From<std::io::Error> for UserError { /* ... */ }
```

### Error Handling Guidelines
```rust
// ✅ Use Result for recoverable errors
fn parse_user_input(input: &str) -> Result<User, ParseError> {
    // Can fail due to invalid input
}

// ✅ Use panic! for programming errors
fn get_array_element(arr: &[i32], index: usize) -> i32 {
    if index >= arr.len() {
        panic!("Index out of bounds: {} >= {}", index, arr.len());
    }
    arr[index]
}

// ✅ Use ? operator for error propagation
fn process_file(filename: &str) -> Result<String, io::Error> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content.to_uppercase())
}

// ✅ Provide context with errors
fn read_config() -> Result<Config, ConfigError> {
    std::fs::read_to_string("config.toml")
        .map_err(|e| ConfigError::FileNotFound(format!("config.toml: {}", e)))?;
    // ... rest of function
}
```

Official Chapter: https://doc.rust-lang.org/book/ch09-00-error-handling.html

---
*Completed: ✓*