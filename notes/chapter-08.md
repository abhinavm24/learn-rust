# Chapter 8: Common Collections

## Key Takeaways

### Collection Types
- **Vector (Vec<T>)**: Growable array, stores elements in contiguous memory
- **String**: Growable, UTF-8 encoded text, wrapper around Vec<u8>
- **HashMap<K, V>**: Key-value store with fast lookup, insertion, and deletion
- **Heap Allocation**: All collections store data on the heap

### Memory Management
- **Ownership**: Collections own their data
- **Growth Strategy**: Automatic resizing when capacity is exceeded
- **Borrowing**: Can borrow elements without taking ownership
- **Drop Semantics**: Automatic cleanup when collections go out of scope

### Performance Characteristics
- **Vector**: O(1) access by index, O(1) amortized push, O(n) insertion
- **String**: O(1) amortized push, O(n) insertion, complex indexing
- **HashMap**: O(1) average case for get/insert, O(n) worst case
- **Memory Efficiency**: Contiguous storage for vectors, hash table for maps

### Use Cases
- **Vector**: Lists, stacks, queues, buffers
- **String**: Text processing, user input, file paths
- **HashMap**: Caches, counters, lookups, configuration

## Chapter Structure

### 8.1: Storing Lists of Values with Vectors
```rust
// Creating vectors
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];  // Using vec! macro

// Adding elements
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);

// Reading elements
let v = vec![1, 2, 3, 4, 5];

// Method 1: Indexing (panics if out of bounds)
let third: &i32 = &v[2];
println!("The third element is {}", third);

// Method 2: get method (returns Option)
match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}

// Iterating over values
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}

// Iterating with mutation
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;  // Dereference to change the value
}

// Using enums to store multiple types
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];

// Vector methods
let mut v = vec![1, 2, 3];
v.push(4);                    // Add element
let last = v.pop();           // Remove and return last element
let len = v.len();            // Get length
let is_empty = v.is_empty();  // Check if empty
v.clear();                    // Remove all elements

// Vector with capacity
let mut v = Vec::with_capacity(10);  // Pre-allocate space
println!("Capacity: {}", v.capacity());
```

### 8.2: Storing UTF-8 Encoded Text with Strings
```rust
// Creating strings
let mut s = String::new();
let s = String::from("initial contents");
let s = "initial contents".to_string();

// Updating strings
let mut s = String::from("foo");
s.push_str("bar");      // Append string slice
s.push('!');            // Append single character

// Concatenation
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // s1 has been moved and can no longer be used

// Using format! macro
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);

// String slicing (be careful with UTF-8!)
let hello = "Здравствуйте";
let s = &hello[0..4];  // "Зд" (each Cyrillic char is 2 bytes)

// Iterating over strings
let hello = "नमस्ते";

// By characters
for c in hello.chars() {
    println!("{}", c);
}

// By bytes
for b in hello.bytes() {
    println!("{}", b);
}

// String methods
let s = String::from("  hello world  ");
let trimmed = s.trim();                    // "hello world"
let uppercase = s.to_uppercase();          // "  HELLO WORLD  "
let replaced = s.replace("world", "Rust"); // "  hello Rust  "
let contains = s.contains("hello");        // true
let starts = s.starts_with("  hello");     // true
```

### 8.3: Storing Keys with Associated Values in Hash Maps
```rust
use std::collections::HashMap;

// Creating hash maps
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Creating from vectors
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let mut scores: HashMap<_, _> = teams.into_iter()
    .zip(initial_scores.into_iter())
    .collect();

// Accessing values
let team_name = String::from("Blue");
let score = scores.get(&team_name);  // Returns Option<&V>

match score {
    Some(s) => println!("Score: {}", s),
    None => println!("Team not found"),
}

// Iterating over hash map
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// Updating hash map
let mut scores = HashMap::new();

// Overwriting a value
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);  // Overwrites 10

// Only inserting if key has no value
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);  // Won't overwrite

// Updating based on old value
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;  // Dereference to modify the value
}

println!("{:?}", map);  // {"hello": 1, "world": 2, "wonderful": 1}
```

## Advanced Collection Patterns

