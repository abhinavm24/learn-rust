//! # Chapter 11.1: How to Write Tests
//! 
//! This example demonstrates:
//! - Writing unit tests with the #[test] attribute
//! - Using assertion macros (assert!, assert_eq!, assert_ne!)
//! - Testing for panic with should_panic
//! - Using Result<T, E> in tests
//! - Custom error messages in assertions
//! - Test organization and best practices

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 11.1", "How to Write Tests");

    println!("=== Basic Testing Concepts ===");
    println!("Tests are functions annotated with #[test] that verify code behavior.");
    println!("Run tests with: cargo test");
    println!();
    
    println!("=== Function Examples (see tests below) ===");
    
    let rect = Rectangle { width: 10, height: 5 };
    println!("Rectangle: {}x{}", rect.width, rect.height);
    println!("Area: {}", rect.area());
    println!("Can hold 5x3 rectangle: {}", rect.can_hold(&Rectangle { width: 5, height: 3 }));
    
    println!("\n=== Greeting Function ===");
    println!("Greeting: {}", greeting("Alice"));
    
    println!("\n=== Guess Validation ===");
    match Guess::new(50) {
        Ok(guess) => println!("Valid guess: {}", guess.value()),
        Err(e) => println!("Invalid guess: {}", e),
    }
}

// Example struct for testing
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Function that we'll test
pub fn add_two(a: i32) -> i32 {
    a + 2
}

// Function for testing custom messages
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// Struct for testing panic conditions
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, String> {
        if value < 1 {
            Err(String::from("Guess value must be greater than or equal to 1"))
        } else if value > 100 {
            Err(String::from("Guess value must be less than or equal to 100"))
        } else {
            Ok(Guess { value })
        }
    }
    
    pub fn value(&self) -> i32 {
        self.value
    }
}

