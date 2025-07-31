//! Chapter 18.3: Pattern Syntax
//! 
//! This example demonstrates:
//! - Literal patterns
//! - Named variables and shadowing
//! - Multiple patterns with |
//! - Range patterns with ..= and ..
//! - Destructuring structs, enums, and tuples
//! - Ignoring values with _ and ..
//! - Match guards with extra conditions
//! - @ bindings to capture values

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 18.3", "Pattern Syntax");
    
    println!("=== Matching Literals ===");
    matching_literals();
    
    println!("\n=== Named Variables ===");
    named_variables();
    
    println!("\n=== Multiple Patterns ===");
    multiple_patterns();
    
    println!("\n=== Matching Ranges ===");
    matching_ranges();
    
    println!("\n=== Destructuring ===");
    destructuring_examples();
    
    println!("\n=== Ignoring Values ===");
    ignoring_values();
    
    println!("\n=== Extra Conditions with Match Guards ===");
    match_guards();
    
    println!("\n=== @ Bindings ===");
    at_bindings();
    
    println!("\n=== Complex Pattern Combinations ===");
    complex_patterns();
}

fn matching_literals() {
    let x = 1;
    
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    
    // Matching string literals
    let message = "hello";
    match message {
        "hello" => println!("Greeting!"),
        "goodbye" => println!("Farewell!"),
        _ => println!("Something else"),
    }
    
    // Matching character literals
    let letter = 'a';
    match letter {
        'a' => println!("First letter of alphabet"),
        'z' => println!("Last letter of alphabet"),
        _ => println!("Some other letter"),
    }
    
    // Matching boolean literals
    let flag = true;
    match flag {
        true => println!("Flag is set"),
        false => println!("Flag is not set"),
    }
    
    // Matching numeric literals of different types
    let float_val = 3.14;
    match float_val {
        3.14 => println!("Pi!"),
        2.71 => println!("Euler's number!"),
        _ => println!("Some other float"),
    }
}

fn named_variables() {
    let x = Some(5);
    let y = 10;
    
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {}", y), // This y shadows the outer y
        _ => println!("Default case, x = {:?}", x),
    }
    
    println!("At the end: x = {:?}, y = {}", x, y); // Outer y is still 10
    
    // Demonstrating variable shadowing in patterns
    let point = (3, 5);
    let x = 1; // Outer x
    
    match point {
        (x, y) => println!("Point: ({}, {})", x, y), // x here shadows outer x
    }
    
    println!("Outer x is still: {}", x); // Still 1
    
    // Using variables in nested patterns
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 0, y: 7 };
    let x = 5; // Outer x
    
    match p {
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y: 0 } => println!("On the x axis at {}", x), // x shadows outer x
        Point { x, y } => println!("Point at ({}, {})", x, y),
    }
    
    println!("Outer x remains: {}", x);
}

fn multiple_patterns() {
    let x = 1;
    
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    
    // Multiple patterns with different types
    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }
    
    let direction = Direction::North;
    
    match direction {
        Direction::North | Direction::South => {
            println!("Moving vertically: {:?}", direction);
        }
        Direction::East | Direction::West => {
            println!("Moving horizontally: {:?}", direction);
        }
    }
    
    // Multiple patterns with enums that have data
    #[derive(Debug)]
    enum Message {
        Hello { id: i32 },
        Write(String),
        Move { x: i32, y: i32 },
    }
    
    let messages = vec![
        Message::Hello { id: 1 },
        Message::Write(String::from("hello")),
        Message::Move { x: 10, y: 20 },
        Message::Hello { id: 2 },
    ];
    
    for msg in messages {
        match msg {
            Message::Hello { id: 1 } | Message::Hello { id: 2 } => {
                println!("Special hello message: {:?}", msg);
            }
            Message::Write(ref text) if text.len() < 10 => {
                println!("Short message: {}", text);
            }
            _ => {
                println!("Other message: {:?}", msg);
            }
        }
    }
}