### Vector Operations
```rust
// Searching and filtering
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// Find first even number
let first_even = numbers.iter().find(|&&x| x % 2 == 0);
println!("First even: {:?}", first_even);

// Filter even numbers
let evens: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
println!("Evens: {:?}", evens);

// Transform and collect
let squares: Vec<_> = numbers.iter().map(|x| x * x).collect();
println!("Squares: {:?}", squares);

// Reduce/fold
let sum: i32 = numbers.iter().sum();
let product: i32 = numbers.iter().product();
let custom_reduce = numbers.iter().fold(0, |acc, &x| acc + x * x);

// Sorting
let mut words = vec!["banana", "apple", "cherry", "date"];
words.sort();  // Alphabetical sort
println!("Sorted: {:?}", words);

words.sort_by(|a, b| a.len().cmp(&b.len()));  // Sort by length
println!("By length: {:?}", words);

// Binary search (only on sorted vectors)
let numbers = vec![1, 3, 5, 7, 9, 11, 13];
match numbers.binary_search(&7) {
    Ok(index) => println!("Found at index: {}", index),
    Err(index) => println!("Would be inserted at index: {}", index),
}

// Deduplication
let mut numbers = vec![1, 2, 2, 3, 3, 3, 4];
numbers.dedup();
println!("Deduplicated: {:?}", numbers);  // [1, 2, 3, 4]

// Splitting and joining
let numbers = vec![1, 2, 3, 4, 5];
let (left, right) = numbers.split_at(2);
println!("Left: {:?}, Right: {:?}", left, right);  // [1, 2], [3, 4, 5]

// Chunking
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
for chunk in numbers.chunks(3) {
    println!("Chunk: {:?}", chunk);  // [1, 2, 3], [4, 5, 6], [7, 8]
}
```

### String Processing
```rust
// Advanced string operations
let text = "The quick brown fox jumps over the lazy dog";

// Split into words
let words: Vec<&str> = text.split_whitespace().collect();
println!("Words: {:?}", words);

// Split by delimiter
let csv = "apple,banana,cherry,date";
let fruits: Vec<&str> = csv.split(',').collect();
println!("Fruits: {:?}", fruits);

// Lines processing
let multiline = "Line 1\nLine 2\nLine 3";
for (i, line) in multiline.lines().enumerate() {
    println!("Line {}: {}", i + 1, line);
}

// Pattern replacement
let text = "Hello, world! Hello, Rust!";
let replaced = text.replace("Hello", "Hi");
println!("Replaced: {}", replaced);

// Case conversion
let text = "Hello World";
println!("Uppercase: {}", text.to_uppercase());
println!("Lowercase: {}", text.to_lowercase());

// String formatting
let name = "Alice";
let age = 30;
let formatted = format!("Name: {}, Age: {}", name, age);
let formatted_positional = format!("{1} is {0} years old", age, name);
let formatted_named = format!("{name} is {age} years old", name=name, age=age);

// String validation
fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

fn count_vowels(text: &str) -> usize {
    text.chars()
        .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
}

// Unicode handling
let unicode_text = "こんにちは世界";  // "Hello world" in Japanese
println!("Character count: {}", unicode_text.chars().count());
println!("Byte count: {}", unicode_text.len());

for (i, c) in unicode_text.char_indices() {
    println!("Character {} at byte position {}", c, i);
}
```

### HashMap Advanced Usage
```rust
use std::collections::HashMap;

// Custom key types
#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

let mut point_map = HashMap::new();
point_map.insert(Point { x: 0, y: 0 }, "Origin");
point_map.insert(Point { x: 1, y: 1 }, "Point (1,1)");

// Complex value types
#[derive(Debug)]
struct PlayerStats {
    score: u32,
    level: u8,
    health: f32,
}

let mut players = HashMap::new();
players.insert("Alice".to_string(), PlayerStats {
    score: 1500,
    level: 15,
    health: 85.5,
});

// Grouping data
let words = vec!["apple", "banana", "apricot", "blueberry", "cherry"];
let mut by_first_letter: HashMap<char, Vec<&str>> = HashMap::new();

for word in words {
    let first_letter = word.chars().next().unwrap();
    by_first_letter.entry(first_letter).or_insert(Vec::new()).push(word);
}

println!("{:?}", by_first_letter);

// Counting occurrences
fn count_occurrences<T: std::hash::Hash + Eq>(items: Vec<T>) -> HashMap<T, usize> {
    let mut counts = HashMap::new();
    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts
}

let numbers = vec![1, 2, 3, 2, 1, 3, 1];
let counts = count_occurrences(numbers);
println!("Counts: {:?}", counts);

// Caching with HashMap
struct Cache<K, V> {
    data: HashMap<K, V>,
    capacity: usize,
}

impl<K: std::hash::Hash + Eq, V> Cache<K, V> {
    fn new(capacity: usize) -> Self {
        Cache {
            data: HashMap::new(),
            capacity,
        }
    }
    
    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }
    
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.data.len() >= self.capacity && !self.data.contains_key(&key) {
            // Simple eviction: remove arbitrary element
            // In real implementation, you'd use LRU or similar
            let first_key = self.data.keys().next().cloned();
            if let Some(k) = first_key {
                self.data.remove(&k);
            }
        }
        self.data.insert(key, value)
    }
}

// Usage
let mut cache = Cache::new(3);
cache.insert("key1", "value1");
cache.insert("key2", "value2");
cache.insert("key3", "value3");
cache.insert("key4", "value4");  // This will evict one item

println!("Cache size: {}", cache.data.len());
```

