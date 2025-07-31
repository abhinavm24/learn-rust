//! # Chapter 11.3: Test Organization
//! 
//! This example demonstrates how to organize tests in Rust:
//! - Unit tests vs Integration tests
//! - Test module organization
//! - Testing private vs public APIs
//! - Common test utilities
//! 
//! Run this example with: `cargo run --example ch11_03_test_organization`
//! Run tests with: `cargo test --example ch11_03_test_organization`

use rust_book_examples::print_chapter_header;

// === MAIN EXAMPLE CODE ===

/// A simple calculator struct to demonstrate testing
#[derive(Debug, Clone)]
pub struct Calculator {
    value: i32,
}

impl Calculator {
    /// Creates a new calculator with initial value of 0
    pub fn new() -> Self {
        Calculator { value: 0 }
    }

    /// Creates a calculator with a specific initial value
    pub fn with_value(value: i32) -> Self {
        Calculator { value }
    }

    /// Adds a number to the current value
    pub fn add(&mut self, n: i32) -> &mut Self {
        self.value += n;
        self
    }

    /// Subtracts a number from the current value
    pub fn subtract(&mut self, n: i32) -> &mut Self {
        self.value -= n;
        self
    }

    /// Multiplies the current value by a number
    pub fn multiply(&mut self, n: i32) -> &mut Self {
        self.value *= n;
        self
    }

    /// Divides the current value by a number
    /// Returns None if dividing by zero
    pub fn divide(&mut self, n: i32) -> Option<&mut Self> {
        if n == 0 {
            None
        } else {
            self.value /= n;
            Some(self)
        }
    }

    /// Gets the current value
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Resets the calculator to zero
    pub fn reset(&mut self) -> &mut Self {
        self.value = 0;
        self
    }

    /// Private helper function for internal calculations
    fn internal_multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }
}

/// Public utility function for basic addition
pub fn add_two(a: i32) -> i32 {
    a + 2
}

/// Private utility function - only accessible in unit tests
fn internal_helper(x: i32) -> i32 {
    x * 2 + 1
}

fn main() {
    print_chapter_header("Chapter 11.3", "Test Organization");

    println!("This example demonstrates different types of tests in Rust:");
    println!();

    // Example usage of Calculator
    let mut calc = Calculator::new();
    calc.add(10).multiply(2).subtract(5);
    println!("Calculator result: {}", calc.value());

    // Example of public function
    println!("add_two(5) = {}", add_two(5));

    println!();
    println!("Test organization includes:");
    println!("• Unit tests - test implementation details and private functions");
    println!("• Integration tests - test public API from external perspective");
    println!("• Common test utilities - shared setup and helper functions");
    println!();
    println!("Run 'cargo test --example ch11_03_test_organization' to see tests in action!");
}

// === UNIT TESTS ===
// These tests go in the same file as the code they're testing
// They can access private functions and implementation details

#[cfg(test)]
mod unit_tests {
    use super::*;

    /// Test suite for Calculator struct
    mod calculator_tests {
        use super::*;

        #[test]
        fn test_new_calculator() {
            let calc = Calculator::new();
            assert_eq!(calc.value(), 0);
        }

        #[test]
        fn test_calculator_with_value() {
            let calc = Calculator::with_value(42);
            assert_eq!(calc.value(), 42);
        }

        #[test]
        fn test_basic_operations() {
            let mut calc = Calculator::new();
            
            calc.add(10);
            assert_eq!(calc.value(), 10);
            
            calc.subtract(3);
            assert_eq!(calc.value(), 7);
            
            calc.multiply(2);
            assert_eq!(calc.value(), 14);
        }

        #[test]
        fn test_method_chaining() {
            let mut calc = Calculator::new();
            let result = calc.add(5).multiply(3).subtract(2).value();
            assert_eq!(result, 13); // (0 + 5) * 3 - 2 = 13
        }

        #[test]
        fn test_division_success() {
            let mut calc = Calculator::with_value(20);
            let result = calc.divide(4);
            assert!(result.is_some());
            assert_eq!(calc.value(), 5);
        }

        #[test]
        fn test_division_by_zero() {
            let mut calc = Calculator::with_value(20);
            let result = calc.divide(0);
            assert!(result.is_none());
            assert_eq!(calc.value(), 20); // Value unchanged
        }

        #[test]
        fn test_reset() {
            let mut calc = Calculator::with_value(100);
            calc.reset();
            assert_eq!(calc.value(), 0);
        }

        // Unit tests can access private functions!
        #[test]
        fn test_internal_multiply() {
            let calc = Calculator::new();
            let result = calc.internal_multiply(6, 7);
            assert_eq!(result, 42);
        }
    }

    /// Tests for utility functions
    mod utility_tests {
        use super::*;

        #[test]
        fn test_add_two() {
            assert_eq!(add_two(2), 4);
            assert_eq!(add_two(-1), 1);
            assert_eq!(add_two(0), 2);
        }

        // Unit tests can test private functions!
        #[test]
        fn test_internal_helper() {
            assert_eq!(internal_helper(5), 11); // 5 * 2 + 1 = 11
            assert_eq!(internal_helper(0), 1);  // 0 * 2 + 1 = 1
            assert_eq!(internal_helper(-2), -3); // -2 * 2 + 1 = -3
        }
    }

    /// Common test utilities used across multiple test modules
    mod test_utils {
        use super::*;

        /// Helper function to create a calculator with a sequence of operations
        pub fn setup_calculator_with_operations() -> Calculator {
            let mut calc = Calculator::new();
            calc.add(10).multiply(2).subtract(5);
            calc
        }

        /// Helper function to assert calculator value with custom message
        pub fn assert_calculator_value(calc: &Calculator, expected: i32, context: &str) {
            assert_eq!(calc.value(), expected, "Failed in context: {}", context);
        }
    }

    /// Tests using common utilities
    mod integration_style_tests {
        use super::*;
        use test_utils::*;

        #[test]
        fn test_complex_calculation_sequence() {
            let calc = setup_calculator_with_operations();
            assert_calculator_value(&calc, 15, "complex sequence");
        }

        #[test]
        fn test_calculator_reuse() {
            let mut calc = setup_calculator_with_operations();
            calc.divide(3);
            assert_calculator_value(&calc, 5, "after division by 3");
        }
    }
}

// === DOCUMENTATION TESTS ===
// These tests are embedded in documentation comments

/// Adds two numbers together.
/// 
/// # Examples
/// 
/// ```
/// # use rust_book_examples::*;
/// // This is a documentation test!
/// let result = 2 + 2;
/// assert_eq!(result, 4);
/// ```
/// 
/// Documentation tests are great for:
/// - Ensuring examples in docs stay up-to-date
/// - Testing public API usage patterns
/// - Providing executable examples for users
pub fn documented_function(a: i32, b: i32) -> i32 {
    a + b
}

// Note: Integration tests would go in a separate `tests/` directory
// at the project root. They can only access the public API and test
// the crate from an external perspective.
// 
// Example integration test file: tests/integration_test.rs
// ```rust
// use rust_book_examples::*;
// 
// #[test]
// fn test_public_api() {
//     // Can only test public functions
//     assert_eq!(add_two(5), 7);
//     // Cannot access private functions like internal_helper
// }
// ```