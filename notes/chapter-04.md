# Chapter 4: Understanding Ownership

## Key Takeaways

### Ownership Fundamentals
- **Memory Safety**: Rust's ownership system prevents memory leaks and dangling pointers
- **No Garbage Collection**: Memory management without runtime overhead
- **Compile-Time Guarantees**: Memory safety enforced at compile time
- **Zero-Cost Abstractions**: Safety with no performance penalty

### Core Ownership Rules
1. **Each value has exactly one owner**
2. **When owner goes out of scope, value is dropped**
3. **Ownership can be moved or borrowed**
4. **References allow access without ownership transfer**

### Memory Management
- **Stack vs Heap**: Different allocation strategies for different data
- **RAII**: Resource Acquisition Is Initialization pattern
- **Automatic Cleanup**: Drop trait automatically cleans up resources
- **Predictable Performance**: No unpredictable garbage collection pauses

### Borrowing System
- **Immutable References**: Multiple readers allowed
- **Mutable References**: Exclusive write access
- **Lifetime Management**: Ensures references remain valid
- **Borrow Checker**: Compile-time validation of reference safety

## Chapter Structure

### 4.1: What is Ownership?
```rust
// Ownership transfer (move)
let s1 = String::from("hello");
let s2 = s1;  // s1 is no longer valid

// Clone for deep copy
let s1 = String::from("hello");
let s2 = s1.clone();  // Both s1 and s2 are valid

// Copy types (stack data)
let x = 5;
let y = x;  // Both x and y are valid (Copy trait)

// Function ownership
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}  // some_string goes out of scope and is dropped

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}  // some_integer goes out of scope, but it's Copy so nothing special happens
```

### 4.2: References and Borrowing
```rust
// Immutable references
fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope, but it doesn't have ownership, so nothing is dropped

let s1 = String::from("hello");
let len = calculate_length(&s1);  // s1 still valid after call

// Mutable references
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

let mut s = String::from("hello");
change(&mut s);  // s is borrowed mutably

// Reference rules
let mut s = String::from("hello");
let r1 = &s;        // No problem
let r2 = &s;        // No problem (multiple immutable refs)
// let r3 = &mut s;    // BIG PROBLEM! Cannot mix mutable and immutable

// Dangling references prevented
// fn dangle() -> &String {  // ❌ Won't compile
//     let s = String::from("hello");
//     &s  // s goes out of scope
// }
```

### 4.3: The Slice Type
```rust
// String slices
let s = String::from("hello world");
let hello = &s[0..5];   // "hello"
let world = &s[6..11];  // "world"
let slice = &s[..];     // Entire string

// String literals are slices
let s = "Hello, world!";  // Type is &str

// Function with string slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// Array slices
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];  // Type is &[i32]
```

## Ownership Patterns

### Move Semantics
```rust
// Basic move
let s1 = String::from("hello");
let s2 = s1;  // Value moved to s2, s1 invalidated

// Function parameter moves
fn take_ownership(s: String) {
    println!("{}", s);
}  // s is dropped here

let s = String::from("hello");
take_ownership(s);  // s is moved, no longer valid
// println!("{}", s);  // ❌ Won't compile

// Return ownership
fn give_ownership() -> String {
    String::from("hello")  // Ownership moved to caller
}

fn take_and_give_back(a_string: String) -> String {
    a_string  // a_string is moved to caller
}
```

### Borrowing Patterns
```rust
// Multiple immutable borrows
let s = String::from("hello");
let r1 = &s;
let r2 = &s;
let r3 = &s;
println!("{}, {}, {}", r1, r2, r3);  // All valid

// Exclusive mutable borrow
let mut s = String::from("hello");
{
    let r1 = &mut s;
    println!("{}", r1);
}  // r1 goes out of scope
let r2 = &mut s;  // Now valid

// Non-lexical lifetimes
let mut s = String::from("hello world");
let word = first_word(&s);  // Immutable borrow
s.clear();  // ❌ Won't compile - word still active
println!("{}", word);
```