## Performance Considerations

### Vector Performance
```rust
// Pre-allocate when size is known
let mut v = Vec::with_capacity(1000);  // Avoids reallocations
for i in 0..1000 {
    v.push(i);
}

// Efficient iteration
let numbers = vec![1, 2, 3, 4, 5];

// ✅ Efficient: iterator doesn't allocate
let sum: i32 = numbers.iter().sum();

// ❌ Less efficient: creates intermediate vector
let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
let sum_doubled: i32 = doubled.iter().sum();

// ✅ More efficient: chain operations
let sum_doubled: i32 = numbers.iter().map(|x| x * 2).sum();

// Bulk operations
let mut v1 = vec![1, 2, 3];
let v2 = vec![4, 5, 6];
v1.extend(v2);  // More efficient than multiple push calls

// Reserve capacity before adding many elements
let mut v = Vec::new();
v.reserve(1000);  // Reserve space for 1000 elements
for i in 0..1000 {
    v.push(i);
}
```

### String Performance
```rust
// Efficient string building
let parts = vec!["Hello", " ", "world", "!"];

// ❌ Inefficient: creates many intermediate strings
let mut result = String::new();
for part in &parts {
    result = result + part;  // Creates new string each time
}

// ✅ Efficient: preallocate and extend
let mut result = String::with_capacity(parts.iter().map(|s| s.len()).sum());
for part in &parts {
    result.push_str(part);
}

// ✅ Most efficient: use join
let result = parts.join("");

// String formatting performance
let name = "Alice";
let age = 30;

// For simple concatenation, format! is fine
let message = format!("Hello, {}! You are {} years old.", name, age);

// For building larger strings, use a buffer
let mut buffer = String::with_capacity(100);
use std::fmt::Write;
write!(&mut buffer, "Hello, {}! You are {} years old.", name, age).unwrap();
```

### HashMap Performance
```rust
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// Custom hash implementation for better performance
#[derive(Debug, Eq, PartialEq)]
struct FastKey {
    id: u64,
}

impl Hash for FastKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Simple hash for integer keys
        state.write_u64(self.id);
    }
}

// Pre-size HashMap when possible
let mut map = HashMap::with_capacity(1000);

// Batch operations
let pairs = vec![("key1", 1), ("key2", 2), ("key3", 3)];
let map: HashMap<_, _> = pairs.into_iter().collect();

// Avoid unnecessary cloning
let map = HashMap::new();
let key = "expensive_key".to_string();

// ❌ Clones key even if it exists
// if !map.contains_key(&key) {
//     map.insert(key.clone(), value);
// }

// ✅ Only clones if needed
// map.entry(key).or_insert(value);
```

## Real-World Examples

### Log Processing
```rust
use std::collections::HashMap;

#[derive(Debug)]
struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

fn process_logs(logs: Vec<LogEntry>) -> HashMap<String, Vec<LogEntry>> {
    let mut by_level = HashMap::new();
    
    for log in logs {
        by_level.entry(log.level.clone()).or_insert(Vec::new()).push(log);
    }
    
    by_level
}

fn count_log_levels(logs: &[LogEntry]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    
    for log in logs {
        *counts.entry(log.level.clone()).or_insert(0) += 1;
    }
    
    counts
}

// Usage
let logs = vec![
    LogEntry {
        timestamp: "2023-01-01T10:00:00".to_string(),
        level: "INFO".to_string(),
        message: "Application started".to_string(),
    },
    LogEntry {
        timestamp: "2023-01-01T10:01:00".to_string(),
        level: "ERROR".to_string(),
        message: "Connection failed".to_string(),
    },
    LogEntry {
        timestamp: "2023-01-01T10:02:00".to_string(),
        level: "INFO".to_string(),
        message: "Request processed".to_string(),
    },
];

let grouped = process_logs(logs.clone());
let counts = count_log_levels(&logs);

println!("Grouped logs: {:#?}", grouped);
println!("Log counts: {:?}", counts);
```