// Function that panics under certain conditions
pub fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("Cannot divide by zero!");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Basic Test Examples ===
    
    #[test]
    fn it_works() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    
    // === Testing with assert! macro ===
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
    
    // === Testing with assert_eq! and assert_ne! ===
    
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
    
    #[test]
    fn it_adds_two_ne() {
        assert_ne!(5, add_two(2));
    }
    
    #[test]
    fn test_area() {
        let rect = Rectangle {
            width: 10,
            height: 5,
        };
        assert_eq!(rect.area(), 50);
    }
    
    // === Custom Error Messages ===
    
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
    
    #[test]
    fn greeting_format() {
        let result = greeting("Alice");
        assert_eq!(
            result,
            "Hello Alice!",
            "Expected greeting format, but got: {}",
            result
        );
    }
    
    // === Testing for Panic ===
    
    #[test]
    #[should_panic]
    fn divide_by_zero() {
        divide(10.0, 0.0);
    }
    
    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn divide_by_zero_with_message() {
        divide(5.0, 0.0);
    }
    
    // === Using Result<T, E> in Tests ===
    
    #[test]
    fn it_works_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    
    #[test]
    fn test_guess_valid() -> Result<(), String> {
        let guess = Guess::new(50)?;
        assert_eq!(guess.value(), 50);
        Ok(())
    }
    
    #[test]
    fn test_guess_too_small() {
        let result = Guess::new(0);
        assert!(result.is_err());
        
        if let Err(msg) = result {
            assert!(msg.contains("greater than or equal to 1"));
        }
    }
    
    #[test]
    fn test_guess_too_large() {
        let result = Guess::new(101);
        assert!(result.is_err());
        
        if let Err(msg) = result {
            assert!(msg.contains("less than or equal to 100"));
        }
    }
    
    // === Testing Different Data Types ===
    
    #[test]
    fn test_strings() {
        let expected = String::from("hello world");
        let actual = "hello world";
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn test_vectors() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![1, 2, 3];
        assert_eq!(v1, v2);
        
        let v3 = vec![1, 2, 4];
        assert_ne!(v1, v3);
    }
    
    #[test]
    fn test_floats() {
        let result = 0.1_f64 + 0.2_f64;
        let expected = 0.3_f64;
        
        // Be careful with floating point comparisons
        assert!((result - expected).abs() < f64::EPSILON);
    }
    
    // === Testing Complex Logic ===
    
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    
    fn factorial(n: u32) -> u32 {
        match n {
            0 | 1 => 1,
            _ => n * factorial(n - 1),
        }
    }
    
    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(is_even(-4));
        assert!(!is_even(1));
        assert!(!is_even(3));
        assert!(!is_even(-1));
    }
    
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(5), 120);
    }
    
    // === Testing with Custom Types ===
    
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl Point {
        fn new(x: i32, y: i32) -> Point {
            Point { x, y }
        }
        
        fn distance_from_origin(&self) -> f64 {
            ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
        }
    }
    
    #[test]
    fn test_point_creation() {
        let p = Point::new(3, 4);
        assert_eq!(p.x, 3);
        assert_eq!(p.y, 4);
    }
    
    #[test]
    fn test_point_equality() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(1, 2);
        let p3 = Point::new(2, 1);
        
        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }
    
    #[test]
    fn test_distance_from_origin() {
        let p = Point::new(3, 4);
        let distance = p.distance_from_origin();
        assert!((distance - 5.0).abs() < f64::EPSILON);
    }
    
    // === Testing Error Conditions ===
    
    fn divide_safe(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(a / b)
        }
    }
    
    #[test]
    fn test_divide_safe_success() {
        let result = divide_safe(10.0, 2.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);
    }
    
    #[test]
    fn test_divide_safe_error() {
        let result = divide_safe(10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Division by zero");
    }
    
    // === Testing with Mock Data ===
    
    struct User {
        id: u32,
        name: String,
        email: String,
    }
    
    impl User {
        fn new(id: u32, name: String, email: String) -> User {
            User { id, name, email }
        }
        
        fn is_valid(&self) -> bool {
            !self.name.is_empty() && self.email.contains('@')
        }
    }
    
    #[test]
    fn test_user_creation() {
        let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Alice");
        assert_eq!(user.email, "alice@example.com");
    }
    
    #[test]
    fn test_user_validation() {
        let valid_user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
        assert!(valid_user.is_valid());
        
        let invalid_user = User::new(2, "".to_string(), "invalid-email".to_string());
        assert!(!invalid_user.is_valid());
        
        let another_invalid = User::new(3, "Bob".to_string(), "no-at-symbol".to_string());
        assert!(!another_invalid.is_valid());
    }
    
    // === Testing Collections ===
    
    fn find_max(numbers: &[i32]) -> Option<i32> {
        if numbers.is_empty() {
            None
        } else {
            Some(*numbers.iter().max().unwrap())
        }
    }
    
    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&[1, 5, 3, 2]), Some(5));
        assert_eq!(find_max(&[10]), Some(10));
        assert_eq!(find_max(&[]), None);
        assert_eq!(find_max(&[-1, -5, -2]), Some(-1));
    }
    
    // === Testing with Setup and Teardown Patterns ===
    
    fn setup_test_data() -> Vec<i32> {
        vec![1, 2, 3, 4, 5]
    }
    
    #[test]
    fn test_with_setup() {
        let data = setup_test_data();
        assert_eq!(data.len(), 5);
        assert_eq!(data[0], 1);
        assert_eq!(data[4], 5);
    }
    
    // === Helper Functions in Tests ===
    
    fn assert_approximately_equal(a: f64, b: f64, tolerance: f64) {
        assert!((a - b).abs() < tolerance, "{} and {} are not approximately equal", a, b);
    }
    
    #[test]
    fn test_with_helper() {
        let result = 1.0 / 3.0 * 3.0;
        assert_approximately_equal(result, 1.0, 1e-10);
    }
}