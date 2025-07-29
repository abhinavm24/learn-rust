# Chapter 10: Generic Types, Traits, and Lifetimes

## Key Takeaways

### Generic Programming
- **Code Reuse**: Write functions and types that work with multiple types
- **Zero-Cost Abstraction**: Generics have no runtime performance penalty
- **Monomorphization**: Compiler generates specific code for each concrete type used
- **Type Safety**: Generic constraints ensure type safety at compile time

### Traits System
- **Shared Behavior**: Define common functionality across different types
- **Interface Definition**: Traits define method signatures that types must implement
- **Multiple Implementation**: Types can implement multiple traits
- **Default Methods**: Traits can provide default implementations

### Lifetime Management
- **Memory Safety**: Lifetimes prevent dangling references
- **Borrow Checker**: Compiler ensures references are valid for their lifetime
- **Lifetime Annotations**: Explicit specification of reference relationships
- **Automatic Inference**: Most lifetimes are inferred by the compiler

### Advanced Type System
- **Trait Bounds**: Constrain generic types to specific capabilities
- **Associated Types**: Type aliases within traits for cleaner APIs
- **Trait Objects**: Dynamic dispatch for runtime polymorphism
- **Higher-Ranked Trait Bounds**: Complex lifetime relationships

## Chapter Structure

### 10.1: Generic Data Types
```rust
// Generic functions
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

// Generic structs
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
}

// Multiple generic parameters
struct Point3D<T, U, V> {
    x: T,
    y: U,
    z: V,
}

// Specific implementations
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Generic enums
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Generic methods
impl<T, U> Point<T> {
    fn mixup<V, W>(self, other: Point<V>) -> Point<T, W>
    where
        Point<V>: Into<Point<V, W>>,  // Imaginary constraint
    {
        // Implementation would go here
        todo!()
    }
}
```

### 10.2: Traits: Defining Shared Behavior
```rust
// Basic trait definition
pub trait Summary {
    fn summarize(&self) -> String;
}

// Trait with default implementation
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
    
    fn summarize_author(&self) -> String;
    
    // Default implementation calling other methods
    fn full_summary(&self) -> String {
        format!("From {}: {}", self.summarize_author(), self.summarize())
    }
}

// Implementing traits
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Traits as parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
pub fn notify(item: &(impl Summary + Display)) {
    // ...
}

pub fn notify<T: Summary + Display>(item: &T) {
    // ...
}

// Where clauses for complex bounds
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}

// Returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// Conditional trait implementations
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Blanket implementations
impl<T: Display> ToString for T {
    // Any type that implements Display gets ToString for free
}
```

### 10.3: Validating References with Lifetimes
```rust
// Basic lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// Lifetime annotations in struct definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Lifetime elision rules
// Rule 1: Each parameter gets its own lifetime
fn first_word(s: &str) -> &str {  // Becomes fn first_word<'a>(s: &'a str) -> &'a str
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// Rule 2: If exactly one input lifetime, it's assigned to all outputs
fn get_first<T>(slice: &[T]) -> &T {  // Becomes fn get_first<'a, T>(slice: &'a [T]) -> &'a T
    &slice[0]
}

// Rule 3: If &self or &mut self, its lifetime is assigned to all outputs
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        // Return type gets same lifetime as &self
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Static lifetime
let s: &'static str = "I have a static lifetime.";

// Generic type parameters, trait bounds, and lifetimes together
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## Advanced Generic Patterns

### Associated Types
```rust
// Iterator trait with associated type
pub trait Iterator {
    type Item;  // Associated type
    
    fn next(&mut self) -> Option<Self::Item>;
    
    // Default implementations using associated type
    fn collect<B: FromIterator<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        FromIterator::from_iter(self)
    }
}

// Implementation
struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = usize;  // Specify the associated type
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

// Associated types vs generics
trait Graph<N, E> {
    // Multiple implementations possible for same type with different N, E
    fn has_edge(&self, node1: &N, node2: &N) -> bool;
}

trait Graph {
    type Node;     // Only one implementation possible per type
    type Edge;
    
    fn has_edge(&self, node1: &Self::Node, node2: &Self::Node) -> bool;
}
```

### Trait Objects and Dynamic Dispatch
```rust
// Trait object syntax
trait Draw {
    fn draw(&self);
}

struct Screen {
    pub components: Vec<Box<dyn Draw>>,  // Trait object
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();  // Dynamic dispatch
        }
    }
}

// Implementing the trait
struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button: {}", self.label);
    }
}

struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing select box with {} options", self.options.len());
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    
    screen.run();
}

// Object safety requirements
trait Clone {
    fn clone(&self) -> Self;  // Not object safe - returns Self
}

// Box<dyn Clone> is not allowed

trait Draw {
    fn draw(&self);           // Object safe - no Self in return type
    
    fn name(&self) -> &str {  // Object safe - no Self
        "Component"
    }
}

// Box<dyn Draw> is allowed
```

### Advanced Lifetime Patterns
```rust
// Multiple lifetime parameters
fn compare_and_display<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    'b: 'a,  // 'b must live at least as long as 'a
{
    println!("Comparing {} and {}", x, y);
    x
}

