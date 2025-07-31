//! Chapter 8.1: Storing Lists of Values with Vectors
//! 
//! This example demonstrates Rust's Vec<T> type, which allows you to store
//! multiple values of the same type in a single data structure that puts
//! all values next to each other in memory. Vectors are dynamically-sized
//! arrays that can grow and shrink at runtime.
//!
//! Key concepts:
//! - Creating and modifying vectors
//! - Different ways to access vector elements
//! - Understanding borrowing rules with vectors
//! - Iterating over vectors
//! - Using enums to store different types in vectors

use rust_book_examples::print_chapter_header;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

#[derive(Debug)]
struct Item {
    name: String,
    price: f64,
    quantity: u32,
}

impl Item {
    fn new(name: &str, price: f64, quantity: u32) -> Self {
        Item {
            name: name.to_string(),
            price,
            quantity,
        }
    }
    
    fn total_price(&self) -> f64 {
        self.price * self.quantity as f64
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
}

fn print_slice(slice: &[i32]) {
    println!("Slice contents: {:?}", slice);
}

fn demonstrate_basic_vectors() {
    println!("\n=== Basic Vector Creation and Usage ===");
    
    // Creating empty vector with type annotation
    let v: Vec<i32> = Vec::new();
    println!("Empty vector: {:?}", v);
    
    // Creating vector with initial values using vec! macro
    let v = vec![1, 2, 3];
    println!("Vector with initial values: {:?}", v);
    
    // Creating and modifying mutable vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    println!("Vector after pushing elements: {:?}", v);
}

fn demonstrate_element_access() {
    println!("\n=== Element Access Methods ===");
    
    let v = vec![1, 2, 3, 4, 5];
    
    // Method 1: Direct indexing (can panic)
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    
    // Method 2: Using get method (returns Option)
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    // Handling out-of-bounds access safely
    println!("\nSafe out-of-bounds access:");
    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(value) => println!("Value: {}", value),
        None => println!("Index 100 is out of bounds"),
    }
    
    // This would panic:
    // let does_not_exist = &v[100];
    println!("Direct indexing with v[100] would panic!");
}

fn demonstrate_borrowing_rules() {
    println!("\n=== Borrowing Rules with Vectors ===");
    
    let mut v = vec![1, 2, 3, 4, 5];
    
    {
        let first = &v[0];  // Immutable borrow
        println!("The first element is: {}", first);
        // Cannot push here because of immutable borrow
    } // first goes out of scope here
    
    v.push(6); // Now we can mutate
    println!("Vector after push: {:?}", v);
    
    println!("\nWhy borrowing rules matter:");
    println!("• Vectors may need to allocate new memory when growing");
    println!("• This could invalidate existing references");
    println!("• Rust prevents this with borrowing rules");
}

fn demonstrate_iteration() {
    println!("\n=== Iterating Over Vectors ===");
    
    let v = vec![100, 32, 57];
    
    // Immutable iteration
    println!("Immutable iteration:");
    for i in &v {
        println!("  {}", i);
    }
    
    // Mutable iteration
    println!("Mutable iteration (adding 50 to each):");
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;  // Dereference to modify the value
    }
    println!("Modified vector: {:?}", v);
    
    // Taking ownership during iteration
    println!("Taking ownership during iteration:");
    let v = vec![String::from("hello"), String::from("world")];
    for s in v {
        println!("  {}", s);
        // v is no longer accessible after this loop
    }
}

fn demonstrate_enum_storage() {
    println!("\n=== Using Enums to Store Multiple Types ===");
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Int(42),
        SpreadsheetCell::Text(String::from("red")),
        SpreadsheetCell::Float(3.14),
    ];
    
    println!("Spreadsheet row contents:");
    for (index, cell) in row.iter().enumerate() {
        match cell {
            SpreadsheetCell::Int(i) => println!("  Cell {}: Integer = {}", index, i),
            SpreadsheetCell::Float(f) => println!("  Cell {}: Float = {:.2}", index, f),
            SpreadsheetCell::Text(s) => println!("  Cell {}: Text = '{}'", index, s),
        }
    }
}

fn demonstrate_shopping_cart() {
    println!("\n=== Real-World Example: Shopping Cart ===");
    
    let mut cart: Vec<Item> = Vec::new();
    
    // Add items to cart
    cart.push(Item::new("Apple", 0.50, 6));
    cart.push(Item::new("Bread", 2.00, 1));
    cart.push(Item::new("Milk", 3.50, 2));
    cart.push(Item::new("Cheese", 4.25, 1));
    
    // Display cart contents and calculate total
    println!("Shopping Cart:");
    let mut total = 0.0;
    for item in &cart {
        let item_total = item.total_price();
        println!("  {}: ${:.2} x {} = ${:.2}", 
                 item.name, item.price, item.quantity, item_total);
        total += item_total;
    }
    
    println!("Total: ${:.2}", total);
    
    // Remove an item (by index)
    if cart.len() > 1 {
        let removed = cart.remove(1);
        println!("Removed: {}", removed.name);
    }
    
    // Display final cart
    println!("Final cart items: {}", cart.len());
    for item in &cart {
        println!("  {}", item.name);
    }
}

