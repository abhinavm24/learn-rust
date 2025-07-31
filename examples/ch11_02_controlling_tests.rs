//! # Chapter 11.2: Controlling How Tests Are Run
//! 
//! This example demonstrates:
//! - Running tests in parallel vs sequentially
//! - Showing function output during tests
//! - Running a subset of tests by name
//! - Ignoring tests unless specifically requested
//! - Test organization: unit tests vs integration tests
//! - Testing private functions

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 11.2", "Controlling How Tests Are Run");

    println!("=== Test Control Commands ===");
    println!("cargo test                    # Run all tests");
    println!("cargo test -- --help          # Show test options");
    println!("cargo test -- --nocapture     # Show println! output");
    println!("cargo test -- --test-threads=1 # Run tests sequentially");
    println!("cargo test test_name          # Run specific test");
    println!("cargo test add                # Run tests with 'add' in name");
    println!("cargo test -- --ignored       # Run only ignored tests");
    println!("cargo test -- --include-ignored # Run all tests including ignored");
    println!();
    
    println!("=== Example Functions (see tests below) ===");
    
    // Demonstrate the functions we'll test
    println!("add_two(3) = {}", add_two(3));
    println!("multiply(4, 5) = {}", multiply(4, 5));
    println!("greeting('Alice') = {}", greeting("Alice"));
    
    // This will print during normal execution but not during tests (unless --nocapture)
    println!("This output is visible in main()");
}

// Public functions for testing
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// Private function that we can still test (unit tests are in the same module)
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// Function that produces output during testing
pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

// Slow function for demonstrating parallel vs sequential execution
pub fn slow_function() -> i32 {
    std::thread::sleep(std::time::Duration::from_millis(100));
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Basic Tests ===
    
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);  // This will fail!
    }
    
    // === Tests for Demonstrating Parallel Execution ===
    
    #[test]
    fn test_slow_function_1() {
        let start = std::time::Instant::now();
        let result = slow_function();
        let duration = start.elapsed();
        
        println!("Test 1 took {:?}", duration);
        assert_eq!(result, 42);
    }
    
    #[test]
    fn test_slow_function_2() {
        let start = std::time::Instant::now();
        let result = slow_function();
        let duration = start.elapsed();
        
        println!("Test 2 took {:?}", duration);
        assert_eq!(result, 42);
    }
    
    #[test]
    fn test_slow_function_3() {
        let start = std::time::Instant::now();
        let result = slow_function();
        let duration = start.elapsed();
        
        println!("Test 3 took {:?}", duration);
        assert_eq!(result, 42);
    }
    
    // === Tests with Similar Names for Filtering ===
    
    #[test]
    fn add_test_positive_numbers() {
        assert_eq!(add_two(3), 5);
    }
    
    #[test]
    fn add_test_negative_numbers() {
        assert_eq!(add_two(-2), 0);
    }
    
    #[test]
    fn add_test_zero() {
        assert_eq!(add_two(0), 2);
    }
    
    #[test]
    fn multiply_test_positive() {
        assert_eq!(multiply(3, 4), 12);
    }
    
    #[test]
    fn multiply_test_negative() {
        assert_eq!(multiply(-2, 3), -6);
    }
    
    #[test]
    fn multiply_test_zero() {
        assert_eq!(multiply(0, 5), 0);
    }
    
    // === Tests for Greeting Function ===
    
    #[test]
    fn greeting_test_simple() {
        let result = greeting("Alice");
        assert!(result.contains("Alice"));
    }
    
    #[test]
    fn greeting_test_format() {
        let result = greeting("Bob");
        assert_eq!(result, "Hello Bob!");
    }
    
    // === Testing Private Functions ===
    
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
    
    #[test]
    fn internal_negative() {
        assert_eq!(0, internal_adder(-2, 2));
    }
    
    // === Ignored Tests ===
    
    #[test]
    #[ignore]
    fn expensive_test() {
        // This test is ignored by default because it's expensive to run
        println!("Running expensive test...");
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(1 + 1, 2);
    }
    
    #[test]
    #[ignore]
    fn another_ignored_test() {
        println!("This test is also ignored");
        assert!(true);
    }
    
    // === Tests that Demonstrate Output ===
    
    #[test]
    fn test_with_output() {
        println!("This is test output - visible with --nocapture");
        println!("Testing value: {}", 42);
        
        let result = add_two(5);
        println!("Result: {}", result);
        
        assert_eq!(result, 7);
    }
    
    #[test]
    fn test_with_debug_output() {
        let data = vec![1, 2, 3, 4, 5];
        println!("Debug data: {:?}", data);
        
        let sum: i32 = data.iter().sum();
        println!("Sum: {}", sum);
        
        assert_eq!(sum, 15);
    }
    
    // === Tests for File I/O (would require actual files in real project) ===
    
    #[test]
    fn test_file_operations() {
        // In a real project, this might test file operations
        // For now, just demonstrate the concept
        let filename = "test_file.txt";
        println!("Would test operations on file: {}", filename);
        
        // Simulated file operation result
        let file_exists = false; // In reality, you'd check if file exists
        assert_eq!(file_exists, false);
    }
    
    // === Performance Tests ===
    
    #[test]
    fn performance_test() {
        let start = std::time::Instant::now();
        
        // Simulate some work
        let mut sum = 0;
        for i in 0..1000 {
            sum += i;
        }
        
        let duration = start.elapsed();
        println!("Performance test completed in {:?}", duration);
        
        assert_eq!(sum, 499500);
        assert!(duration.as_millis() < 100); // Should complete quickly
    }
    
    // === Tests with Setup and Cleanup ===
    
    fn setup() -> Vec<i32> {
        println!("Setting up test data");
        vec![1, 2, 3, 4, 5]
    }
    
    fn cleanup(data: Vec<i32>) {
        println!("Cleaning up test data with {} items", data.len());
        // In real tests, this might clean up files, database connections, etc.
    }
    
    #[test]
    fn test_with_setup_cleanup() {
        let data = setup();
        
        // Test logic
        assert_eq!(data.len(), 5);
        assert_eq!(data[0], 1);
        assert_eq!(data[4], 5);
        
        cleanup(data);
    }
    
    // === Conditional Tests ===
    
    #[test]
    #[cfg(target_os = "linux")]
    fn linux_only_test() {
        println!("This test only runs on Linux");
        assert!(true);
    }
    
    #[test]
    #[cfg(target_os = "windows")]
    fn windows_only_test() {
        println!("This test only runs on Windows");
        assert!(true);
    }
    
    #[test]
    #[cfg(debug_assertions)]
    fn debug_only_test() {
        println!("This test only runs in debug builds");
        assert!(true);
    }
    
    // === Tests for Error Conditions ===
    
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("Cannot divide by zero".to_string())
        } else {
            Ok(a / b)
        }
    }
    
    #[test]
    fn test_divide_success() {
        let result = divide(10.0, 2.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);
    }
    
    #[test]
    fn test_divide_by_zero() {
        let result = divide(10.0, 0.0);
        assert!(result.is_err());
        
        if let Err(msg) = result {
            assert_eq!(msg, "Cannot divide by zero");
        }
    }
    
    // === Stress Tests ===
    
    #[test]
    #[ignore] // Ignored because it's a stress test
    fn stress_test_large_vector() {
        println!("Creating large vector for stress test");
        let large_vec: Vec<i32> = (0..1_000_000).collect();
        
        let sum: i32 = large_vec.iter().sum();
        println!("Sum of large vector: {}", sum);
        
        assert_eq!(sum, 499999500000i32);
    }
    
    // === Tests with Mock Data ===
    
    struct MockDatabase {
        users: Vec<String>,
    }
    
    impl MockDatabase {
        fn new() -> Self {
            MockDatabase {
                users: vec!["Alice".to_string(), "Bob".to_string()],
            }
        }
        
        fn get_user_count(&self) -> usize {
            self.users.len()
        }
        
        fn add_user(&mut self, name: String) {
            self.users.push(name);
        }
    }
    
    #[test]
    fn test_mock_database() {
        let mut db = MockDatabase::new();
        
        assert_eq!(db.get_user_count(), 2);
        
        db.add_user("Charlie".to_string());
        assert_eq!(db.get_user_count(), 3);
    }
}

