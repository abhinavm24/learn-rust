//! Chapter 18.2: Refutability: Whether a Pattern Might Fail to Match
//! 
//! This example demonstrates:
//! - Irrefutable patterns (always match)
//! - Refutable patterns (might not match)
//! - Where each type of pattern can be used
//! - Converting between refutable and irrefutable contexts
//! - Pattern matching exhaustiveness

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 18.2", "Refutability: Whether a Pattern Might Fail to Match");
    
    println!("=== Irrefutable Patterns ===");
    irrefutable_patterns();
    
    println!("\n=== Refutable Patterns ===");
    refutable_patterns();
    
    println!("\n=== Pattern Context Requirements ===");
    pattern_context_requirements();
    
    println!("\n=== Converting Between Pattern Types ===");
    converting_patterns();
    
    println!("\n=== Exhaustiveness in Pattern Matching ===");
    exhaustiveness_examples();
}

fn irrefutable_patterns() {
    println!("These patterns always match and cannot fail:");
    
    // Variable binding - always matches
    let x = 5;
    println!("Variable binding: x = {}", x);
    
    // Tuple destructuring - always matches if structure is correct
    let point = (3, 5);
    let (x, y) = point;
    println!("Tuple destructuring: ({}, {})", x, y);
    
    // Array destructuring - always matches
    let arr = [1, 2, 3];
    let [a, b, c] = arr;
    println!("Array destructuring: [{}, {}, {}]", a, b, c);
    
    // Struct destructuring - always matches
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 10, y: 20 };
    let Point { x, y } = p;
    println!("Struct destructuring: Point {{ x: {}, y: {} }}", x, y);
    
    // Reference patterns - always match
    let value = 42;
    let ref_value = &value;
    let &x = ref_value;
    println!("Reference pattern: {}", x);
    
    // Wildcard pattern - always matches
    let _ = "anything";
    println!("Wildcard pattern: matched anything");
    
    // Multiple variable assignment - always matches
    let (first, second, third) = (1, 2, 3);
    println!("Multiple assignment: {}, {}, {}", first, second, third);
}

fn refutable_patterns() {
    println!("These patterns might not match:");
    
    // Option patterns - might be None
    let some_value: Option<i32> = Some(3);
    let none_value: Option<i32> = None;
    
    // This works with Some
    if let Some(x) = some_value {
        println!("Got Some: {}", x);
    } else {
        println!("Got None");
    }
    
    // This demonstrates the refutable nature
    if let Some(x) = none_value {
        println!("Got Some: {}", x);
    } else {
        println!("Got None - pattern didn't match");
    }
    
    // Result patterns - might be Err
    let ok_result: Result<i32, &str> = Ok(42);
    let err_result: Result<i32, &str> = Err("error");
    
    if let Ok(value) = ok_result {
        println!("Success: {}", value);
    } else {
        println!("Error case");
    }
    
    if let Err(error) = err_result {
        println!("Error: {}", error);
    } else {
        println!("Success case");
    }
    
    // Enum variant patterns - might not match specific variant
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    let msg = Message::Write(String::from("hello"));
    
    if let Message::Move { x, y } = msg {
        println!("Move message: ({}, {})", x, y);
    } else {
        println!("Not a Move message");
    }
    
    // Literal patterns - might not match
    let number = 4;
    if let 5 = number {
        println!("Number is 5");
    } else {
        println!("Number is not 5: {}", number);
    }
    
    // Range patterns - might not match
    let grade = 85;
    if let 90..=100 = grade {
        println!("Grade A");
    } else if let 80..=89 = grade {
        println!("Grade B");
    } else {
        println!("Lower grade");
    }
    
    // Guard patterns - condition might not be true
    let number = Some(4);
    if let Some(x) = number {
        if x < 5 {
            println!("Small number: {}", x);
        } else {
            println!("Large number");
        }
    } else {
        println!("None");
    }
}