### Slice Patterns
```rust
// String processing with slices
fn process_text(text: &str) -> (&str, &str) {
    let mid = text.len() / 2;
    (&text[..mid], &text[mid..])
}

// Generic slice operations
fn sum_slice(slice: &[i32]) -> i32 {
    let mut total = 0;
    for &value in slice {
        total += value;
    }
    total
}

// Slice iteration
let numbers = [1, 2, 3, 4, 5];
for &num in &numbers[1..4] {  // Iterate slice
    println!("{}", num);
}
```

## Memory Model Understanding

### Stack vs Heap Allocation
```rust
// Stack allocated (known size)
let x = 5;                    // i32 on stack
let arr = [1, 2, 3];         // Array on stack
let tuple = (1, 2, 3);       // Tuple on stack

// Heap allocated (dynamic size)
let s = String::from("hello"); // String data on heap
let v = vec![1, 2, 3];        // Vector data on heap
let b = Box::new(5);          // Boxed value on heap
```

### Drop Trait and RAII
```rust
struct CustomDrop {
    data: String,
}

impl Drop for CustomDrop {
    fn drop(&mut self) {
        println!("Dropping CustomDrop with data `{}`", self.data);
    }
}

fn main() {
    let c = CustomDrop {
        data: String::from("some data"),
    };
    println!("Created CustomDrop");
}  // c goes out of scope and drop is called
```

## Common Ownership Patterns

### Passing Data to Functions
```rust
// Taking ownership
fn process_string(s: String) -> String {
    s.to_uppercase()
}

// Borrowing immutably
fn get_length(s: &String) -> usize {
    s.len()
}

// Borrowing mutably
fn append_exclamation(s: &mut String) {
    s.push('!');
}

// Best practice: use string slices when possible
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}
```

### Returning Data from Functions
```rust
// Return owned data
fn create_string() -> String {
    String::from("hello")
}

// Return borrowed data (with lifetime)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Multiple return values
fn analyze_string(s: &str) -> (usize, &str) {
    let len = s.len();
    let first_word = s.split_whitespace().next().unwrap_or("");
    (len, first_word)
}
```

## Integration with Type System

### Copy vs Move Types
```rust
// Copy types (stack allocated, cheap to copy)
let x = 5;
let y = x;  // x is still valid

// Move types (heap allocated or complex)
let s1 = String::from("hello");
let s2 = s1;  // s1 is no longer valid

// Clone when you need deep copy
let s1 = String::from("hello");
let s2 = s1.clone();  // Both valid, but expensive
```

### Trait Bounds and Ownership
```rust
fn duplicate<T: Clone>(x: T) -> (T, T) {
    let y = x.clone();
    (x, y)
}

fn print_twice<T: std::fmt::Display + Copy>(x: T) {
    println!("{}", x);
    println!("{}", x);  // x still valid because it's Copy
}
```

## Performance Implications

### Zero-Cost Abstractions
- Ownership rules enforced at compile time
- No runtime checks for memory safety
- Predictable performance characteristics
- No garbage collection pauses

### Memory Efficiency
- Automatic memory management without GC overhead
- Stack allocation preferred when possible
- Minimal heap allocations in well-designed code
- Deterministic memory usage patterns

## Common Mistakes and Solutions

### Fighting the Borrow Checker
```rust
// ❌ Common mistake
// let mut v = vec![1, 2, 3];
// for i in &v {
//     v.push(*i);  // Can't modify while iterating
// }

// ✅ Solution: collect what you need first
let mut v = vec![1, 2, 3];
let to_add: Vec<i32> = v.iter().cloned().collect();
for item in to_add {
    v.push(item);
}
```

### Use of Cloning
```rust
// ❌ Unnecessary cloning
fn process(s: String) -> String {
    s.to_uppercase()
}

let original = String::from("hello");
let result = process(original.clone());  // Expensive clone
println!("{}", original);

// ✅ Better: use references
fn process(s: &str) -> String {
    s.to_uppercase()
}

let original = String::from("hello");
let result = process(&original);  // Cheap borrow
println!("{}", original);
```

## Best Practices

1. **Prefer borrowing over moving** when you don't need ownership
2. **Use string slices (&str)** instead of String when possible
3. **Clone only when necessary** and understand the cost
4. **Design APIs to minimize ownership transfers**
5. **Use scoped blocks** to control lifetime and borrowing
6. **Understand Copy vs Clone** for different types

Official Chapter: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

---
*Completed: ✓*