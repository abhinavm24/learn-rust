# Chapter 10.1: Generic Data Types

## Key Takeaways

### Generic Fundamentals
- **Generics**: Allow code to work with multiple types without duplication
- **Type Parameters**: Placeholder types specified with `<T>`, `<U>`, etc.
- **Compile-Time Resolution**: Generics are resolved to specific types during compilation
- **Zero-Cost Abstraction**: No runtime performance penalty for using generics

### Generic Benefits
- **Code Reuse**: Write once, use with many types
- **Type Safety**: Compiler ensures type correctness
- **Performance**: As fast as hand-written type-specific code
- **Maintainability**: Single implementation reduces bugs and maintenance burden

### Monomorphization
- **Process**: Compiler creates specific versions for each concrete type used
- **Result**: Generic code becomes multiple specialized functions/structs
- **Performance**: Identical to writing separate functions for each type
- **Binary Size**: Trade-off between code reuse and binary size

### Important Syntax and Operators

#### Generic Function
```rust
fn function_name<T>(parameter: T) -> T {
    // function body
}
```

#### Generic Struct
```rust
struct StructName<T> {
    field: T,
}
```

#### Generic Enum
```rust
enum EnumName<T> {
    Variant(T),
}
```

#### Generic Method
```rust
impl<T> StructName<T> {
    fn method(&self) -> &T {
        &self.field
    }
}
```

### Programming Concepts Introduced
- **Type Parameterization**: Making data structures and functions work with any type
- **Generic Programming**: Programming paradigm focused on writing type-agnostic code
- **Type Inference**: Compiler automatically determines generic types when possible
- **Constraint-Based Generics**: Using trait bounds to restrict what types can be used

### Code Examples and Patterns

#### Generic Functions
```rust
// Function that works with any type that can be compared
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

#### Before and After: Removing Duplication
```rust
// Before: Duplicate code for different types
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// After: Single generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

#### Generic Structs
```rust
// Single type parameter
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &T {
        &self.y
    }
}

// Multiple type parameters
struct Point2D<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2D<T, U> {
    fn new(x: T, y: U) -> Point2D<T, U> {
        Point2D { x, y }
    }
    
    // Method that mixes generic types
    fn mixup<V, W>(self, other: Point2D<V, W>) -> Point2D<T, W> {
        Point2D {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // Same type for both coordinates
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);
    
    // Different types for coordinates
    let mixed_point = Point2D::new(5, 10.4);
    let string_point = Point2D::new("Hello", 'c');
    
    // Using mixup method
    let point1 = Point2D::new(5, 10.4);
    let point2 = Point2D::new("Hello", 'c');
    let point3 = point1.mixup(point2);
    
    println!("point3.x = {}, point3.y = {}", point3.x, point3.y);
}
```

#### Generic Enums
```rust
// Option enum (from standard library)
enum Option<T> {
    Some(T),
    None,
}

// Result enum (from standard library)
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Custom generic enum
enum Container<T> {
    Empty,
    Single(T),
    Multiple(Vec<T>),
}

impl<T> Container<T> {
    fn new() -> Container<T> {
        Container::Empty
    }
    
    fn add_single(item: T) -> Container<T> {
        Container::Single(item)
    }
    
    fn add_multiple(items: Vec<T>) -> Container<T> {
        Container::Multiple(items)
    }
    
    fn is_empty(&self) -> bool {
        matches!(self, Container::Empty)
    }
}

fn main() {
    let empty_container: Container<i32> = Container::new();
    let single_container = Container::add_single(42);
    let multiple_container = Container::add_multiple(vec![1, 2, 3, 4, 5]);
    
    println!("Empty: {}", empty_container.is_empty());
    println!("Single: {}", single_container.is_empty());
    println!("Multiple: {}", multiple_container.is_empty());
}
```

#### Generic Methods
```rust
struct Point<T> {
    x: T,
    y: T,
}

// Methods for all Point<T>
impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
    
    fn get_x(&self) -> &T {
        &self.x
    }
    
    fn get_y(&self) -> &T {
        &self.y
    }
}

// Methods only for Point<f32>
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Methods only for Point<i32>
impl Point<i32> {
    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn main() {
    let float_point = Point::new(3.0, 4.0);
    let int_point = Point::new(3, 4);
    
    // Available for all Point<T>
    println!("Float x: {}", float_point.get_x());
    println!("Int x: {}", int_point.get_x());
    
    // Only available for Point<f32>
    println!("Distance from origin: {}", float_point.distance_from_origin());
    
    // Only available for Point<i32>
    println!("Manhattan distance: {}", int_point.manhattan_distance());
}
```

#### Real-World Example: Generic Data Structures
```rust
// Generic stack implementation
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack {
            items: Vec::new(),
        }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    // Stack of integers
    let mut int_stack = Stack::new();
    int_stack.push(1);
    int_stack.push(2);
    int_stack.push(3);
    
    while let Some(value) = int_stack.pop() {
        println!("Popped: {}", value);
    }
    
    // Stack of strings
    let mut string_stack = Stack::new();
    string_stack.push("Hello".to_string());
    string_stack.push("World".to_string());
    
    if let Some(top) = string_stack.peek() {
        println!("Top item: {}", top);
    }
}
```

