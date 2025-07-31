//! Chapter 6.2: The match Control Flow Construct
//! 
//! This example demonstrates the power of pattern matching in Rust using the `match` 
//! control flow construct. The `match` expression is more powerful than switch statements 
//! in other languages and ensures exhaustive pattern coverage.
//!
//! Key concepts:
//! - Pattern matching with enums
//! - Destructuring data from enum variants
//! - Exhaustive matching (all cases must be handled)
//! - Match guards and complex patterns
//! - Using match with Option and Result types

use rust_book_examples::print_chapter_header;

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Texas,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn describe_point(point: (i32, i32)) -> String {
    match point {
        (0, 0) => String::from("Origin"),
        (0, y) => format!("On the Y axis at {}", y),
        (x, 0) => format!("On the X axis at {}", x),
        (x, y) => format!("Point at ({}, {})", x, y),
    }
}

fn dice_roll(roll: u8) -> String {
    match roll {
        1 => String::from("Critical failure!"),
        6 => String::from("Critical success!"),
        2 | 3 | 4 | 5 => String::from("Normal roll"),
        _ => String::from("Invalid dice roll"),
    }
}

fn check_number(x: Option<i32>) -> String {
    match x {
        Some(n) if n < 0 => String::from("Negative number"),
        Some(n) if n == 0 => String::from("Zero"),
        Some(n) if n > 100 => String::from("Large positive number"),
        Some(n) => format!("Positive number: {}", n),
        None => String::from("No number"),
    }
}

#[derive(Debug)]
enum HttpStatus {
    Ok,
    NotFound,
    ServerError(String),
    Redirect { location: String, permanent: bool },
}

fn handle_response(status: HttpStatus) -> String {
    match status {
        HttpStatus::Ok => {
            String::from("Request successful")
        }
        HttpStatus::NotFound => {
            String::from("Page not found")
        }
        HttpStatus::ServerError(error) => {
            format!("Server error occurred: {}", error)
        }
        HttpStatus::Redirect { location, permanent } => {
            let redirect_type = if permanent { "permanent" } else { "temporary" };
            format!("Redirecting to {} ({})", location, redirect_type)
        }
    }
}

fn main() {
    print_chapter_header("Chapter 6.2", "The match Control Flow Construct");

    // === Basic Enum Matching ===
    println!("\n=== Basic Enum Matching ===");
    
    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value: {} cents", value_in_cents(coin));
    
    let coin2 = Coin::Penny;
    println!("Value: {} cents", value_in_cents(coin2));

    // === Complex Message Processing ===
    println!("\n=== Complex Message Processing ===");
    
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 30 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(200, 255, 255),
    ];
    
    for msg in messages {
        process_message(msg);
    }

    // === Matching with Option<T> ===
    println!("\n=== Matching with Option<T> ===");
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("Original: {:?}, Plus one: {:?}", five, six);
    println!("None plus one: {:?}", none);

    // === Tuple Pattern Matching ===
    println!("\n=== Tuple Pattern Matching ===");
    
    let points = vec![(0, 0), (0, 5), (3, 0), (2, 4)];
    
    for point in points {
        println!("{}", describe_point(point));
    }

    // === Catch-All Patterns ===
    println!("\n=== Catch-All Patterns ===");
    
    for roll in 1..=8 {
        println!("Roll {}: {}", roll, dice_roll(roll));
    }

    // === Match Guards (Conditions) ===
    println!("\n=== Match Guards (Conditions) ===");
    
    let numbers = vec![Some(-5), Some(0), Some(42), Some(150), None];
    
    for num in numbers {
        println!("{:?} -> {}", num, check_number(num));
    }

    // === Real-World HTTP Status Example ===
    println!("\n=== Real-World HTTP Status Matching ===");
    
    let responses = vec![
        HttpStatus::Ok,
        HttpStatus::NotFound,
        HttpStatus::ServerError(String::from("Database connection failed")),
        HttpStatus::Redirect {
            location: String::from("https://example.com"),
            permanent: true,
        },
    ];
    
    for response in responses {
        println!("{}", handle_response(response));
    }

    // === Demonstrating Exhaustiveness ===
    println!("\n=== Exhaustiveness Checking ===");
    println!("The match expression must handle all possible cases!");
    println!("If we forget a case, the compiler will give us an error.");
    println!("This prevents bugs and ensures complete handling of all variants.");

    // === References and Pattern Matching ===
    println!("\n=== References and Pattern Matching ===");
    
    let x = Some(42);
    
    // Match against reference
    match &x {
        Some(n) => println!("Got a reference to number: {}", n),
        None => println!("Got a reference to nothing"),
    }
    
    // Match against owned value (x is moved)
    match x {
        Some(n) => println!("Got owned number: {}", n),
        None => println!("Got owned nothing"),
    }

    println!("\n=== Key Takeaways ===");
    println!("• match expressions must be exhaustive");
    println!("• Patterns can extract values from complex types");
    println!("• Use _ as a catch-all pattern");
    println!("• Match guards allow additional conditions");
    println!("• match compiles to efficient code");
    println!("• Compiler ensures all cases are handled");
}