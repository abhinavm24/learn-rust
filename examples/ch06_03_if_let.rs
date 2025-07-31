//! Chapter 6.3: Concise Control Flow with if let
//! 
//! This example demonstrates the `if let` syntactic sugar for pattern matching
//! when you only care about one specific pattern. It provides a more concise
//! alternative to `match` when you don't need exhaustive pattern matching.
//!
//! Key concepts:
//! - if let as syntactic sugar for match
//! - When to choose if let vs match
//! - Using if let with Option and Result types
//! - Combining if let with else
//! - while let for loops with pattern matching

use rust_book_examples::print_chapter_header;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Texas,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn check_value(value: Option<i32>) {
    if let Some(x) = value {
        println!("Got a value: {}", x);
    } else {
        println!("No value provided");
    }
}

fn handle_write_messages(msg: &Message) {
    if let Message::Write(text) = msg {
        println!("Writing text: {}", text);
    }
}

fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

fn process_nested_option(outer: Option<Option<i32>>) {
    if let Some(inner) = outer {
        if let Some(value) = inner {
            println!("Found nested value: {}", value);
        } else {
            println!("Outer Some, inner None");
        }
    } else {
        println!("Outer None");
    }
}

fn check_conditions(x: Option<i32>, y: Option<i32>) {
    // Using if let with tuple destructuring
    if let (Some(a), Some(b)) = (x, y) {
        if a > 0 && b > 0 {
            println!("Both values are positive: {} and {}", a, b);
        }
    }
}

fn analyze_reference(value: &Option<String>) {
    // Pattern match the reference directly
    if let Some(text) = value {
        println!("Text content: {}", text);
        println!("Text length: {}", text.len());
    } else {
        println!("No text provided");
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(String),
}

fn handle_color_if_let(color: &Color) {
    // Using if let - only care about custom colors
    if let Color::Custom(name) = color {
        println!("Custom color: {}", name);
    } else {
        println!("Standard color: {:?}", color);
    }
}

fn handle_color_match(color: &Color) {
    // Using match - handle all cases explicitly
    match color {
        Color::Red => println!("It's red!"),
        Color::Green => println!("It's green!"),
        Color::Blue => println!("It's blue!"),
        Color::Custom(name) => println!("Custom color: {}", name),
    }
}

fn main() {
    print_chapter_header("Chapter 6.3", "Concise Control Flow with if let");

    // === Basic if let with Option ===
    println!("\n=== Basic if let with Option ===");
    
    let config_max = Some(3u8);
    
    // Using if let
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    
    // Compare with equivalent match
    println!("\nEquivalent match expression:");
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (), // We don't care about None case
    }

    // === if let with else ===
    println!("\n=== if let with else ===");
    
    check_value(Some(42));
    check_value(None);

    // === Enum Variant Matching ===
    println!("\n=== Enum Variant Matching ===");
    
    let coin = Coin::Quarter(UsState::Alaska);
    
    // Only interested in quarters
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        println!("Not a quarter");
    }
    
    let penny = Coin::Penny;
    if let Coin::Quarter(state) = penny {
        println!("State quarter from {:?}!", state);
    } else {
        println!("Not a quarter");
    }

    // === Complex Enum with if let ===
    println!("\n=== Complex Enum with if let ===");
    
    let messages = vec![
        Message::Write(String::from("Hello")),
        Message::Move { x: 10, y: 20 },
        Message::Quit,
        Message::ChangeColor(255, 0, 0),
    ];
    
    for msg in &messages {
        // Only handle Write messages
        handle_write_messages(msg);
        
        // Only handle Move messages
        if let Message::Move { x, y } = msg {
            println!("Moving to position ({}, {})", x, y);
        }
    }

    // === Result Type Handling ===
    println!("\n=== Result Type Handling ===");
    
    let input = "42";
    
    // Only care about successful parsing
    if let Ok(number) = parse_number(input) {
        println!("Successfully parsed: {}", number);
    }
    
    let bad_input = "not_a_number";
    
    // Only care about parsing errors
    if let Err(error) = parse_number(bad_input) {
        println!("Failed to parse: {}", error);
    }

    // === Nested if let ===
    println!("\n=== Nested if let ===");
    
    process_nested_option(Some(Some(42)));
    process_nested_option(Some(None));
    process_nested_option(None);

    // === Multiple Conditions ===
    println!("\n=== Multiple Conditions ===");
    
    check_conditions(Some(5), Some(10));
    check_conditions(Some(-5), Some(10));
    check_conditions(None, Some(10));

    // === Counter Example with if let ===
    println!("\n=== Counter Example with if let ===");
    
    let coins = vec![
        Coin::Quarter(UsState::Alaska),
        Coin::Penny,
        Coin::Quarter(UsState::Texas),
        Coin::Dime,
        Coin::Quarter(UsState::California),
    ];
    
    let mut count = 0;
    for coin in coins {
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
            count += 1;
        }
    }
    
    println!("Found {} quarters", count);

    // === while let Loop ===
    println!("\n=== while let Loop ===");
    
    let mut stack = Vec::new();
    
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    // Pop all values using while let
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
    
    println!("Stack is now empty");

    // === Reference Patterns with if let ===
    println!("\n=== Reference Patterns with if let ===");
    
    let maybe_text = Some(String::from("Hello, world!"));
    analyze_reference(&maybe_text);
    
    let no_text: Option<String> = None;
    analyze_reference(&no_text);

    // === Comparing if let vs match ===
    println!("\n=== Comparing if let vs match ===");
    
    let colors = vec![
        Color::Red,
        Color::Custom(String::from("Purple")),
        Color::Blue,
        Color::Custom(String::from("Orange")),
    ];
    
    println!("Using if let (only shows custom colors and 'other'):");
    for color in &colors {
        handle_color_if_let(color);
    }
    
    println!("\nUsing match (shows all cases explicitly):");
    for color in &colors {
        handle_color_match(color);
    }

    // === When to Use if let vs match ===
    println!("\n=== When to Use if let vs match ===");
    println!("Use if let when:");
    println!("• You only care about one or two patterns");
    println!("• The code is more readable with less boilerplate");
    println!("• Exhaustiveness checking isn't critical");
    println!("• You're working with Option or Result in simple scenarios");
    println!();
    println!("Use match when:");
    println!("• You need exhaustiveness checking");
    println!("• You have multiple patterns to handle");
    println!("• The logic is complex for each case");
    println!("• You want the compiler to catch missing cases");

    println!("\n=== Key Takeaways ===");
    println!("• if let is syntactic sugar for simple pattern matching");
    println!("• Use it when you only care about one pattern");
    println!("• You lose exhaustiveness checking compared to match");
    println!("• while let is great for iterator-like patterns");
    println!("• Choose the right tool: if let for simplicity, match for completeness");
}