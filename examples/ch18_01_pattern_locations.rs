//! Chapter 18.1: All the Places Patterns Can Be Used
//! 
//! This example demonstrates:
//! - Pattern matching in match expressions
//! - Conditional if let expressions  
//! - while let loops with patterns
//! - for loop destructuring
//! - let statement patterns
//! - Function parameter patterns
//! - Closure parameter patterns

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 18.1", "All the Places Patterns Can Be Used");
    
    println!("=== Match Arms ===");
    match_arms_examples();
    
    println!("\n=== Conditional if let ===");
    conditional_if_let_examples();
    
    println!("\n=== while let Loops ===");
    while_let_examples();
    
    println!("\n=== for Loop Patterns ===");
    for_loop_patterns();
    
    println!("\n=== let Statement Patterns ===");
    let_statement_patterns();
    
    println!("\n=== Function Parameter Patterns ===");
    function_parameter_patterns();
    
    println!("\n=== Closure Parameter Patterns ===");
    closure_parameter_patterns();
    
    println!("\n=== Nested and Complex Patterns ===");
    complex_patterns();
}

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

fn match_arms_examples() {
    // Basic match with literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything else"),
    }
    
    // Match with enum destructuring
    let coin = Coin::Quarter(UsState::Alaska);
    let value = match coin {
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
    };
    println!("Coin value: {} cents", value);
    
    // Match with Option
    let some_number = Some(5);
    match some_number {
        None => println!("No value"),
        Some(i) => println!("Got a value: {}", i),
    }
    
    // Match with guards
    let number = Some(4);
    match number {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("Greater than or equal to five: {}", x),
        None => println!("No number"),
    }
    
    // Match with multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything else"),
    }
    
    // Match with ranges
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