fn demonstrate_vector_methods() {
    println!("\n=== Vector Methods and Operations ===");
    
    let mut numbers = vec![1, 2, 3];
    println!("Initial vector: {:?}", numbers);
    
    // Adding elements
    numbers.push(4);
    numbers.push(5);
    println!("After pushing 4 and 5: {:?}", numbers);
    
    // Removing elements
    let last = numbers.pop();
    println!("Popped element: {:?}", last);
    println!("Vector after pop: {:?}", numbers);
    
    // Inserting at specific position
    numbers.insert(1, 10);
    println!("After inserting 10 at index 1: {:?}", numbers);
    
    // Removing at specific position
    let removed = numbers.remove(1);
    println!("Removed element at index 1: {}", removed);
    println!("Vector after removal: {:?}", numbers);
    
    // Length and capacity
    println!("Length: {}", numbers.len());
    println!("Capacity: {}", numbers.capacity());
    
    // Check if empty
    if !numbers.is_empty() {
        println!("Vector is not empty");
    }
    
    // Append another vector
    let mut other = vec![100, 200];
    numbers.append(&mut other);
    println!("After appending: {:?}", numbers);
    println!("Other vector after append: {:?}", other); // Now empty
}

fn demonstrate_slices() {
    println!("\n=== Working with Vector Slices ===");
    
    let v = vec![1, 2, 3, 4, 5];
    
    // Pass entire vector as slice
    println!("Entire vector as slice:");
    print_slice(&v);
    
    // Pass part of vector as slice
    println!("Partial slice [1..3]:");
    print_slice(&v[1..3]);
    
    // Convert vector to slice
    let slice: &[i32] = &v;
    println!("Slice length: {}", slice.len());
    
    // Different slice operations
    println!("First 3 elements: {:?}", &v[..3]);
    println!("Last 2 elements: {:?}", &v[3..]);
    println!("Middle elements: {:?}", &v[1..4]);
}

fn demonstrate_complex_types() {
    println!("\n=== Vector with Complex Types ===");
    
    let mut people = Vec::new();
    
    people.push(Person::new("Alice", 30));
    people.push(Person::new("Bob", 25));
    people.push(Person::new("Charlie", 35));
    people.push(Person::new("Diana", 28));
    
    // Find person by name
    println!("Looking for Bob:");
    for person in &people {
        if person.name == "Bob" {
            println!("Found {}, age {}", person.name, person.age);
            break;
        }
    }
    
    // Filter people by age (using iterators)
    println!("People 30 or older:");
    let adults: Vec<&Person> = people.iter()
        .filter(|person| person.age >= 30)
        .collect();
    
    for person in adults {
        println!("  {} ({})", person.name, person.age);
    }
    
    // Sort by age
    people.sort_by(|a, b| a.age.cmp(&b.age));
    println!("People sorted by age: {:?}", people);
}

fn demonstrate_capacity_management() {
    println!("\n=== Vector Capacity Management ===");
    
    let mut v = Vec::new();
    println!("Initial capacity: {}", v.capacity());
    
    // Reserve capacity to avoid reallocations
    v.reserve(10);
    println!("Capacity after reserve(10): {}", v.capacity());
    
    // Add elements and observe capacity changes
    println!("Adding elements:");
    for i in 0..5 {
        v.push(i);
        println!("  After push({}): len={}, capacity={}", i, v.len(), v.capacity());
    }
    
    // Shrink to fit actual size
    v.shrink_to_fit();
    println!("After shrink_to_fit: len={}, capacity={}", v.len(), v.capacity());
    
    // Clear all elements
    v.clear();
    println!("After clear: len={}, capacity={}", v.len(), v.capacity());
}

fn demonstrate_memory_management() {
    println!("\n=== Memory Management and Dropping ===");
    
    {
        let v = vec![1, 2, 3, 4];
        println!("Vector inside scope: {:?}", v);
        // v is valid here
    } // v goes out of scope and is dropped here, along with its elements
    
    println!("Vector is no longer accessible outside its scope");
    
    // Demonstrate ownership transfer
    let v1 = vec![String::from("hello"), String::from("world")];
    let v2 = v1;  // v1 is moved to v2
    
    // println!("{:?}", v1);  // This would cause a compile error
    println!("v2 now owns the data: {:?}", v2);
    
    println!("Key points:");
    println!("• Vectors automatically manage memory");
    println!("• Elements are dropped when vector is dropped");
    println!("• Ownership rules prevent use-after-free bugs");
}

fn main() {
    print_chapter_header("Chapter 8.1", "Storing Lists of Values with Vectors");

    println!("Vectors (Vec<T>) are dynamically-sized arrays that store multiple");
    println!("values of the same type in contiguous memory locations.");

    demonstrate_basic_vectors();
    demonstrate_element_access();
    demonstrate_borrowing_rules();
    demonstrate_iteration();
    demonstrate_enum_storage();
    demonstrate_shopping_cart();
    demonstrate_vector_methods();
    demonstrate_slices();
    demonstrate_complex_types();
    demonstrate_capacity_management();
    demonstrate_memory_management();

    println!("\n=== Key Takeaways ===");
    println!("• Use Vec::new() or vec![] macro to create vectors");
    println!("• Use push() to add elements, pop() to remove from end");
    println!("• Access elements with indexing [] or get() method");
    println!("• get() returns Option<T> for safe access");
    println!("• Vectors must be mutable to modify after creation");
    println!("• Borrowing rules prevent dangling references");
    println!("• Use enums to store different types in same vector");
    println!("• Vectors automatically manage memory allocation");
}