// Lifetime bounds in structs
struct Ref<'a, T: 'a> {
    value: &'a T,
}

// Or equivalently in modern Rust:
struct Ref<'a, T> {
    value: &'a T,
}

// Higher-ranked trait bounds (HRTB)
fn higher_ranked_fn<F>(f: F)
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let s = "hello";
    let result = f(s);
    println!("Result: {}", result);
}

// Lifetime subtyping
fn choose_first<'a: 'b, 'b>(first: &'a str, _second: &'b str) -> &'b str {
    first  // 'a must outlive 'b for this to work
}

// Phantom lifetimes
use std::marker::PhantomData;

struct Slice<'a, T> {
    ptr: *const T,
    len: usize,
    _marker: PhantomData<&'a T>,  // Phantom lifetime parameter
}

impl<'a, T> Slice<'a, T> {
    fn new(slice: &'a [T]) -> Self {
        Slice {
            ptr: slice.as_ptr(),
            len: slice.len(),
            _marker: PhantomData,
        }
    }
    
    fn as_slice(&self) -> &'a [T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr, self.len)
        }
    }
}
```

### Generic Collections and Algorithms
```rust
// Generic container
struct Container<T> {
    items: Vec<T>,
}

impl<T> Container<T> {
    fn new() -> Self {
        Container { items: Vec::new() }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// Generic algorithms with trait bounds
fn find_max<T: PartialOrd + Copy>(slice: &[T]) -> Option<T> {
    if slice.is_empty() {
        return None;
    }
    
    let mut max = slice[0];
    for &item in slice.iter().skip(1) {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

fn binary_search<T: PartialOrd>(slice: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = slice.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        
        if &slice[mid] == target {
            return Some(mid);
        } else if &slice[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    
    None
}

// Generic with multiple trait bounds
fn sort_and_print<T>(mut vec: Vec<T>) -> Vec<T>
where
    T: Ord + std::fmt::Debug,
{
    vec.sort();
    println!("Sorted: {:?}", vec);
    vec
}

// Generic closure parameter
fn apply_to_all<T, F>(vec: Vec<T>, f: F) -> Vec<T>
where
    F: Fn(T) -> T,
{
    vec.into_iter().map(f).collect()
}

fn main() {
    let numbers = vec![64, 34, 25, 12, 22, 11, 90];
    let max = find_max(&numbers).unwrap();
    println!("Max: {}", max);
    
    let index = binary_search(&numbers, &25);
    println!("Index of 25: {:?}", index);
    
    let sorted = sort_and_print(numbers.clone());
    
    let doubled = apply_to_all(vec![1, 2, 3, 4], |x| x * 2);
    println!("Doubled: {:?}", doubled);
}
```

### Trait Composition and Supertraits
```rust
// Supertrait requirements
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

trait OutlinePrint: std::fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();  // Can use Display methods
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Multiple trait bounds
fn print_and_compare<T>(item: &T)
where
    T: std::fmt::Display + PartialOrd,
{
    println!("Item: {}", item);
}

// Trait aliases (RFC 1733 - not yet stable)
// trait PrintableAndComparable = std::fmt::Display + PartialOrd;

// Workaround with marker traits
trait DisplayAndOrd: std::fmt::Display + Ord {}

// Automatic implementation for any type that satisfies both bounds
impl<T: std::fmt::Display + Ord> DisplayAndOrd for T {}

fn use_display_and_ord<T: DisplayAndOrd>(item: &T) {
    println!("Displayable and orderable: {}", item);
}
```

### Generic Const Parameters (Rust 1.51+)
```rust
// Generic const parameters
struct Array<T, const N: usize> {
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> Array<T, N> {
    fn new() -> Self {
        Array {
            data: [T::default(); N],
        }
    }
    
    fn len(&self) -> usize {
        N
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
}

// Generic const functions
const fn max_array_size<const N: usize, const M: usize>() -> usize {
    if N > M { N } else { M }
}

fn main() {
    let arr1: Array<i32, 5> = Array::new();
    let arr2: Array<f64, 10> = Array::new();
    
    println!("Array 1 length: {}", arr1.len());
    println!("Array 2 length: {}", arr2.len());
    
    const MAX_SIZE: usize = max_array_size::<100, 200>();
    println!("Max size: {}", MAX_SIZE);
}
```

## Performance Considerations

### Monomorphization
```rust
// This generic function
fn generic_function<T: std::fmt::Display>(value: T) {
    println!("Value: {}", value);
}

// Gets compiled to specific functions:
// fn generic_function_i32(value: i32) { println!("Value: {}", value); }
// fn generic_function_String(value: String) { println!("Value: {}", value); }

fn main() {
    generic_function(42);         // Uses i32 version
    generic_function("hello");    // Uses &str version
}

// Static vs dynamic dispatch
trait Animal {
    fn make_sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) { println!("Woof!"); }
}

impl Animal for Cat {
    fn make_sound(&self) { println!("Meow!"); }
}

// Static dispatch - zero cost
fn static_dispatch<T: Animal>(animal: &T) {
    animal.make_sound();  // Compiled to specific implementation
}

// Dynamic dispatch - runtime cost
fn dynamic_dispatch(animal: &dyn Animal) {
    animal.make_sound();  // Virtual function call through vtable
}

fn main() {
    let dog = Dog;
    let cat = Cat;
    
    // Static dispatch - no runtime overhead
    static_dispatch(&dog);
    static_dispatch(&cat);
    
    // Dynamic dispatch - runtime polymorphism
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];
    
    for animal in &animals {
        dynamic_dispatch(animal.as_ref());
    }
}
```

### Generic Specialization (unstable)
```rust
// Not yet stable in Rust, but shows the concept
#![feature(specialization)]

trait Example {
    fn method(&self) -> String;
}

// Default implementation
impl<T> Example for T {
    default fn method(&self) -> String {
        "default implementation".to_string()
    }
}

// Specialized implementation for String
impl Example for String {
    fn method(&self) -> String {
        format!("specialized for String: {}", self)
    }
}
```

## Testing Generic Code
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generic_container() {
        let mut container = Container::new();
        assert!(container.is_empty());
        
        container.push(1);
        container.push(2);
        container.push(3);
        
        assert_eq!(container.len(), 3);
        assert_eq!(container.pop(), Some(3));
        assert_eq!(container.len(), 2);
    }
    
    #[test]
    fn test_different_types() {
        let mut int_container = Container::new();
        int_container.push(42);
        
        let mut string_container = Container::new();
        string_container.push("hello".to_string());
        
        assert_eq!(int_container.pop(), Some(42));
        assert_eq!(string_container.pop(), Some("hello".to_string()));
    }
    
    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&[1, 5, 3, 2, 4]), Some(5));
        assert_eq!(find_max(&["a", "z", "m"]), Some("z"));
        assert_eq!(find_max::<i32>(&[]), None);
    }
    
    #[test]
    fn test_trait_objects() {
        let button = Button {
            width: 50,
            height: 10,
            label: "Click me!".to_string(),
        };
        
        let select_box = SelectBox {
            width: 75,
            height: 10,
            options: vec!["Option 1".to_string(), "Option 2".to_string()],
        };
        
        let components: Vec<Box<dyn Draw>> = vec![
            Box::new(button),
            Box::new(select_box),
        ];
        
        // Test that we can call draw on each component
        for component in &components {
            component.draw();  // Should not panic
        }
    }
}
```

## Best Practices

### Generic Design
```rust
// ✅ Good: Use meaningful names
struct Database<Connection, Query, Result> {
    connection: Connection,
    _phantom: std::marker::PhantomData<(Query, Result)>,
}

// ❌ Bad: Unclear names
struct Database<T, U, V> {
    connection: T,
    _phantom: std::marker::PhantomData<(U, V)>,
}

// ✅ Good: Appropriate trait bounds
fn process_items<T: Iterator<Item = String>>(items: T) -> Vec<String> {
    items.filter(|s| !s.is_empty()).collect()
}

// ❌ Bad: Too generic
fn process_items<T>(items: T) -> Vec<String> {
    // Can't do anything useful with T
}

// ✅ Good: Use associated types when there's one logical type
trait Iterator {
    type Item;  // One item type per iterator
}

// ❌ Bad: Generics when associated types are better
trait Iterator<Item> {  // Multiple possible item types per iterator
}
```

### Lifetime Best Practices
```rust
// ✅ Good: Let the compiler infer when possible
fn first_word(s: &str) -> &str {  // Lifetime elision works
    s.split_whitespace().next().unwrap_or("")
}

// ❌ Bad: Unnecessary explicit lifetimes
fn first_word<'a>(s: &'a str) -> &'a str {  // Unnecessary
    s.split_whitespace().next().unwrap_or("")
}

// ✅ Good: Use 'static judiciously
const GREETING: &'static str = "Hello";  // Lives for entire program

// ❌ Bad: Forcing 'static when not needed
fn process_string(s: &'static str) -> String {  // Too restrictive
    s.to_uppercase()
}

// ✅ Better: Accept any lifetime
fn process_string(s: &str) -> String {
    s.to_uppercase()
}
```

### Error Handling with Generics
```rust
// Generic error handling
fn parse_and_process<T, E>(input: &str) -> Result<T, E>
where
    T: std::str::FromStr<Err = E>,
    E: std::fmt::Display,
{
    match input.parse() {
        Ok(value) => Ok(value),
        Err(e) => {
            eprintln!("Parse error: {}", e);
            Err(e)
        }
    }
}

// Usage with different types
fn main() {
    let number: Result<i32, _> = parse_and_process("42");
    let float: Result<f64, _> = parse_and_process("3.14");
    
    println!("Number: {:?}", number);
    println!("Float: {:?}", float);
}
```

Official Chapter: https://doc.rust-lang.org/book/ch10-00-generics.html

---
*Completed: ✓*