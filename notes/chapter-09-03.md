# Chapter 9.3: To panic! or Not to panic!

## Key Takeaways

### Decision Framework
- **panic!**: For unrecoverable errors and programming bugs where continuing is unsafe
- **Result**: For expected, recoverable errors where the caller can decide how to handle them
- **Context Matters**: The same operation might warrant different approaches in different contexts
- **Caller Empowerment**: Result gives calling code choices; panic! removes all choices

### When to panic!
- **Programming Errors**: Bugs that should never happen in correct code
- **Invalid States**: When program state becomes logically impossible
- **Contract Violations**: When API preconditions are not met
- **Security Concerns**: When continuing could compromise safety or security

### When to Use Result
- **Expected Failures**: Operations that commonly fail in normal usage
- **User Input**: Parsing, validation, and processing user-provided data
- **External Dependencies**: Network, file system, and other unreliable resources
- **Recoverable Conditions**: When the program can handle the error gracefully

### Important Decision Factors

#### Context Consideration
- **Prototyping vs Production**: Different error handling standards
- **Library vs Application**: Libraries should generally prefer Result
- **Critical vs Non-critical**: Mission-critical code needs different handling
- **Performance vs Safety**: Balance between speed and robustness

### Programming Concepts Introduced
- **Error Handling Strategy**: Systematic approach to choosing error mechanisms
- **API Design Philosophy**: How error handling affects interface design
- **Risk Assessment**: Evaluating consequences of different error approaches
- **Defensive Programming**: Using types and validation to prevent errors

### Code Examples and Patterns

#### Examples and Prototyping - panic! is OK
```rust
use std::net::IpAddr;

fn main() {
    // In examples and prototypes, unwrap is acceptable
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    
    println!("Home IP: {}", home);
}

// Quick prototype function
fn quick_calculation(input: &str) -> i32 {
    // For prototyping, unwrap is fine
    input.parse().unwrap() * 2
}
```

#### Production Code - Use Result
```rust
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug)]
enum ConfigError {
    InvalidIp(String),
    MissingValue,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::InvalidIp(ip) => write!(f, "Invalid IP address: {}", ip),
            ConfigError::MissingValue => write!(f, "Missing configuration value"),
        }
    }
}

impl std::error::Error for ConfigError {}

fn parse_ip_from_config(config_value: Option<&str>) -> Result<IpAddr, ConfigError> {
    let ip_str = config_value.ok_or(ConfigError::MissingValue)?;
    
    IpAddr::from_str(ip_str)
        .map_err(|_| ConfigError::InvalidIp(ip_str.to_string()))
}

fn main() {
    // Production code handles errors gracefully
    match parse_ip_from_config(Some("127.0.0.1")) {
        Ok(ip) => println!("Configured IP: {}", ip),
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            std::process::exit(1);
        }
    }
}
```

#### When You Have More Information Than the Compiler
```rust
use std::net::IpAddr;

fn main() {
    // You know this will always be valid, but compiler doesn't
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should always be valid");
    
    // Another example with array access
    let values = vec![1, 2, 3, 4, 5];
    let index = 2; // You know this is always valid for this vec
    let value = values[index]; // This is OK because you control both vec and index
    
    println!("Home: {}, Value: {}", home, value);
}

// Better approach: make the invariant explicit
fn get_middle_value(values: &[i32]) -> Option<i32> {
    if values.len() >= 3 {
        Some(values[values.len() / 2])
    } else {
        None
    }
}
```

#### Guidelines for Error Propagation
```rust
use std::fs::File;
use std::io::{self, Read};

// Library function - should return Result
pub fn read_config_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Application function - can choose how to handle errors
fn load_application_config() {
    match read_config_file("app.conf") {
        Ok(config) => {
            println!("Loaded config: {}", config);
            // Continue with application startup
        }
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            eprintln!("Using default configuration");
            // Use default config or exit
        }
    }
}

// Critical system function - might need to panic
fn load_security_config() {
    let config = read_config_file("security.conf")
        .expect("Security configuration is required for safe operation");
    
    // If security config can't be loaded, it's better to crash
    // than run with potentially unsafe defaults
}
```

