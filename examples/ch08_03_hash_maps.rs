//! Chapter 8.3: Storing Keys with Associated Values in Hash Maps
//! 
//! This example demonstrates HashMap<K, V>, which stores key-value pairs using
//! a hash function. Hash maps are useful when you want to look up data not by
//! using an index, but by using a key that can be of any type. They provide
//! average O(1) time complexity for insertions, deletions, and lookups.
//!
//! Key concepts:
//! - Creating and manipulating hash maps
//! - Different ways to access values
//! - Ownership rules with hash maps
//! - Entry API for advanced operations
//! - Iterating over hash maps

use std::collections::HashMap;
use rust_book_examples::print_chapter_header;

#[derive(Debug)]
struct Student {
    name: String,
    grade: f64,
    age: u32,
}

impl Student {
    fn new(name: &str, grade: f64, age: u32) -> Self {
        Student {
            name: name.to_string(),
            grade,
            age,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct ProductId {
    category: String,
    id: u32,
}

#[derive(Debug)]
struct Product {
    name: String,
    price: f64,
    stock: i32,
}

struct Cache {
    data: HashMap<String, String>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            data: HashMap::new(),
        }
    }
    
    fn get(&mut self, key: &str) -> &str {
        self.data.entry(key.to_string()).or_insert_with(|| {
            // Simulate expensive computation
            println!("  Computing value for key: {}", key);
            format!("computed_value_for_{}", key)
        })
    }
    
    fn invalidate(&mut self, key: &str) {
        self.data.remove(key);
    }
    
    fn size(&self) -> usize {
        self.data.len()
    }
}

fn get_score(scores: &HashMap<String, i32>, team: &str) -> Option<i32> {
    scores.get(team).copied()
}

fn demonstrate_basic_creation() {
    println!("\n=== Basic HashMap Creation and Usage ===");
    
    // Creating a new hash map
    let mut scores = HashMap::new();
    
    // Inserting key-value pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 25);
    
    println!("Scores: {:?}", scores);
    
    // Accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue team score: {}", score);
    
    // Using unwrap_or for default values
    let green_score = scores.get("Green").copied().unwrap_or(0);
    println!("Green team score (default): {}", green_score);
    
    // Direct access (would panic if key doesn't exist)
    // let score = scores[&team_name]; // Uncomment to see panic
    println!("Note: Direct indexing with [] would panic if key doesn't exist!");
}

fn demonstrate_creation_from_collections() {
    println!("\n=== Creating HashMap from Collections ===");
    
    let teams = vec![String::from("Blue"), String::from("Yellow"), String::from("Red")];
    let initial_scores = vec![10, 50, 25];
    
    // Using zip and collect to create HashMap
    let scores: HashMap<_, _> = teams.into_iter()
        .zip(initial_scores.into_iter())
        .collect();
    
    println!("HashMap from vectors: {:?}", scores);
    
    // Alternative: create from tuples
    let scores: HashMap<&str, i32> = [
        ("Alice", 95),
        ("Bob", 87),
        ("Charlie", 92),
    ].iter().cloned().collect();
    
    println!("HashMap from array of tuples: {:?}", scores);
}

fn demonstrate_ownership() {
    println!("\n=== Ownership and Hash Maps ===");
    
    let mut map = HashMap::new();
    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    
    println!("Before insert - field_name: '{}', field_value: '{}'", field_name, field_value);
    
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // println!("{}", field_name); // This would cause a compile error
    
    println!("After insert - strings are moved into HashMap: {:?}", map);
    
    // For Copy types, values are copied
    let mut numbers = HashMap::new();
    let x = 5;
    let y = 10;
    numbers.insert(x, y);
    println!("Copy types remain valid - x: {}, y: {}", x, y);
    println!("Numbers map: {:?}", numbers);
    
    // Working with references (requires lifetime considerations)
    let mut ref_map = HashMap::new();
    let key = "hello";
    let value = "world";
    ref_map.insert(key, value);
    println!("Reference map: {:?}", ref_map);
    println!("Original references still valid - key: '{}', value: '{}'", key, value);
}

fn demonstrate_iteration() {
    println!("\n=== Iterating Over Hash Maps ===");
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 25);
    scores.insert(String::from("Green"), 35);
    