### Data Processing Pipeline
```rust
#[derive(Debug, Clone)]
struct Sale {
    product: String,
    amount: f64,
    region: String,
    date: String,
}

fn analyze_sales(sales: Vec<Sale>) -> HashMap<String, f64> {
    sales
        .into_iter()
        .filter(|sale| sale.amount > 0.0)  // Valid sales only
        .fold(HashMap::new(), |mut acc, sale| {
            *acc.entry(sale.region).or_insert(0.0) += sale.amount;
            acc
        })
}

fn top_products(sales: &[Sale], n: usize) -> Vec<(String, f64)> {
    let mut product_totals = HashMap::new();
    
    for sale in sales {
        *product_totals.entry(sale.product.clone()).or_insert(0.0) += sale.amount;
    }
    
    let mut products: Vec<_> = product_totals.into_iter().collect();
    products.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    products.into_iter().take(n).collect()
}

// Text analysis
fn analyze_text(text: &str) -> HashMap<String, usize> {
    text.split_whitespace()
        .map(|word| word.to_lowercase().trim_matches(|c: char| !c.is_alphabetic()).to_string())
        .filter(|word| !word.is_empty())
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
}

fn most_common_words(word_counts: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut words: Vec<_> = word_counts.iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    
    words.sort_by(|a, b| b.1.cmp(&a.1));
    words.into_iter().take(n).collect()
}
```

## Error Handling with Collections

### Safe Collection Access
```rust
fn safe_get_element(v: &Vec<i32>, index: usize) -> Option<i32> {
    v.get(index).copied()
}

fn safe_string_char(s: &str, index: usize) -> Option<char> {
    s.chars().nth(index)
}

fn safe_hashmap_lookup(map: &HashMap<String, i32>, key: &str) -> Option<i32> {
    map.get(key).copied()
}

// Result-based error handling
#[derive(Debug)]
enum CollectionError {
    IndexOutOfBounds,
    KeyNotFound,
    InvalidInput,
}

fn get_element_or_error(v: &Vec<i32>, index: usize) -> Result<i32, CollectionError> {
    v.get(index)
        .copied()
        .ok_or(CollectionError::IndexOutOfBounds)
}

fn parse_and_sum_numbers(input: &str) -> Result<i32, CollectionError> {
    let numbers: Result<Vec<i32>, _> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect();
    
    match numbers {
        Ok(nums) => Ok(nums.iter().sum()),
        Err(_) => Err(CollectionError::InvalidInput),
    }
}
```

## Testing Collections
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_vector_operations() {
        let mut v = vec![1, 2, 3];
        v.push(4);
        assert_eq!(v.len(), 4);
        assert_eq!(v[3], 4);
        
        let popped = v.pop();
        assert_eq!(popped, Some(4));
        assert_eq!(v.len(), 3);
    }
    
    #[test]
    fn test_string_operations() {
        let mut s = String::from("Hello");
        s.push_str(", world!");
        assert_eq!(s, "Hello, world!");
        
        let uppercase = s.to_uppercase();
        assert_eq!(uppercase, "HELLO, WORLD!");
    }
    
    #[test]
    fn test_hashmap_operations() {
        let mut map = HashMap::new();
        map.insert("key1", 100);
        map.insert("key2", 200);
        
        assert_eq!(map.get("key1"), Some(&100));
        assert_eq!(map.get("key3"), None);
        
        *map.entry("key1").or_insert(0) += 50;
        assert_eq!(map.get("key1"), Some(&150));
    }
    
    #[test]
    fn test_word_count() {
        let text = "hello world hello rust world";
        let counts = analyze_text(text);
        
        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&2));
        assert_eq!(counts.get("rust"), Some(&1));
    }
}
```

## Best Practices

### Choosing the Right Collection
```rust
// ✅ Use Vec for:
// - Ordered data
// - Index-based access
// - Stack-like operations (push/pop)
let mut stack = Vec::new();
stack.push(1);
let top = stack.pop();

// ✅ Use String for:
// - Text manipulation
// - Building strings dynamically
// - UTF-8 text storage
let mut message = String::new();
message.push_str("Hello, ");
message.push_str("world!");

// ✅ Use HashMap for:
// - Key-value associations
// - Fast lookup by key
// - Counting/grouping
let mut counts = HashMap::new();
for item in items {
    *counts.entry(item).or_insert(0) += 1;
}

// ✅ Consider other collections for specific needs:
// - VecDeque for double-ended queue
// - HashSet for unique values
// - BTreeMap for sorted keys
// - LinkedList for frequent insertion/removal
```

### Memory Efficiency
```rust
// ✅ Pre-allocate when size is known
let mut v = Vec::with_capacity(expected_size);
let mut s = String::with_capacity(expected_length);
let mut map = HashMap::with_capacity(expected_entries);

// ✅ Use appropriate types
// For small collections, Vec might be faster than HashMap
if expected_size < 10 {
    let pairs = vec![("key1", "value1"), ("key2", "value2")];
    // Linear search is fast for small collections
}

// ✅ Avoid unnecessary clones
fn process_strings(strings: &[String]) -> Vec<String> {
    strings.iter()
        .filter(|s| s.len() > 5)
        .cloned()  // Only clone filtered items
        .collect()
}
```

Official Chapter: https://doc.rust-lang.org/book/ch08-00-common-collections.html

---
*Completed: ✓*