// === Integration Test Examples ===
// Note: In a real project, integration tests would be in a separate `tests/` directory

pub fn public_api_function(input: &str) -> String {
    format!("Processed: {}", input)
}

pub struct PublicStruct {
    pub data: String,
}

impl PublicStruct {
    pub fn new(data: String) -> Self {
        PublicStruct { data }
    }
    
    pub fn process(&self) -> String {
        format!("Processing: {}", self.data)
    }
}

#[cfg(test)]
mod integration_style_tests {
    use super::*;

    // These tests only use the public API, similar to how integration tests work
    
    #[test]
    fn test_public_api() {
        let result = public_api_function("test input");
        assert_eq!(result, "Processed: test input");
    }
    
    #[test]
    fn test_public_struct() {
        let instance = PublicStruct::new("test data".to_string());
        assert_eq!(instance.data, "test data");
        
        let processed = instance.process();
        assert_eq!(processed, "Processing: test data");
    }
}

// === Benchmark-style Tests (would use external crate like criterion in real project) ===

#[cfg(test)]
mod benchmark_style_tests {
    use super::*;
    
    #[test]
    #[ignore] // Ignored because it's a benchmark
    fn benchmark_add_function() {
        let iterations = 1_000_000;
        let start = std::time::Instant::now();
        
        for i in 0..iterations {
            let _ = add_two(i as i32);
        }
        
        let duration = start.elapsed();
        println!("add_two() {} iterations took {:?}", iterations, duration);
        println!("Average time per call: {:?}", duration / iterations);
        
        // Just assert it completed (in real benchmarks, you'd compare against baselines)
        assert!(duration.as_millis() < 1000);
    }
}