fn matching_ranges() {
    let x = 5;
    
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    
    // Character ranges
    let letter = 'c';
    match letter {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    
    // Different range syntaxes
    let num = 42;
    match num {
        0..=10 => println!("single digit or 10"),
        11..=99 => println!("two digits"),
        100..=999 => println!("three digits"),
        _ => println!("very large or negative"),
    }
    
    // Ranges in conditional contexts
    let grade = 85;
    let letter_grade = match grade {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => '?', // Invalid grade
    };
    println!("Grade {}: {}", grade, letter_grade);
    
    // Using ranges with different numeric types
    let temperature: f64 = 22.5;
    let description = match temperature as i32 {
        ..=0 => "freezing",
        1..=15 => "cold",
        16..=25 => "comfortable",
        26..=35 => "warm",
        36.. => "hot",
    };
    println!("Temperature {:.1}°C is {}", temperature, description);
}

fn destructuring_examples() {
    // Destructuring structs
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 0, y: 7 };
    
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    
    // Destructuring with different field names
    let Point { x: a, y: b } = p;
    println!("Renamed fields: a = {}, b = {}", a, b);
    
    // Destructuring enums
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    let msg = Message::ChangeColor(0, 160, 255);
    
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
    
    // Nested enum destructuring
    #[derive(Debug)]
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    
    #[derive(Debug)]
    enum NestedMessage {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    
    let msg = NestedMessage::ChangeColor(Color::Hsv(0, 160, 255));
    
    match msg {
        NestedMessage::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {}, green {}, blue {}", r, g, b);
        }
        NestedMessage::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {}, saturation {}, value {}", h, s, v);
        }
        _ => println!("Other message"),
    }
    
    // Destructuring tuples
    let triple = (0, -2, 3);
    match triple {
        (0, y, z) => println!("First is 0, y = {}, z = {}", y, z),
        (1, ..) => println!("First is 1 and the rest doesn't matter"),
        (.., 2) => println!("Last is 2 and the rest doesn't matter"),
        (3, .., 4) => println!("First is 3, last is 4, middle doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }
    
    // Complex nested destructuring
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("Height: {}' {}\", Point: ({}, {})", feet, inches, x, y);
}

fn ignoring_values() {
    // Ignoring entire values with _
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    
    foo(3, 4);
    
    // Ignoring parts of values
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    
    println!("setting is {:?}", setting_value);
    
    // Ignoring multiple parts with _
    let numbers = (2, 4, 8, 16, 32);
    
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
    
    // Ignoring unused variable with _name
    let _x = 5; // Won't get "unused variable" warning
    let y = 10; // Would get warning if unused
    println!("y = {}", y);
    
    // Ignoring remaining parts with ..
    #[derive(Debug)]
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }
    
    let origin = Point3D { x: 0, y: 0, z: 0 };
    
    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }
    
    // Using .. with tuples
    let numbers = (2, 4, 8, 16, 32);
    
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
    
    // .. in the middle
    match numbers {
        (first, second, .., fourth, fifth) => {
            println!("First: {}, Second: {}, Fourth: {}, Fifth: {}", 
                    first, second, fourth, fifth);
        }
    }
    
    // Array destructuring with ..
    let arr = [1, 2, 3, 4, 5];
    match arr {
        [first, .., last] => println!("First: {}, Last: {}", first, last),
    }
    
    match arr {
        [first, second, ..] => println!("First two: {}, {}", first, second),
    }
}