#### Using Types for Validation
```rust
// Custom type that enforces validation
pub struct GuessNumber {
    value: i32,
}

impl GuessNumber {
    pub fn new(value: i32) -> Result<GuessNumber, String> {
        if value < 1 || value > 100 {
            Err(format!("Guess value must be between 1 and 100, got {}", value))
        } else {
            Ok(GuessNumber { value })
        }
    }
    
    pub fn value(&self) -> i32 {
        self.value // Guaranteed to be valid
    }
}

// Usage
fn play_guessing_game() {
    let guess_str = "50";
    
    // Parse and validate in one step
    let guess = match guess_str.parse::<i32>() {
        Ok(num) => match GuessNumber::new(num) {
            Ok(guess) => guess,
            Err(e) => {
                println!("Invalid guess: {}", e);
                return;
            }
        },
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };
    
    println!("Your guess: {}", guess.value());
    // guess.value() is guaranteed to be between 1 and 100
}

// Alternative with chaining
fn play_guessing_game_v2() {
    let guess_str = "50";
    
    let guess = guess_str
        .parse::<i32>()
        .map_err(|_| "Please enter a valid number".to_string())
        .and_then(|num| GuessNumber::new(num));
    
    match guess {
        Ok(g) => println!("Your guess: {}", g.value()),
        Err(e) => println!("Error: {}", e),
    }
}
```

#### Contract Violations - When to panic!
```rust
// Function with preconditions that should be enforced
fn calculate_average(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        panic!("Cannot calculate average of empty slice");
    }
    
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}

// Better approach: use type system to prevent invalid input
fn calculate_average_safe(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        None
    } else {
        let sum: f64 = numbers.iter().sum();
        Some(sum / numbers.len() as f64)
    }
}

// Even better: use NonEmpty type (would need external crate)
// fn calculate_average_guaranteed(numbers: NonEmpty<&[f64]>) -> f64 {
//     let sum: f64 = numbers.iter().sum();
//     sum / numbers.len() as f64
// }

fn main() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let empty_data: Vec<f64> = vec![];
    
    // This will work
    println!("Average: {}", calculate_average(&data));
    
    // This will panic - demonstrates contract violation
    // println!("Average: {}", calculate_average(&empty_data));
    
    // Safe version handles empty case gracefully
    match calculate_average_safe(&empty_data) {
        Some(avg) => println!("Average: {}", avg),
        None => println!("Cannot calculate average of empty data"),
    }
}
```

#### Testing - panic! is Expected
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
    fn test_successful_division() {
        let result = divide(10.0, 2.0).unwrap(); // OK to unwrap in tests
        assert_eq!(result, 5.0);
    }
    
    #[test]
    fn test_division_by_zero() {
        let result = divide(10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Division by zero");
    }
    
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_panic_on_division_by_zero() {
        let _result = divide(10.0, 0.0).expect("Division by zero");
    }
}
```

#### Security and Safety Concerns
```rust
use std::collections::HashMap;

// Authentication service - security critical
struct AuthService {
    users: HashMap<String, String>, // username -> password_hash
}

