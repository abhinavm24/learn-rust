//! # Chapter 9.3: To panic! or Not to panic!
//! 
//! This example demonstrates:
//! - When to use panic! vs Result
//! - Guidelines for error handling in different contexts
//! - Examples, prototype code, and tests
//! - Cases where you have more information than the compiler
//! - Creating custom types for validation

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 9.3", "To panic! or Not to panic!");

    println!("=== When panic! is Appropriate ===");
    when_panic_is_appropriate();
    
    println!("\n=== When to Return Result ===");
    when_to_return_result();
    
    println!("\n=== Guidelines for Library Code ===");
    library_guidelines();
    
    println!("\n=== Custom Types for Validation ===");
    custom_validation_types();
    
    println!("\n=== Real-world Examples ===");
    real_world_examples();
}

fn when_panic_is_appropriate() {
    println!("panic! is appropriate when:");
    println!("1. Examples, prototype code, and tests");
    println!("2. When you have more information than the compiler");
    println!("3. When failure indicates a programming error");
    println!();
    
    // Example 1: In tests, panic is often desired
    println!("=== Example: Test Code ===");
    fn test_addition() {
        let result = 2 + 2;
        assert_eq!(result, 4); // panic! if assertion fails - this is desired in tests
        println!("Test passed: 2 + 2 = {}", result);
    }
    test_addition();
    
    // Example 2: When you know more than the compiler
    println!("\n=== Example: You Know More Than the Compiler ===");
    use std::net::IpAddr;
    
    // This IP address is hardcoded and known to be valid
    let home: IpAddr = "127.0.0.1".parse().expect("Hardcoded IP address should be valid");
    println!("Home IP: {}", home);
    
    // Example 3: Programming logic guarantees success
    println!("\n=== Example: Logic Guarantees Success ===");
    let numbers = vec![1, 2, 3, 4, 5];
    // We know the vector has elements, so unwrap is safe here
    let first = numbers.first().expect("Vector should have at least one element");
    println!("First number: {}", first);
    
    // Example 4: Contract violations (programming errors)
    println!("\n=== Example: Contract Violations ===");
    fn divide_positive_numbers(a: u32, b: u32) -> u32 {
        if b == 0 {
            panic!("Division by zero is a programming error in this context");
        }
        a / b
    }
    
    let result = divide_positive_numbers(10, 2);
    println!("10 / 2 = {}", result);
    
    // This would panic - uncomment to see:
    // divide_positive_numbers(10, 0);
}

fn when_to_return_result() {
    println!("Return Result when:");
    println!("1. Failure is expected and recoverable");
    println!("2. The caller should decide how to handle the error");
    println!("3. In library code where you can't assume the caller's needs");
    println!("4. When failure doesn't indicate a bug");
    println!();
    
    // Example 1: File operations (expected to potentially fail)
    use std::fs::File;
    use std::io::Read;
    
    fn read_config_file(filename: &str) -> Result<String, std::io::Error> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
    
    match read_config_file("config.txt") {
        Ok(contents) => println!("Config loaded: {} bytes", contents.len()),
        Err(e) => println!("Could not load config (this is expected): {}", e),
    }
    
    // Example 2: User input validation
    fn parse_age(input: &str) -> Result<u8, String> {
        let age: u8 = input.parse().map_err(|_| "Invalid number".to_string())?;
        if age > 150 {
            Err("Age seems unrealistic".to_string())
        } else {
            Ok(age)
        }
    }
    
    match parse_age("25") {
        Ok(age) => println!("Valid age: {}", age),
        Err(e) => println!("Invalid age: {}", e),
    }
    
    match parse_age("200") {
        Ok(age) => println!("Valid age: {}", age),
        Err(e) => println!("Invalid age: {}", e),
    }
}

fn library_guidelines() {
    println!("=== Guidelines for Library Code ===");
    println!();
    println!("In library code, prefer Result over panic! because:");
    println!("- Callers can decide how to handle errors");
    println!("- Libraries shouldn't crash the entire program");
    println!("- Different applications have different error handling needs");
    println!();
    
    // Example: A math library function
    fn safe_divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
        if denominator == 0.0 {
            Err("Division by zero")
        } else if denominator.is_nan() || numerator.is_nan() {
            Err("Cannot divide with NaN values")
        } else {
            Ok(numerator / denominator)
        }
    }
    
    // Callers can decide how to handle the error
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("Library division result: {}", result),
        Err(e) => println!("Library division error: {}", e),
    }
    
    // Application-specific error handling
    let result = safe_divide(10.0, 0.0).unwrap_or(f64::INFINITY);
    println!("App-specific handling (use infinity): {}", result);
    
    // Another application might handle it differently
    match safe_divide(5.0, 0.0) {
        Ok(r) => println!("Result: {}", r),
        Err(_) => {
            println!("Division failed, using default calculation");
            println!("Using alternative: {}", 5.0 * 2.0);
        }
    }
}

