# Chapter 8.3: Storing Keys with Associated Values in Hash Maps

## Key Takeaways

### HashMap Fundamentals
- **HashMap<K, V>**: Stores key-value pairs using a hash function
- **Generic Types**: Keys (K) and values (V) must have consistent types
- **Heap Storage**: Data is stored on the heap, can grow dynamically
- **Fast Access**: O(1) average time complexity for access, insertion, and deletion

### Key-Value Mapping
- **Unique Keys**: Each key can only appear once in the map
- **Associated Values**: Each key maps to exactly one value
- **Hash Function**: Uses hashing to determine storage location
- **Flexible Types**: Keys and values can be any type that implements required traits

### Ownership Rules
- **Copy Types**: Values are copied into the hash map
- **Owned Types**: Values are moved into the hash map
- **References**: Must have valid lifetimes for the map's lifetime
- **Borrow Checker**: Ensures memory safety with borrowed data

### Important Syntax and Operators

#### HashMap Creation and Import
```rust
use std::collections::HashMap;

let mut map = HashMap::new();
```

#### Insertion and Access
```rust
map.insert(key, value);                    // Insert key-value pair
let value = map.get(&key);                 // Get value (returns Option<&V>)
let value = map[&key];                     // Direct access (panics if not found)
```

#### Entry API
```rust
map.entry(key).or_insert(value);           // Insert if key doesn't exist
let entry = map.entry(key).or_default();   // Insert default value if key doesn't exist
```

### Programming Concepts Introduced
- **Hash Tables**: Efficient key-value storage data structure
- **Entry API**: Ergonomic pattern for conditional insertion and updates
- **Iteration Patterns**: Multiple ways to iterate over maps
- **Hashing Functions**: How data is organized for fast access

### Code Examples and Patterns

#### Basic HashMap Creation and Usage
```rust
use std::collections::HashMap;

fn main() {
    // Creating a new hash map
    let mut scores = HashMap::new();
    
    // Inserting key-value pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // Accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue team score: {}", score);
    
    // Direct access (will panic if key doesn't exist)
    // let score = scores[&team_name];
}
```

#### Creating HashMap from Collections
```rust
use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    
    // Using zip and collect to create HashMap
    let mut scores: HashMap<_, _> = teams.into_iter()
        .zip(initial_scores.into_iter())
        .collect();
    
    println!("{:?}", scores);
}
```

#### Ownership and Hash Maps
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // println!("{}", field_name); // This would cause a compile error
    
    // For Copy types, values are copied
    let mut numbers = HashMap::new();
    let x = 5;
    let y = 10;
    numbers.insert(x, y);
    println!("x is still valid: {}", x); // Copy types remain valid
}
```

#### Iterating Over Hash Maps
```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 25);
    
    // Iterate over key-value pairs
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // Iterate over keys only
    for key in scores.keys() {
        println!("Team: {}", key);
    }
    
    // Iterate over values only
    for value in scores.values() {
        println!("Score: {}", value);
    }
    
    // Mutable iteration over values
    for value in scores.values_mut() {
        *value += 10; // Add 10 to each score
    }
    
    println!("Updated scores: {:?}", scores);
}
```

#### Updating Hash Maps
```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    
    // Overwriting a value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // Overwrites previous value
    println!("{:?}", scores); // {"Blue": 25}
    
    // Only inserting if key doesn't exist
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // Won't overwrite
    println!("{:?}", scores); // {"Blue": 25, "Yellow": 50}
}
```

#### Entry API for Complex Updates
```rust
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    // Count word occurrences
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", map); // {"hello": 1, "world": 2, "wonderful": 1}
}
```

#### Real-World Example: Student Grade Management
```rust
use std::collections::HashMap;

#[derive(Debug)]
struct Student {
    name: String,
    grade: f64,
}

impl Student {
    fn new(name: String, grade: f64) -> Self {
        Student { name, grade }
    }
}

fn main() {
    let mut students: HashMap<u32, Student> = HashMap::new();
    
    // Add students with ID as key
    students.insert(1001, Student::new("Alice".to_string(), 85.5));
    students.insert(1002, Student::new("Bob".to_string(), 92.0));
    students.insert(1003, Student::new("Charlie".to_string(), 78.5));
    
    // Look up student by ID
    if let Some(student) = students.get(&1002) {
        println!("Student 1002: {} has grade {}", student.name, student.grade);
    }
    
    // Update a grade
    if let Some(student) = students.get_mut(&1001) {
        student.grade = 88.0;
        println!("Updated {}'s grade to {}", student.name, student.grade);
    }
    
    // Calculate average grade
    let total_grade: f64 = students.values().map(|s| s.grade).sum();
    let average = total_grade / students.len() as f64;
    println!("Average grade: {:.2}", average);
    
    // Find students above average
    println!("Students above average:");
    for (id, student) in &students {
        if student.grade > average {
            println!("  ID {}: {} ({:.1})", id, student.name, student.grade);
        }
    }
}
```

#### Working with Different Key Types
```rust
use std::collections::HashMap;