fn pattern_context_requirements() {
    println!("Different contexts require different pattern types:");
    
    // let statements require irrefutable patterns
    println!("\n--- let statements (irrefutable context) ---");
    let x = 5; // ✓ Always matches
    let (a, b) = (1, 2); // ✓ Always matches
    println!("let patterns: x={}, a={}, b={}", x, a, b);
    
    // This would cause a compile error:
    // let Some(x) = some_option_value; // ❌ Refutable pattern in irrefutable context
    
    // Function parameters require irrefutable patterns
    println!("\n--- function parameters (irrefutable context) ---");
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Coordinates: ({}, {})", x, y);
    }
    
    print_coordinates(&(10, 20));
    
    // This would cause a compile error:
    // fn process_option(Some(x): Option<i32>) {} // ❌ Refutable pattern
    
    // for loops require irrefutable patterns
    println!("\n--- for loops (irrefutable context) ---");
    let points = vec![(1, 2), (3, 4), (5, 6)];
    for (x, y) in points {
        println!("Point: ({}, {})", x, y);
    }
    
    // if let allows refutable patterns
    println!("\n--- if let expressions (refutable context) ---");
    let optional_value = Some(7);
    if let Some(x) = optional_value {
        println!("Found value: {}", x);
    }
    
    // while let allows refutable patterns
    println!("\n--- while let loops (refutable context) ---");
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
    
    // match arms can have refutable patterns
    println!("\n--- match expressions (refutable context) ---");
    let value = Some(5);
    match value {
        Some(x) => println!("Got {}", x),
        None => println!("Got nothing"),
    }
}

fn converting_patterns() {
    println!("Converting between refutable and irrefutable contexts:");
    
    // Making refutable patterns work in irrefutable contexts
    println!("\n--- Using unwrap/expect to force irrefutable ---");
    let some_option: Option<i32> = Some(10);
    let x = some_option.unwrap(); // Forces the pattern to be irrefutable
    println!("Unwrapped value: {}", x);
    
    // Using match to handle all cases
    println!("\n--- Using match to handle refutable patterns ---");
    let maybe_value: Option<i32> = Some(15);
    let result = match maybe_value {
        Some(x) => x * 2,
        None => 0,
    };
    println!("Result: {}", result);
    
    // Using if let for optional handling
    println!("\n--- Using if let for graceful handling ---");
    let data: Result<i32, &str> = Ok(42);
    if let Ok(value) = data {
        println!("Processing value: {}", value);
    } else {
        println!("Could not process data");
    }
    
    // Making irrefutable patterns more specific
    println!("\n--- Adding specificity to irrefutable patterns ---");
    #[derive(Debug)]
    enum Status {
        Active(String),
        Inactive,
    }
    
    let status = Status::Active(String::from("running"));
    
    // Instead of always destructuring, use match for safety
    match status {
        Status::Active(msg) => println!("System is active: {}", msg),
        Status::Inactive => println!("System is inactive"),
    }
    
    // Demonstrating pattern refinement
    println!("\n--- Pattern refinement techniques ---");
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Using get() instead of indexing for safety
    if let Some(&first) = numbers.get(0) {
        println!("First element: {}", first);
    } else {
        println!("Vector is empty");
    }
    
    // Using pattern matching with collections
    match numbers.as_slice() {
        [] => println!("Empty vector"),
        [single] => println!("Single element: {}", single),
        [first, rest @ ..] => println!("First: {}, {} more elements", first, rest.len()),
    }
}