fn custom_validation_types() {
    println!("=== Custom Types for Validation ===");
    println!("Use custom types to make invalid states unrepresentable");
    println!();
    
    // Custom type that ensures validation
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
    
    // Valid guess
    let guess = Guess::new(50);
    println!("Valid guess: {}", guess.value());
    
    // This would panic - uncomment to see:
    // let invalid_guess = Guess::new(200);
    
    // Alternative: validation that returns Result
    #[derive(Debug)]
    pub struct ValidatedEmail {
        email: String,
    }
    
    impl ValidatedEmail {
        pub fn new(email: String) -> Result<ValidatedEmail, &'static str> {
            if email.contains('@') && email.contains('.') {
                Ok(ValidatedEmail { email })
            } else {
                Err("Invalid email format")
            }
        }
        
        pub fn as_str(&self) -> &str {
            &self.email
        }
    }
    
    match ValidatedEmail::new("user@example.com".to_string()) {
        Ok(email) => println!("Valid email: {}", email.as_str()),
        Err(e) => println!("Email validation error: {}", e),
    }
    
    match ValidatedEmail::new("invalid-email".to_string()) {
        Ok(email) => println!("Valid email: {}", email.as_str()),
        Err(e) => println!("Email validation error: {}", e),
    }
}

fn real_world_examples() {
    println!("=== Real-world Decision Examples ===");
    println!();
    
    // Example 1: Configuration loading
    println!("1. Configuration Loading:");
    println!("   - Missing config file: Return Result (user might want defaults)");
    println!("   - Malformed config syntax: Return Result (user should fix it)");
    println!("   - Programming error in config parser: panic!");
    
    fn load_config(path: &str) -> Result<Config, ConfigError> {
        // This would return Result because file might not exist (expected)
        Config::from_file(path)
    }
    
    #[derive(Debug)]
    struct Config {
        database_url: String,
        port: u16,
    }
    
    #[derive(Debug)]
    enum ConfigError {
        FileNotFound,
        ParseError(String),
    }
    
    impl Config {
        fn from_file(_path: &str) -> Result<Config, ConfigError> {
            // Simulating file not found
            Err(ConfigError::FileNotFound)
        }
    }
    
    match load_config("app.toml") {
        Ok(config) => println!("Config loaded: {:?}", config),
        Err(ConfigError::FileNotFound) => {
            println!("Config not found, using defaults");
            let default_config = Config {
                database_url: "localhost:5432".to_string(),
                port: 8080,
            };
            println!("Using default config: {:?}", default_config);
        },
        Err(ConfigError::ParseError(msg)) => {
            println!("Config parse error: {}", msg);
        }
    }
    
    // Example 2: Web server request handling
    println!("\n2. Web Server Request Handling:");
    println!("   - Invalid user input: Return Result (send error response)");
    println!("   - Database connection failure: Return Result (might retry)");
    println!("   - Out of memory: panic! (can't continue safely)");
    
    fn handle_user_request(input: &str) -> Result<String, &'static str> {
        if input.is_empty() {
            Err("Empty input") // Return error, don't crash server
        } else if input.len() > 1000 {
            Err("Input too long") // Return error, don't crash server  
        } else {
            Ok(format!("Processed: {}", input))
        }
    }
    
    match handle_user_request("") {
        Ok(response) => println!("Response: {}", response),
        Err(e) => println!("Request error (send 400 response): {}", e),
    }
    
    match handle_user_request("valid input") {
        Ok(response) => println!("Response: {}", response),
        Err(e) => println!("Request error: {}", e),
    }
    
    // Example 3: Array bounds checking
    println!("\n3. Array Access:");
    println!("   - User-provided index: Return Option (might be out of bounds)");
    println!("   - Compiler-verified index: Use direct access (panic on bug)");
    
    fn get_user_selected_item(items: &[String], user_index: usize) -> Option<&String> {
        items.get(user_index) // Returns None if out of bounds
    }
    
    fn get_first_item(items: &[String]) -> &String {
        &items[0] // Panic if empty - this would be a programming error
    }
    
    let items = vec!["apple".to_string(), "banana".to_string(), "orange".to_string()];
    
    match get_user_selected_item(&items, 5) {
        Some(item) => println!("User selected: {}", item),
        None => println!("Invalid selection, please try again"),
    }
    
    if !items.is_empty() {
        let first = get_first_item(&items);
        println!("First item: {}", first);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panic_in_tests_is_ok() {
        // In tests, panic! is expected and desirable for assertion failures
        assert_eq!(2 + 2, 4);
        
        // This would panic the test, which is the desired behavior:
        // assert_eq!(2 + 2, 5);
    }

    #[test]
    fn test_result_handling() {
        fn might_fail(should_fail: bool) -> Result<i32, &'static str> {
            if should_fail {
                Err("Operation failed")
            } else {
                Ok(42)
            }
        }
        
        assert_eq!(might_fail(false).unwrap(), 42);
        assert!(might_fail(true).is_err());
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn test_guess_panic_on_invalid() {
        // This test expects a panic - that's the desired behavior
        let _guess = custom_validation_types::Guess::new(200);
    }
}

// Helper module for the custom validation example
mod custom_validation_types {
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
}