    // Iterate over key-value pairs
    println!("All teams and scores:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }
    
    // Iterate over keys only
    println!("All teams:");
    for key in scores.keys() {
        println!("  Team: {}", key);
    }
    
    // Iterate over values only
    println!("All scores:");
    for value in scores.values() {
        println!("  Score: {}", value);
    }
    
    // Mutable iteration over values
    println!("Adding 10 points to each team:");
    for value in scores.values_mut() {
        *value += 10; // Add 10 to each score
    }
    
    println!("Updated scores: {:?}", scores);
    
    // Collecting keys and values
    let teams: Vec<String> = scores.keys().cloned().collect();
    let all_scores: Vec<i32> = scores.values().cloned().collect();
    println!("All teams: {:?}", teams);
    println!("All scores: {:?}", all_scores);
}

fn demonstrate_updating() {
    println!("\n=== Updating Hash Maps ===");
    
    let mut scores = HashMap::new();
    
    // Overwriting a value
    scores.insert(String::from("Blue"), 10);
    println!("Initial: {:?}", scores);
    
    scores.insert(String::from("Blue"), 25); // Overwrites previous value
    println!("After overwrite: {:?}", scores);
    
    // Only inserting if key doesn't exist
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // Won't overwrite
    println!("After conditional inserts: {:?}", scores);
    
    // Check if key exists before inserting
    if !scores.contains_key("Green") {
        scores.insert(String::from("Green"), 30);
        println!("Added Green team: {:?}", scores);
    }
}

fn demonstrate_entry_api() {
    println!("\n=== Entry API for Complex Updates ===");
    
    let text = "hello world wonderful world hello rust world";
    let mut map = HashMap::new();
    
    // Count word occurrences
    println!("Counting words in: '{}'", text);
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("Word counts: {:?}", map);
    
    // Using entry with more complex logic
    let mut scores = HashMap::new();
    let game_results = vec![
        ("Alice", 10),
        ("Bob", 15),
        ("Alice", 25),
        ("Charlie", 30),
        ("Bob", 20),
        ("Alice", 5),
    ];
    
    println!("Processing game results:");
    for (player, score) in game_results {
        println!("  {} scored {} points", player, score);
        scores.entry(player)
            .and_modify(|total| *total += score)
            .or_insert(score);
    }
    
    println!("Final scores: {:?}", scores);
    
    // Find the winner
    if let Some((winner, &max_score)) = scores.iter().max_by_key(|(_, &score)| score) {
        println!("Winner: {} with {} points!", winner, max_score);
    }
}

fn demonstrate_student_management() {
    println!("\n=== Real-World Example: Student Grade Management ===");
    
    let mut students: HashMap<u32, Student> = HashMap::new();
    
    // Add students with ID as key
    students.insert(1001, Student::new("Alice", 85.5, 20));
    students.insert(1002, Student::new("Bob", 92.0, 19));
    students.insert(1003, Student::new("Charlie", 78.5, 21));
    students.insert(1004, Student::new("Diana", 96.5, 20));
    
    // Look up student by ID
    if let Some(student) = students.get(&1002) {
        println!("Student 1002: {} (age {}) has grade {}", 
                 student.name, student.age, student.grade);
    }
    
    // Update a grade
    if let Some(student) = students.get_mut(&1001) {
        let old_grade = student.grade;
        student.grade = 88.0;
        println!("Updated {}'s grade from {} to {}", 
                 student.name, old_grade, student.grade);
    }
    
    // Calculate statistics
    let total_grade: f64 = students.values().map(|s| s.grade).sum();
    let average = total_grade / students.len() as f64;
    println!("Average grade: {:.2}", average);
    
    // Find students above average
    println!("Students above average ({:.2}):", average);
    for (id, student) in &students {
        if student.grade > average {
            println!("  ID {}: {} - {:.1}", id, student.name, student.grade);
        }
    }
    
    // Group by age
    let mut age_groups: HashMap<u32, Vec<&str>> = HashMap::new();
    for student in students.values() {
        age_groups.entry(student.age).or_insert_with(Vec::new).push(&student.name);
    }
    
    println!("Students grouped by age:");
    for (age, names) in &age_groups {
        println!("  Age {}: {:?}", age, names);
    }
}

fn demonstrate_different_key_types() {
    println!("\n=== Working with Different Key Types ===");
    
    // String keys
    let mut string_map: HashMap<String, i32> = HashMap::new();
    string_map.insert("apple".to_string(), 5);
    string_map.insert("banana".to_string(), 3);
    println!("String keys: {:?}", string_map);
    
    // Integer keys
    let mut int_map: HashMap<i32, String> = HashMap::new();
    int_map.insert(1, "first".to_string());
    int_map.insert(2, "second".to_string());
    int_map.insert(42, "answer".to_string());
    println!("Integer keys: {:?}", int_map);
    
    // Character keys
    let mut char_map: HashMap<char, i32> = HashMap::new();
    char_map.insert('a', 1);
    char_map.insert('b', 2);
    char_map.insert('z', 26);
    println!("Character keys: {:?}", char_map);
    
    // Tuple keys (must implement Hash and Eq)
    let mut coord_map: HashMap<(i32, i32), String> = HashMap::new();
    coord_map.insert((0, 0), "origin".to_string());
    coord_map.insert((1, 1), "diagonal".to_string());
    coord_map.insert((-1, 5), "quadrant II".to_string());
    
    println!("Coordinate map:");
    for ((x, y), description) in &coord_map {
        println!("  ({}, {}): {}", x, y, description);
    }
    
    // Looking up coordinates
    if let Some(description) = coord_map.get(&(0, 0)) {
        println!("Point (0, 0) is: {}", description);
    }
}

fn demonstrate_error_handling() {
    println!("\n=== Error Handling with HashMap Operations ===");
    
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Red".to_string(), 50);
    scores.insert("Yellow".to_string(), 25);
    
    // Safe access with error handling
    match get_score(&scores, "Blue") {
        Some(score) => println!("Blue team score: {}", score),
        None => println!("Blue team not found"),
    }
    
    match get_score(&scores, "Green") {
        Some(score) => println!("Green team score: {}", score),
        None => println!("Green team not found"),
    }
    
    // Using unwrap_or for default values
    let purple_score = scores.get("Purple").copied().unwrap_or(0);
    println!("Purple team score (with default): {}", purple_score);
    
    // Using entry API to handle missing keys
    let orange_score = *scores.entry("Orange".to_string()).or_insert(15);
    println!("Orange team score (inserted if missing): {}", orange_score);
    
    // Try to get or insert with computation
    let computed_score = *scores.entry("Pink".to_string()).or_insert_with(|| {
        println!("  Computing score for Pink team...");
        42 // Computed value
    });
    println!("Pink team computed score: {}", computed_score);
    
    println!("Final scores: {:?}", scores);
}

fn demonstrate_hash_map_methods() {
    println!("\n=== HashMap Methods and Operations ===");
    
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    map.insert("d", 4);
    
    println!("Initial map: {:?}", map);
    
    // Check if key exists
    println!("Contains key 'a': {}", map.contains_key("a"));
    println!("Contains key 'z': {}", map.contains_key("z"));
    
    // Get number of key-value pairs
    println!("Length: {}", map.len());
    
    // Check if empty
    println!("Is empty: {}", map.is_empty());
    
    // Remove a key-value pair
    if let Some(value) = map.remove("b") {
        println!("Removed 'b' with value: {}", value);
    }
    println!("After removal: {:?}", map);
    
    // Retain only certain entries
    let original_len = map.len();
    map.retain(|&k, &mut v| k == "a" || v > 2);
    println!("After retain (keep 'a' or value > 2): {:?}", map);
    println!("Removed {} entries", original_len - map.len());
    
    // Clear all entries (commented to show final state)
    // map.clear();
    // println!("After clear: {:?}", map);
}

fn demonstrate_caching_example() {
    println!("\n=== Advanced HashMap Usage: Caching ===");
    
    let mut cache = Cache::new();
    
    // First access - will compute
    println!("Getting value for 'user123':");
    println!("Value: {}", cache.get("user123"));
    
    // Second access - will use cached value
    println!("Getting value for 'user123' again:");
    println!("Value: {}", cache.get("user123"));
    
    // Different key - will compute
    println!("Getting value for 'user456':");
    println!("Value: {}", cache.get("user456"));
    
    println!("Cache size: {}", cache.size());
    
    // Invalidate and access again
    cache.invalidate("user123");
    println!("After invalidating 'user123', getting it again:");
    println!("Value: {}", cache.get("user123"));
    
    println!("Final cache size: {}", cache.size());
}

fn demonstrate_custom_types() {
    println!("\n=== HashMap with Custom Types ===");
    
    let mut inventory: HashMap<ProductId, Product> = HashMap::new();
    
    // Add products
    inventory.insert(
        ProductId { category: "electronics".to_string(), id: 1001 },
        Product { name: "Laptop".to_string(), price: 999.99, stock: 5 }
    );
    
    inventory.insert(
        ProductId { category: "books".to_string(), id: 2001 },
        Product { name: "Rust Book".to_string(), price: 39.99, stock: 15 }
    );
    
    inventory.insert(
        ProductId { category: "electronics".to_string(), id: 1002 },
        Product { name: "Mouse".to_string(), price: 25.99, stock: 20 }
    );
    
    // Look up product
    let product_id = ProductId { category: "electronics".to_string(), id: 1001 };
    if let Some(product) = inventory.get(&product_id) {
        println!("Found product: {} - ${:.2} (stock: {})", 
                 product.name, product.price, product.stock);
    }
    
    // Update stock
    if let Some(product) = inventory.get_mut(&product_id) {
        product.stock -= 1;
        println!("Sold one {}. New stock: {}", product.name, product.stock);
    }
    
    // List all products by category
    let mut categories: HashMap<String, Vec<&Product>> = HashMap::new();
    for (id, product) in &inventory {
        categories.entry(id.category.clone())
            .or_insert_with(Vec::new)
            .push(product);
    }
    
    println!("Inventory by category:");
    for (category, products) in &categories {
        println!("  {}:", category);
        for product in products {
            println!("    {} - ${:.2}", product.name, product.price);
        }
    }
    
    // Calculate total inventory value
    let total_value: f64 = inventory.values()
        .map(|product| product.price * product.stock as f64)
        .sum();
    println!("Total inventory value: ${:.2}", total_value);
}

fn main() {
    print_chapter_header("Chapter 8.3", "Storing Keys with Associated Values in Hash Maps");

    println!("HashMap<K, V> stores key-value pairs using a hash function for fast lookups.");
    println!("Hash maps provide O(1) average time complexity for insertions, deletions, and lookups.");

    demonstrate_basic_creation();
    demonstrate_creation_from_collections();
    demonstrate_ownership();
    demonstrate_iteration();
    demonstrate_updating();
    demonstrate_entry_api();
    demonstrate_student_management();
    demonstrate_different_key_types();
    demonstrate_error_handling();
    demonstrate_hash_map_methods();
    demonstrate_caching_example();
    demonstrate_custom_types();

    println!("\n=== Key Takeaways ===");
    println!("• HashMap stores key-value pairs with fast O(1) average access");
    println!("• Keys must implement Hash and Eq traits");
    println!("• Use entry() API for advanced insertion and update operations");
    println!("• HashMap takes ownership of keys and values (unless they're Copy types)");
    println!("• Iteration order is not guaranteed to be consistent");
    println!("• Use get() method for safe access that returns Option<&V>");
    println!("• Perfect for counting, caching, and fast key-based lookups");
}