#### Generic Wrapper Types
```rust
// Generic wrapper that adds debugging information
#[derive(Debug)]
struct Debuggable<T> {
    value: T,
    debug_info: String,
}

impl<T> Debuggable<T> {
    fn new(value: T, debug_info: String) -> Debuggable<T> {
        Debuggable { value, debug_info }
    }
    
    fn value(&self) -> &T {
        &self.value
    }
    
    fn debug_info(&self) -> &str {
        &self.debug_info
    }
    
    fn unwrap(self) -> T {
        self.value
    }
}

// Generic pair type
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Pair<T, U> {
        Pair { first, second }
    }
    
    fn first(&self) -> &T {
        &self.first
    }
    
    fn second(&self) -> &U {
        &self.second
    }
    
    fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

fn main() {
    let debug_int = Debuggable::new(42, "Important number".to_string());
    println!("Value: {}, Info: {}", debug_int.value(), debug_int.debug_info());
    
    let pair = Pair::new("Hello", 42);
    println!("First: {}, Second: {}", pair.first(), pair.second());
    
    let swapped = pair.swap();
    println!("Swapped - First: {}, Second: {}", swapped.first(), swapped.second());
}
```

#### Generic Functions with Multiple Parameters
```rust
// Function with multiple generic type parameters
fn combine<T, U, V, F>(first: T, second: U, combiner: F) -> V
where
    F: Fn(T, U) -> V,
{
    combiner(first, second)
}

fn main() {
    // Combine two numbers
    let sum = combine(5, 10, |a, b| a + b);
    println!("Sum: {}", sum);
    
    // Combine string and number
    let message = combine("Count: ", 42, |s, n| format!("{}{}", s, n));
    println!("{}", message);
    
    // Combine two strings
    let greeting = combine("Hello", "World", |a, b| format!("{}, {}!", a, b));
    println!("{}", greeting);
}
```

#### Monomorphization Example
```rust
// This generic function...
fn print_value<T: std::fmt::Display>(value: T) {
    println!("Value: {}", value);
}

fn main() {
    print_value(42);      // i32
    print_value(3.14);    // f64
    print_value("hello"); // &str
}

// ...becomes these specific functions after monomorphization:
// fn print_value_i32(value: i32) {
//     println!("Value: {}", value);
// }
// 
// fn print_value_f64(value: f64) {
//     println!("Value: {}", value);
// }
// 
// fn print_value_str(value: &str) {
//     println!("Value: {}", value);
// }
```

#### Complex Generic Example: Cache
```rust
use std::collections::HashMap;
use std::hash::Hash;

struct Cache<K, V> 
where
    K: Eq + Hash,
{
    data: HashMap<K, V>,
    max_size: usize,
}

impl<K, V> Cache<K, V> 
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn new(max_size: usize) -> Cache<K, V> {
        Cache {
            data: HashMap::new(),
            max_size,
        }
    }
    
    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }
    
    fn insert(&mut self, key: K, value: V) {
        if self.data.len() >= self.max_size {
            // Simple eviction: clear all (real cache would be smarter)
            self.data.clear();
        }
        self.data.insert(key, value);
    }
    
    fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }
    
    fn len(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    let mut string_cache: Cache<String, i32> = Cache::new(3);
    
    string_cache.insert("one".to_string(), 1);
    string_cache.insert("two".to_string(), 2);
    string_cache.insert("three".to_string(), 3);
    
    if let Some(value) = string_cache.get(&"two".to_string()) {
        println!("Found value: {}", value);
    }
    
    println!("Cache size: {}", string_cache.len());
}
```

### Practical Applications
- Collections (Vec, HashMap, etc.) that work with any type
- Error handling with Result<T, E>
- Optional values with Option<T>
- Abstract data types (stacks, queues, trees)
- Mathematical operations that work with different numeric types
- Wrapper types that add functionality

### Generic Naming Conventions
- `T` - Type (most common single generic parameter)
- `E` - Error (used in Result<T, E>)
- `K` - Key (used in key-value structures)
- `V` - Value (used in key-value structures)
- `U`, `W`, `X`, `Y`, `Z` - Additional type parameters
- Use descriptive names for complex generics: `Item`, `Output`, `Input`

### Performance Characteristics
- **Zero Runtime Cost**: Generics are resolved at compile time
- **Code Bloat**: Each concrete type creates a separate copy
- **Compile Time**: More generic code can increase compilation time
- **Optimization**: Compiler can optimize each monomorphized version separately

### Integration with Previous Chapters
- Uses structs and enums as building blocks for generic types
- Applies to collections like Vec and HashMap
- Works with error handling patterns (Result, Option)
- Enables reusable code without sacrificing performance

### Community Conventions and Idioms
- Use single letters (T, U, V) for simple generic parameters
- Use descriptive names for complex or domain-specific generics
- Keep generic parameter lists reasonable in length
- Document constraints and expected behavior for generic types
- Prefer generic code over duplicated type-specific code

### Limitations and Considerations
- Cannot use generic parameters in const expressions (in most cases)
- Some operations require trait bounds to work
- Generic code can be harder to debug
- Binary size increases with number of concrete types used
- Compile times can increase with heavy generic usage

### Personal Notes
- Generics are essential for writing reusable, efficient code in Rust
- Understanding monomorphization helps explain Rust's zero-cost abstraction philosophy
- The combination of generics and traits (next chapter) is extremely powerful
- Generic programming takes practice to master but is very rewarding
- Rust's generics are more powerful than templates in many other languages

Official Chapter: https://doc.rust-lang.org/book/ch10-01-syntax.html

---
*Completed: ✓*