fn main() {
    // String keys
    let mut string_map: HashMap<String, i32> = HashMap::new();
    string_map.insert("apple".to_string(), 5);
    
    // Integer keys
    let mut int_map: HashMap<i32, String> = HashMap::new();
    int_map.insert(1, "first".to_string());
    int_map.insert(2, "second".to_string());
    
    // Character keys
    let mut char_map: HashMap<char, i32> = HashMap::new();
    char_map.insert('a', 1);
    char_map.insert('b', 2);
    
    // Tuple keys (must implement Hash and Eq)
    let mut coord_map: HashMap<(i32, i32), String> = HashMap::new();
    coord_map.insert((0, 0), "origin".to_string());
    coord_map.insert((1, 1), "diagonal".to_string());
    
    println!("Coordinate (0, 0): {:?}", coord_map.get(&(0, 0)));
}
```

#### Error Handling with HashMap Operations
```rust
use std::collections::HashMap;

fn get_score(scores: &HashMap<String, i32>, team: &str) -> Option<i32> {
    scores.get(team).copied()
}

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Red".to_string(), 50);
    
    // Safe access with error handling
    match get_score(&scores, "Blue") {
        Some(score) => println!("Blue team score: {}", score),
        None => println!("Blue team not found"),
    }
    
    // Using unwrap_or for default values
    let yellow_score = scores.get("Yellow").copied().unwrap_or(0);
    println!("Yellow team score: {}", yellow_score);
    
    // Using entry API to handle missing keys
    let green_score = *scores.entry("Green".to_string()).or_insert(15);
    println!("Green team score: {}", green_score);
}
```

#### HashMap Methods and Operations
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    
    // Check if key exists
    println!("Contains key 'a': {}", map.contains_key("a"));
    
    // Get number of key-value pairs
    println!("Length: {}", map.len());
    
    // Check if empty
    println!("Is empty: {}", map.is_empty());
    
    // Remove a key-value pair
    if let Some(value) = map.remove("b") {
        println!("Removed 'b' with value: {}", value);
    }
    
    // Clear all entries
    // map.clear();
    
    // Retain only certain entries
    map.retain(|&k, &mut v| k == "a" || v > 2);
    println!("After retain: {:?}", map);
}
```

#### Advanced HashMap Usage: Caching
```rust
use std::collections::HashMap;

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
            println!("Computing value for key: {}", key);
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

fn main() {
    let mut cache = Cache::new();
    
    // First access - will compute
    println!("Value: {}", cache.get("user123"));
    
    // Second access - will use cached value
    println!("Value: {}", cache.get("user123"));
    
    println!("Cache size: {}", cache.size());
    
    // Invalidate and access again
    cache.invalidate("user123");
    println!("Value after invalidation: {}", cache.get("user123"));
}
```

#### HashMap with Custom Types
```rust
use std::collections::HashMap;

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

fn main() {
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
    
    // Look up product
    let product_id = ProductId { category: "electronics".to_string(), id: 1001 };
    if let Some(product) = inventory.get(&product_id) {
        println!("Found product: {} - ${}", product.name, product.price);
    }
    
    // Update stock
    if let Some(product) = inventory.get_mut(&product_id) {
        product.stock -= 1;
        println!("Updated stock to: {}", product.stock);
    }
}
```

### Practical Applications
- Configuration storage (key-value pairs)
- Caching computed results
- Counting occurrences (histograms)
- Database-like record storage
- Mapping relationships between entities
- Building indexes for fast lookups

### Common HashMap Operations
- `insert(key, value)` - Add or update key-value pair
- `get(&key)` - Retrieve value by key (returns Option)
- `get_mut(&key)` - Get mutable reference to value
- `remove(&key)` - Remove key-value pair
- `contains_key(&key)` - Check if key exists
- `entry(key)` - Get entry for advanced manipulation
- `len()` - Number of key-value pairs
- `is_empty()` - Check if map is empty
- `clear()` - Remove all entries

### Entry API Patterns
- `entry(key).or_insert(value)` - Insert if key doesn't exist
- `entry(key).or_default()` - Insert default value if key doesn't exist
- `entry(key).and_modify(|v| *v += 1).or_insert(1)` - Update existing or insert new

### Performance Characteristics
- **Average Case**: O(1) for insertion, deletion, and lookup
- **Worst Case**: O(n) if many hash collisions occur
- **Memory**: Uses more memory than arrays due to hash table overhead
- **Hashing**: Default hasher is cryptographically secure but slower

### Integration with Previous Chapters
- Uses ownership rules for key and value management
- Keys must implement `Hash` and `Eq` traits
- Values can be any type, including structs and enums
- Works well with pattern matching for complex logic

### Community Conventions and Idioms
- Use `HashMap` when you need fast key-based lookups
- Use `BTreeMap` when you need sorted keys
- Prefer `entry()` API for conditional insertions and updates
- Use `HashMap::with_capacity(n)` when you know approximate size
- Consider `fnv` or `ahash` crates for performance-critical applications

### Hash Requirements for Keys
Keys must implement:
- `Hash` trait - for computing hash values  
- `Eq` trait - for equality comparison
- Most standard types implement these automatically
- Custom types need `#[derive(Hash, Eq, PartialEq)]`

### Personal Notes
- HashMap is essential for many programming patterns
- The entry API is extremely powerful for complex update logic
- Understanding ownership is crucial when working with HashMap
- Hash maps are generally preferred over linear searches for lookups
- The default hasher prioritizes security over raw performance

Official Chapter: https://doc.rust-lang.org/book/ch08-03-hash-maps.html

---
*Completed: ✓*