fn exhaustiveness_examples() {
    println!("Pattern matching exhaustiveness:");
    
    // Exhaustive enum matching
    #[derive(Debug)]
    enum Color {
        Red,
        Green,
        Blue,
        Rgb(u8, u8, u8),
        Hsv(u16, u8, u8),
    }
    
    let color = Color::Rgb(255, 0, 0);
    
    // This match is exhaustive - all variants are covered
    match color {
        Color::Red => println!("Pure red"),
        Color::Green => println!("Pure green"),
        Color::Blue => println!("Pure blue"),
        Color::Rgb(r, g, b) => println!("RGB color: ({}, {}, {})", r, g, b),
        Color::Hsv(h, s, v) => println!("HSV color: ({}, {}, {})", h, s, v),
    }
    
    // Using catch-all patterns
    println!("\n--- Catch-all patterns ---");
    let number = 7;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else: {}", number), // Catch-all
    }
    
    // Partial matching with catch-all
    let color2 = Color::Green;
    match color2 {
        Color::Red | Color::Green | Color::Blue => println!("Primary color"),
        _ => println!("Custom color"), // Handles Rgb and Hsv
    }
    
    // Demonstrating non-exhaustive patterns
    println!("\n--- Handling optional exhaustiveness ---");
    
    #[derive(Debug)]
    enum NetworkState {
        Connected,
        Disconnected,
        Error(String),
        Timeout,
    }
    
    let state = NetworkState::Error(String::from("Connection refused"));
    
    // Exhaustive matching
    let message = match state {
        NetworkState::Connected => "All good",
        NetworkState::Disconnected => "Not connected",
        NetworkState::Error(ref err) => {
            println!("Error details: {}", err);
            "Error occurred"
        }
        NetworkState::Timeout => "Request timed out",
    };
    println!("Status: {}", message);
    
    // Using ranges in exhaustive matching
    println!("\n--- Range patterns in exhaustive matching ---");
    let grade = 85;
    let letter_grade = match grade {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => '?', // Handle invalid grades
    };
    println!("Grade {}: {}", grade, letter_grade);
    
    // Multiple patterns in single arm
    println!("\n--- Multiple patterns ---");
    let day = 3;
    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }
    
    // Nested pattern exhaustiveness
    println!("\n--- Nested pattern matching ---");
    
    #[derive(Debug)]
    enum Shape {
        Circle(f64),
        Rectangle { width: f64, height: f64 },
        Triangle(f64, f64, f64),
    }
    
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle { width: 10.0, height: 20.0 },
        Shape::Triangle(3.0, 4.0, 5.0),
    ];
    
    for shape in shapes {
        let area = match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle(a, b, c) => {
                // Using Heron's formula
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        };
        println!("Shape: {:?}, Area: {:.2}", shape, area);
    }
}

// Examples of patterns that would cause compile errors
fn compile_error_examples() {
    // These examples show what would cause compile errors:
    
    // ❌ This would fail: refutable pattern in let statement
    // let some_option_value: Option<i32> = None;
    // let Some(x) = some_option_value; // Compile error!
    
    // ✅ Correct way:
    let some_option_value: Option<i32> = None;
    if let Some(x) = some_option_value {
        println!("Got value: {}", x);
    }
    
    // ❌ This would fail: irrefutable pattern in if let
    // let x = 5;
    // if let y = x { // Compile warning: irrefutable pattern
    //     println!("y: {}", y);
    // }
    
    // ✅ Correct way:
    let x = 5;
    let y = x; // Just use direct assignment
    println!("y: {}", y);
    
    // ❌ This would fail: non-exhaustive match
    // enum Color { Red, Green, Blue }
    // let color = Color::Red;
    // match color {
    //     Color::Red => println!("red"),
    //     Color::Green => println!("green"),
    //     // Missing Blue case - compile error!
    // }
    
    // ✅ Correct way:
    #[derive(Debug)]
    enum Color { Red, Green, Blue }
    let color = Color::Red;
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"), // All cases covered
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_irrefutable_patterns() {
        let point = (5, 10);
        let (x, y) = point;
        assert_eq!(x, 5);
        assert_eq!(y, 10);
    }

    #[test]
    fn test_refutable_patterns() {
        let some_value = Some(42);
        if let Some(x) = some_value {
            assert_eq!(x, 42);
        } else {
            panic!("Expected Some(42)");
        }
        
        let none_value: Option<i32> = None;
        if let Some(_) = none_value {
            panic!("Expected None");
        }
    }

    #[test]
    fn test_pattern_exhaustiveness() {
        #[derive(Debug, PartialEq)]
        enum Status { Active, Inactive, Pending }
        
        let status = Status::Active;
        let result = match status {
            Status::Active => "active",
            Status::Inactive => "inactive",
            Status::Pending => "pending",
        };
        assert_eq!(result, "active");
    }

    #[test]
    fn test_pattern_conversion() {
        let maybe_value: Option<i32> = Some(10);
        let doubled = match maybe_value {
            Some(x) => x * 2,
            None => 0,
        };
        assert_eq!(doubled, 20);
    }

    #[test]
    fn test_range_patterns() {
        let grade = 85;
        let letter = match grade {
            90..=100 => 'A',
            80..=89 => 'B',
            70..=79 => 'C',
            60..=69 => 'D',
            _ => 'F',
        };
        assert_eq!(letter, 'B');
    }
}