fn match_guards() {
    let num = Some(4);
    
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("x: {}", x),
        None => (),
    }
    
    // Match guards with multiple patterns
    let x = 4;
    let y = false;
    
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    
    // Complex match guards
    #[derive(Debug)]
    enum Message {
        Hello { id: i32 },
    }
    
    let msg = Message::Hello { id: 5 };
    
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } if id_variable != 4 => {
            println!("Found an id in range [3, 7] but not 4: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
    
    // Match guards with destructured values
    let point = (3, 5);
    match point {
        (x, y) if x == y => println!("These are equal"),
        (x, y) if x + y == 8 => println!("Sum is 8: {} + {} = 8", x, y),
        (x, y) => println!("No special relationship: ({}, {})", x, y),
    }
    
    // Match guards with enums
    #[derive(Debug)]
    enum Temperature {
        Celsius(i32),
        Fahrenheit(i32),
    }
    
    let temp = Temperature::Celsius(35);
    
    match temp {
        Temperature::Celsius(t) if t > 30 => println!("Hot day: {}°C", t),
        Temperature::Celsius(t) if t < 0 => println!("Freezing: {}°C", t),
        Temperature::Celsius(t) => println!("Moderate temperature: {}°C", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("Hot day: {}°F", t),
        Temperature::Fahrenheit(t) if t < 32 => println!("Freezing: {}°F", t),
        Temperature::Fahrenheit(t) => println!("Moderate temperature: {}°F", t),
    }
    
    // Match guards with references
    let numbers = vec![1, 2, 3, 4, 5];
    let target = 3;
    
    for &num in &numbers {
        match num {
            n if n == target => println!("Found target: {}", n),
            n if n < target => println!("Below target: {}", n),
            n => println!("Above target: {}", n),
        }
    }
}

fn at_bindings() {
    #[derive(Debug)]
    enum Message {
        Hello { id: i32 },
    }
    
    let msg = Message::Hello { id: 5 };
    
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
    
    // @ with different patterns
    let x = 5;
    match x {
        e @ 1..=5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }
    
    // @ bindings with structs
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let point = Point { x: 10, y: 5 };
    
    match point {
        Point { x: x_val @ 0..=5, y } => {
            println!("x is small: {}, y: {}", x_val, y)
        }
        Point { x: x_val @ 6..=10, y } => {
            println!("x is medium: {}, y: {}", x_val, y)
        }
        Point { x, y } => println!("x: {}, y: {}", x, y),
    }
    
    // Nested @ bindings
    #[derive(Debug)]
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    
    #[derive(Debug)]
    enum NestedMessage {
        ChangeColor(Color),
        Move { x: i32, y: i32 },
    }
    
    let msg = NestedMessage::ChangeColor(Color::Rgb(255, 0, 0));
    
    match msg {
        NestedMessage::ChangeColor(color @ Color::Rgb(r, g, b)) if r > 200 => {
            println!("Bright red color: {:?} with values ({}, {}, {})", color, r, g, b)
        }
        NestedMessage::ChangeColor(color) => {
            println!("Other color: {:?}", color)
        }
        NestedMessage::Move { x: x_val @ 0..=10, y } => {
            println!("Small x movement: {}, y: {}", x_val, y)
        }
        msg => println!("Other message: {:?}", msg),
    }
    
    // @ with multiple patterns
    let value = 2;
    match value {
        n @ (1 | 2) => println!("Got 1 or 2: {}", n),
        n @ 3..=5 => println!("Got 3-5: {}", n),
        n => println!("Got something else: {}", n),
    }
}

fn complex_patterns() {
    // Combining multiple pattern features
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        address: Address,
    }
    
    #[derive(Debug)]
    struct Address {
        street: String,
        city: String,
        zip: String,
    }
    
    let person = Person {
        name: String::from("Alice"),
        age: 25,
        address: Address {
            street: String::from("123 Main St"),
            city: String::from("Springfield"),
            zip: String::from("12345"),
        },
    };
    
    // Complex nested destructuring with guards and @ bindings
    match person {
        Person {
            name,
            age: age @ 18..=30,
            address: Address { city, zip, .. }
        } if city == "Springfield" => {
            println!("Young person {} (age {}) from Springfield, ZIP: {}", name, age, zip);
        }
        Person {
            name,
            age: age @ 31..=65,
            address: Address { city, .. }
        } => {
            println!("Middle-aged person {} (age {}) from {}", name, age, city);
        }
        Person { name, age, .. } => {
            println!("Person {} (age {})", name, age);
        }
    }
    
    // Pattern matching with collections
    let data = vec![
        (String::from("Alice"), Some(25), true),
        (String::from("Bob"), None, false),
        (String::from("Charlie"), Some(30), true),
    ];
    
    for entry in data {
        match entry {
            (name, Some(age @ 20..=30), true) => {
                println!("{} is a young active person (age {})", name, age);
            }
            (name, Some(age), false) => {
                println!("{} is {} years old but not active", name, age);
            }
            (name, None, active) => {
                println!("{} has unknown age, active: {}", name, active);
            }
            (name, Some(age), true) => {
                println!("{} is {} years old and active", name, age);
            }
        }
    }
    
    // Advanced enum pattern matching
    #[derive(Debug)]
    enum ApiResponse<T> {
        Success { data: T, status: u16 },
        Error { message: String, code: u16 },
        Loading,
    }
    
    let responses = vec![
        ApiResponse::Success { data: "Hello".to_string(), status: 200 },
        ApiResponse::Error { message: "Not found".to_string(), code: 404 },
        ApiResponse::Loading,
        ApiResponse::Error { message: "Server error".to_string(), code: 500 },
    ];
    
    for response in responses {
        match response {
            ApiResponse::Success { data, status: 200 } => {
                println!("Success: {}", data);
            }
            ApiResponse::Success { data, status: status @ 201..=299 } => {
                println!("Success with status {}: {}", status, data);
            }
            ApiResponse::Error { message, code: error_code @ 400..=499 } => {
                println!("Client error {}: {}", error_code, message);
            }
            ApiResponse::Error { message, code: error_code @ 500..=599 } => {
                println!("Server error {}: {}", error_code, message);
            }
            ApiResponse::Loading => {
                println!("Still loading...");
            }
            response => {
                println!("Unexpected response: {:?}", response);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_matching() {
        let result = match 2 {
            1 => "one",
            2 => "two",
            _ => "other",
        };
        assert_eq!(result, "two");
    }

    #[test]
    fn test_range_matching() {
        let result = match 5 {
            1..=5 => "small",
            6..=10 => "medium",
            _ => "large",
        };
        assert_eq!(result, "small");
    }

    #[test]
    fn test_destructuring() {
        struct Point { x: i32, y: i32 }
        let p = Point { x: 5, y: 10 };
        
        let result = match p {
            Point { x: 0, y } => format!("on y-axis: {}", y),
            Point { x, y: 0 } => format!("on x-axis: {}", x),
            Point { x, y } => format!("point: ({}, {})", x, y),
        };
        
        assert_eq!(result, "point: (5, 10)");
    }

    #[test]
    fn test_match_guards() {
        let x = Some(5);
        let result = match x {
            Some(n) if n < 5 => "small",
            Some(n) if n >= 5 => "large",
            None => "none",
        };
        assert_eq!(result, "large");
    }

    #[test]
    fn test_at_bindings() {
        let x = 5;
        let result = match x {
            n @ 1..=5 => format!("small: {}", n),
            n @ 6..=10 => format!("medium: {}", n),
            n => format!("large: {}", n),
        };
        assert_eq!(result, "small: 5");
    }
}