impl AuthService {
    fn authenticate(&self, username: &str, password: &str) -> Result<bool, String> {
        let stored_hash = self.users.get(username)
            .ok_or_else(|| "User not found".to_string())?;
        
        // In real code, use proper password hashing
        if stored_hash == password {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    // This function has security implications
    fn grant_admin_access(&mut self, username: &str) {
        if !self.users.contains_key(username) {
            // This is a serious error - granting admin to non-existent user
            panic!("Attempted to grant admin access to non-existent user: {}", username);
        }
        
        // Grant admin access logic here
        println!("Granted admin access to {}", username);
    }
}

// Alternative: use Result even for security issues
impl AuthService {
    fn grant_admin_access_safe(&mut self, username: &str) -> Result<(), String> {
        if !self.users.contains_key(username) {
            return Err(format!("User {} does not exist", username));
        }
        
        // Grant admin access logic here
        println!("Granted admin access to {}", username);
        Ok(())
    }
}
```

#### API Design Considerations
```rust
// Library API - should give callers options
pub fn parse_config_file(path: &str) -> Result<Config, ConfigError> {
    // Implementation that returns Result
    // Callers can decide how to handle errors
    todo!()
}

// Application code - can choose to panic
fn main() {
    // Application can decide: recover, use defaults, or exit
    let config = match parse_config_file("app.conf") {
        Ok(config) => config,
        Err(ConfigError::FileNotFound) => {
            println!("Config file not found, using defaults");
            Config::default()
        }
        Err(ConfigError::ParseError(msg)) => {
            eprintln!("Invalid config format: {}", msg);
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            std::process::exit(1);
        }
    };
    
    // Or application can choose to panic if config is critical
    // let config = parse_config_file("app.conf")
    //     .expect("Configuration file is required");
}

#[derive(Debug)]
enum ConfigError {
    FileNotFound,
    ParseError(String),
}

struct Config {
    // config fields
}

impl Config {
    fn default() -> Self {
        Config {
            // default values
        }
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound => write!(f, "Configuration file not found"),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}
```

#### Performance vs Safety Trade-offs
```rust
// High-performance version - assumes valid input
fn fast_array_access(arr: &[i32], index: usize) -> i32 {
    // In performance-critical code, you might choose to panic
    // rather than return Option, if you can guarantee valid indices
    arr[index]
}

// Safe version - handles invalid input gracefully
fn safe_array_access(arr: &[i32], index: usize) -> Option<i32> {
    arr.get(index).copied()
}

// Hybrid approach - debug assertions
fn hybrid_array_access(arr: &[i32], index: usize) -> i32 {
    debug_assert!(index < arr.len(), "Index {} out of bounds for array of length {}", index, arr.len());
    arr[index] // Panic in debug, undefined behavior in release (unsafe!)
}

// Better hybrid - bounds check in debug, unsafe in release
fn optimized_array_access(arr: &[i32], index: usize) -> i32 {
    if cfg!(debug_assertions) {
        arr[index] // Bounds checked
    } else {
        // Only use this if you can GUARANTEE index is valid
        unsafe { *arr.get_unchecked(index) }
    }
}
```

### Decision Guidelines

#### Use panic! when:
- **Examples and prototypes** where quick iteration is more important than robustness
- **Tests** where failure should stop execution immediately
- **Contract violations** where continuing would be unsafe or meaningless
- **Security issues** where graceful handling might mask serious problems
- **Impossible states** that indicate programming errors

#### Use Result when:
- **Expected failures** like file not found, network timeouts, parse errors
- **User input validation** where users might provide invalid data
- **Library APIs** where callers should decide how to handle errors
- **Recoverable errors** where the program can continue with alternative approaches
- **External dependencies** that might fail for reasons outside your control

### Integration with Previous Chapters
- Builds on panic! concepts from Chapter 9.1
- Uses Result patterns from Chapter 9.2
- Applies to real-world scenarios using previous data structures
- Demonstrates proper error handling in complete applications

### Community Conventions and Idioms
- Libraries should prefer Result, applications can choose panic!
- Document panic conditions in function documentation
- Use expect() with descriptive messages instead of unwrap()
- Consider using custom error types for better error handling
- Use debug_assert! for expensive checks that only run in debug builds

### Error Handling Strategy Framework
1. **Identify error types**: Programming errors vs expected failures
2. **Consider the caller**: Library vs application code
3. **Evaluate consequences**: Can the program continue safely?
4. **Choose appropriate mechanism**: panic! vs Result vs Option
5. **Document decisions**: Make error behavior clear to users

### Personal Notes
- The choice between panic! and Result is context-dependent
- Library code should almost always use Result
- Application code has more flexibility to panic on critical errors
- Good error handling makes programs more robust and user-friendly
- The Rust type system helps make error handling explicit and safe

Official Chapter: https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html

---
*Completed: ✓*