fn conditional_if_let_examples() {
    // Basic if let with Option
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
    
    // if let with complex patterns
    #[derive(Debug)]
    enum Message {
        Hello { id: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    let msg = Message::Hello { id: 5 };
    if let Message::Hello { id: id_variable @ 3..=7 } = msg {
        println!("Found an id in range: {}", id_variable);
    } else {
        println!("ID not in range or different message type");
    }
    
    // if let with Result
    let result: Result<i32, &str> = Ok(42);
    if let Ok(value) = result {
        println!("Success with value: {}", value);
    } else {
        println!("Operation failed");
    }
    
    // Multiple if let chains
    let data = Some("hello");
    if let Some(text) = data {
        if text.len() > 3 {
            println!("Text '{}' is long enough", text);
        }
    }
}

fn while_let_examples() {
    // Basic while let with Option
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("Popping from stack:");
    while let Some(top) = stack.pop() {
        println!("  {}", top);
    }
    
    // while let with iterator
    let v = vec!['a', 'b', 'c'];
    let mut iter = v.iter();
    
    println!("Iterating with while let:");
    while let Some(value) = iter.next() {
        println!("  {}", value);
    }
    
    // while let with complex patterns
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    map.insert("key3", 3);
    
    let mut keys: Vec<_> = map.keys().cloned().collect();
    
    println!("Processing map entries:");
    while let Some(key) = keys.pop() {
        if let Some(value) = map.remove(key) {
            println!("  Removed {}: {}", key, value);
        }
    }
    
    // while let with Result
    let results = vec![Ok(1), Ok(2), Err("error"), Ok(4)];
    let mut iter = results.into_iter();
    
    println!("Processing results until error:");
    while let Ok(value) = iter.next().unwrap_or(Err("done")) {
        println!("  Got value: {}", value);
    }
}

fn for_loop_patterns() {
    // Basic tuple destructuring
    let v = vec!['a', 'b', 'c'];
    println!("Enumerating with destructuring:");
    for (index, value) in v.iter().enumerate() {
        println!("  {} is at index {}", value, index);
    }
    
    // Destructuring tuples
    let points = vec![(0, 1), (2, 3), (4, 5)];
    println!("Iterating over points:");
    for (x, y) in points {
        println!("  Point: ({}, {})", x, y);
    }
    
    // Destructuring structs
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let points = vec![
        Point { x: 0, y: 1 },
        Point { x: 2, y: 3 },
        Point { x: 4, y: 5 },
    ];
    
    println!("Destructuring structs in for loop:");
    for Point { x, y } in points {
        println!("  Point coordinates: ({}, {})", x, y);
    }
    
    // Complex destructuring
    let data = vec![
        ("Alice", 30, "Engineer"),
        ("Bob", 25, "Designer"),
        ("Charlie", 35, "Manager"),
    ];
    
    println!("Processing employee data:");
    for (name, age, title) in data {
        println!("  {}: {} years old, works as {}", name, age, title);
    }
    
    // Nested destructuring
    let nested = vec![
        ((1, 2), (3, 4)),
        ((5, 6), (7, 8)),
    ];
    
    println!("Nested tuple destructuring:");
    for ((a, b), (c, d)) in nested {
        println!("  First pair: ({}, {}), Second pair: ({}, {})", a, b, c, d);
    }
    
    // Using references in patterns
    let data = vec![1, 2, 3, 4, 5];
    println!("Iterating with references:");
    for &item in &data {
        println!("  Value: {}", item);
    }
}

fn let_statement_patterns() {
    // Basic tuple destructuring
    let point = (3, 5);
    let (x, y) = point;
    println!("Point coordinates: x = {}, y = {}", x, y);
    
    // Struct destructuring
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 0, y: 7 };
    
    // Rename fields during destructuring
    let Point { x: a, y: b } = p;
    println!("Renamed coordinates: a = {}, b = {}", a, b);
    
    // Shorthand when variable names match field names
    let p = Point { x: 10, y: 20 };
    let Point { x, y } = p;
    println!("Shorthand destructuring: x = {}, y = {}", x, y);
    
    // Array destructuring
    let arr = [1, 2, 3, 4, 5];
    let [first, second, ..] = arr;
    println!("First two elements: {}, {}", first, second);
    
    let [.., fourth, fifth] = arr;
    println!("Last two elements: {}, {}", fourth, fifth);
    
    let [first, .., last] = arr;
    println!("First and last: {}, {}", first, last);
    
    // Complex nested destructuring
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("Height: {} feet {} inches, Point: ({}, {})", feet, inches, x, y);
    
    // Destructuring with references
    let point = (1, 2);
    let (ref x, ref y) = point;
    println!("References: x = {}, y = {}", x, y);
    
    // Mutable destructuring
    let mut point = (0, 0);
    let (ref mut x, ref mut y) = point;
    *x = 5;
    *y = 10;
    println!("Modified point: {:?}", point);
}

// Function parameter patterns
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn print_person_name(Person { name, .. }: Person) {
    println!("Person's name: {}", name);
}

fn print_person_info(Person { name, age }: &Person) {
    println!("{} is {} years old", name, age);
}

fn calculate_distance(&(x1, y1): &(f64, f64), &(x2, y2): &(f64, f64)) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn function_parameter_patterns() {
    let point = (3, 5);
    print_coordinates(&point);
    
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    // This function takes ownership
    print_person_name(Person {
        name: String::from("Bob"),
        age: 25,
    });
    
    // This function borrows
    print_person_info(&person);
    
    let point1 = (0.0, 0.0);
    let point2 = (3.0, 4.0);
    let distance = calculate_distance(&point1, &point2);
    println!("Distance between points: {:.2}", distance);
    
    // Function with complex parameter destructuring
    fn process_data((id, (name, score)): (u32, (String, f64))) {
        println!("ID: {}, Name: {}, Score: {:.1}", id, name, score);
    }
    
    let data = (1, (String::from("Alice"), 95.5));
    process_data(data);
}

fn closure_parameter_patterns() {
    // Basic tuple destructuring in closures
    let points = vec![(0, 1), (2, 3), (4, 5)];
    let sum_of_squares: i32 = points
        .iter()
        .map(|(x, y)| x * x + y * y)
        .sum();
    println!("Sum of squares: {}", sum_of_squares);
    
    // Struct destructuring in closures
    #[derive(Debug)]
    struct Employee {
        name: String,
        salary: u32,
    }
    
    let employees = vec![
        Employee { name: String::from("Alice"), salary: 50000 },
        Employee { name: String::from("Bob"), salary: 60000 },
        Employee { name: String::from("Charlie"), salary: 55000 },
    ];
    
    let high_earners: Vec<_> = employees
        .iter()
        .filter(|Employee { salary, .. }| *salary > 55000)
        .map(|Employee { name, salary }| format!("{}: ${}", name, salary))
        .collect();
    
    println!("High earners:");
    for earner in high_earners {
        println!("  {}", earner);
    }
    
    // Complex pattern matching in closures
    let data = vec![
        Some((String::from("Alice"), 25)),
        None,
        Some((String::from("Bob"), 30)),
        Some((String::from("Charlie"), 28)),
    ];
    
    let adults: Vec<_> = data
        .into_iter()
        .filter_map(|person| match person {
            Some((name, age)) if age >= 25 => Some(format!("{} ({})", name, age)),
            _ => None,
        })
        .collect();
    
    println!("Adults (25+):");
    for adult in adults {
        println!("  {}", adult);
    }
    
    // Nested pattern in closure
    let nested_data = vec![
        ((String::from("Product A"), 100), (String::from("Electronics"), 4.5)),
        ((String::from("Product B"), 200), (String::from("Books"), 4.8)),
        ((String::from("Product C"), 150), (String::from("Electronics"), 4.2)),
    ];
    
    let expensive_electronics: Vec<_> = nested_data
        .into_iter()
        .filter_map(|((name, price), (category, rating))| {
            if category == "Electronics" && price > 120 {
                Some(format!("{}: ${} (rating: {:.1})", name, price, rating))
            } else {
                None
            }
        })
        .collect();
    
    println!("Expensive electronics:");
    for item in expensive_electronics {
        println!("  {}", item);
    }
}

fn complex_patterns() {
    // Nested struct and enum patterns
    #[derive(Debug)]
    struct Address {
        street: String,
        city: String,
        country: String,
    }
    
    #[derive(Debug)]
    struct Person {
        name: String,
        address: Address,
    }
    
    let person = Person {
        name: String::from("Alice"),
        address: Address {
            street: String::from("123 Main St"),
            city: String::from("Anytown"),
            country: String::from("USA"),
        },
    };
    
    // Nested destructuring
    let Person {
        name,
        address: Address { city, country, .. },
    } = person;
    
    println!("{} lives in {}, {}", name, city, country);
    
    // Complex enum with nested patterns
    #[derive(Debug)]
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    
    let messages = vec![
        Message::Move { x: 10, y: 30 },
        Message::ChangeColor(Color::Rgb(255, 0, 0)),
        Message::ChangeColor(Color::Hsv(0, 100, 100)),
        Message::Write(String::from("Hello, world!")),
    ];
    
    println!("Processing messages:");
    for msg in messages {
        match msg {
            Message::Quit => println!("  Quit message received"),
            Message::Move { x, y } => println!("  Move to coordinates ({}, {})", x, y),
            Message::Write(text) => println!("  Write message: {}", text),
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("  Change color to RGB({}, {}, {})", r, g, b);
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("  Change color to HSV({}, {}, {})", h, s, v);
            }
        }
    }
    
    // Pattern with @ binding
    let numbers = vec![1, 5, 10, 15, 20];
    for num in numbers {
        match num {
            n @ 1..=5 => println!("Small number: {}", n),
            n @ 6..=15 => println!("Medium number: {}", n),
            n @ 16..=25 => println!("Large number: {}", n),
            n => println!("Other number: {}", n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_matching() {
        let coin = Coin::Quarter(UsState::Alaska);
        match coin {
            Coin::Quarter(state) => assert!(matches!(state, UsState::Alaska)),
            _ => panic!("Expected Quarter"),
        }
    }

    #[test]
    fn test_if_let() {
        let some_value = Some(3);
        if let Some(x) = some_value {
            assert_eq!(x, 3);
        } else {
            panic!("Expected Some(3)");
        }
    }

    #[test]
    fn test_destructuring() {
        let point = (5, 10);
        let (x, y) = point;
        assert_eq!(x, 5);
        assert_eq!(y, 10);
    }

    #[test]
    fn test_function_parameters() {
        fn add_points(&(x1, y1): &(i32, i32), &(x2, y2): &(i32, i32)) -> (i32, i32) {
            (x1 + x2, y1 + y2)
        }
        
        let result = add_points(&(1, 2), &(3, 4));
        assert_eq!(result, (4, 6));
    }
}