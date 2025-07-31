//! Chapter 19.2: Advanced Traits
//! 
//! This example demonstrates:
//! - Associated types vs generic types
//! - Default generic type parameters
//! - Supertraits and trait bounds
//! - Newtype pattern for external trait implementation
//! - Fully qualified syntax for disambiguation

use rust_book_examples::print_chapter_header;
use std::fmt;
use std::ops::Add;

fn main() {
    print_chapter_header("Chapter 19.2", "Advanced Traits");
    
    println!("=== Associated Types ===");
    associated_types_example();
    
    println!("\n=== Default Generic Type Parameters ===");
    default_generic_params_example();
    
    println!("\n=== Supertraits ===");
    supertraits_example();
    
    println!("\n=== Newtype Pattern ===");
    newtype_pattern_example();
    
    println!("\n=== Fully Qualified Syntax ===");
    fully_qualified_syntax_example();
    
    println!("\n=== Advanced Trait Patterns ===");
    advanced_trait_patterns();
}

// Associated Types Example
trait Iterator {
    type Item; // Associated type
    
    fn next(&mut self) -> Option<Self::Item>;
}

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
    type Item = usize; // Specify the associated type
    
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

// Comparison: Generic trait (allows multiple implementations)
trait GenericIterator<T> {
    fn next_generic(&mut self) -> Option<T>;
}

// We could implement this for the same type with different T
impl GenericIterator<String> for Counter {
    fn next_generic(&mut self) -> Option<String> {
        self.next().map(|n| format!("Item {}", n))
    }
}

impl GenericIterator<u32> for Counter {
    fn next_generic(&mut self) -> Option<u32> {
        self.next().map(|n| n as u32 * 10)
    }
}

fn associated_types_example() {
    println!("Associated types provide one implementation per type:");
    
    let mut counter = Counter::new(3);
    
    println!("Using associated type Iterator:");
    while let Some(item) = counter.next() {
        println!("  Item: {}", item);
    }
    
    // Reset counter for generic examples
    let mut counter = Counter::new(3);
    
    println!("Using generic trait with String:");
    while let Some(item) = GenericIterator::<String>::next_generic(&mut counter) {
        println!("  {}", item);
    }
    
    let mut counter = Counter::new(3);
    println!("Using generic trait with u32:");
    while let Some(item) = GenericIterator::<u32>::next_generic(&mut counter) {
        println!("  Value: {}", item);
    }
    
    // Function using associated types
    fn process_iterator<I>(mut iter: I) 
    where 
        I: Iterator,
        I::Item: fmt::Display,
    {
        println!("Processing iterator:");
        while let Some(item) = iter.next() {
            println!("  Processing: {}", item);
        }
    }
    
    let counter = Counter::new(3);
    process_iterator(counter);
}

// Default Generic Type Parameters
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Custom Add implementation with default RHS type parameter
impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Custom type that can be added to Point
#[derive(Debug, Clone, Copy)]
struct Offset {
    dx: i32,
    dy: i32,
}

impl Add<Offset> for Point {
    type Output = Point;
    
    fn add(self, offset: Offset) -> Point {
        Point {
            x: self.x + offset.dx,
            y: self.y + offset.dy,
        }
    }
}

// Custom Add trait showing default parameter
trait CustomAdd<Rhs = Self> {
    type Output;
    
    fn custom_add(self, rhs: Rhs) -> Self::Output;
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Millimeters(u32);

#[derive(Debug, PartialEq, Clone, Copy)]
struct Meters(u32);

impl CustomAdd for Millimeters {
    type Output = Millimeters;
    
    fn custom_add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

impl CustomAdd<Meters> for Millimeters {
    type Output = Millimeters;
    
    fn custom_add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn default_generic_params_example() {
    println!("Default generic type parameters:");
    
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2; // Uses default Add<Point> for Point
    
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    println!("p1 + p2 = {:?}", p3);
    
    let offset = Offset { dx: 5, dy: 10 };
    let p4 = p1 + offset; // Uses Add<Offset> for Point
    
    println!("offset: {:?}", offset);
    println!("p1 + offset = {:?}", p4);
    
    // Custom Add examples
    let mm1 = Millimeters(1000);
    let mm2 = Millimeters(500);
    let mm_result = mm1.custom_add(mm2); // Uses default Rhs=Self
    
    println!("{}mm + {}mm = {:?}", mm1.0, mm2.0, mm_result);
    
    let mm3 = Millimeters(500);
    let m1 = Meters(2);
    let mm_result2 = mm3.custom_add(m1); // Uses Rhs=Meters
    
    println!("{}mm + {}m = {:?}", mm3.0, m1.0, mm_result2);
}

// Supertraits
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// Animal hierarchy demonstrating supertraits
trait Animal {
    fn name(&self) -> &'static str;
}

trait Dog: Animal {
    fn bark(&self) {
        println!("{} says: Woof!", self.name());
    }
    
    fn wag_tail(&self) {
        println!("{} is wagging its tail!", self.name());
    }
}

struct GoldenRetriever {
    name: &'static str,
}

impl Animal for GoldenRetriever {
    fn name(&self) -> &'static str {
        self.name
    }
}

impl Dog for GoldenRetriever {}

fn supertraits_example() {
    println!("Supertraits require implementing prerequisite traits:");
    
    let person = Human;
    
    // Ambiguous method calls need disambiguation
    person.fly(); // Calls Human::fly
    Pilot::fly(&person); // Explicitly calls Pilot::fly  
    Wizard::fly(&person); // Explicitly calls Wizard::fly
    
    // OutlinePrint requires Display to be implemented
    let point = Point { x: 1, y: 3 };
    point.outline_print();
    
    // Animal hierarchy
    let dog = GoldenRetriever { name: "Buddy" };
    println!("Dog's name: {}", dog.name());
    dog.bark();
    dog.wag_tail();
    
    // More complex supertrait example
    trait Drawable {
        fn draw(&self);
    }
    
    trait Shape: Drawable {
        fn area(&self) -> f64;
        
        fn draw_with_area(&self) {
            self.draw();
            println!("Area: {:.2}", self.area());
        }
    }
    
    struct Circle {
        radius: f64,
    }
    
    impl Drawable for Circle {
        fn draw(&self) {
            println!("Drawing a circle with radius {}", self.radius);
        }
    }
    
    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }
    
    let circle = Circle { radius: 5.0 };
    circle.draw_with_area();
}

// Newtype Pattern
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// Can't implement external trait on external type directly
// This would be an error:
// impl fmt::Display for Vec<String> { ... }

// But we can use newtype pattern:
struct MyVec<T>(Vec<T>);

impl<T: fmt::Display> fmt::Display for MyVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyVec[")?;
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

// Newtype for type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UserId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProductId(u32);

// These are now different types even though both wrap u32
impl UserId {
    fn new(id: u32) -> Self {
        UserId(id)
    }
    
    fn value(&self) -> u32 {
        self.0
    }
}

impl ProductId {
    fn new(id: u32) -> Self {
        ProductId(id)
    }
    
    fn value(&self) -> u32 {
        self.0
    }
}

fn newtype_pattern_example() {
    println!("Newtype pattern for external trait implementation:");
    
    let w = Wrapper(vec![
        String::from("hello"),
        String::from("world"),
    ]);
    println!("Wrapper: {}", w);
    
    let my_vec = MyVec(vec![1, 2, 3, 4, 5]);
    println!("MyVec: {}", my_vec);
    
    // Type safety with newtypes
    let user_id = UserId::new(123);
    let product_id = ProductId::new(456);
    
    println!("User ID: {:?}", user_id);
    println!("Product ID: {:?}", product_id);
    
    // This would be a compile error:
    // let same = user_id == product_id; // Can't compare different newtype wrappers
    
    // But these work:
    let same_user = user_id == UserId::new(123);
    println!("Same user: {}", same_user);
    
    // Function that only accepts UserId, not raw u32
    fn get_user_profile(user_id: UserId) -> String {
        format!("Profile for user {}", user_id.value())
    }
    
    println!("{}", get_user_profile(user_id));
    // println!("{}", get_user_profile(123)); // This would be a compile error
}

// Fully Qualified Syntax
trait Greet {
    fn greet(&self);
}

trait Farewell {
    fn greet(&self); // Same method name as Greet
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

impl Farewell for Person {
    fn greet(&self) {
        println!("Goodbye from {}", self.name);
    }
}

impl Person {
    fn greet(&self) {
        println!("Hey there, I'm {}", self.name);
    }
}

// Associated functions (no self parameter)
trait AnimalNames {
    fn baby_name() -> String;
}

struct DogStruct;

impl DogStruct {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

impl AnimalNames for DogStruct {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

fn fully_qualified_syntax_example() {
    println!("Fully qualified syntax for disambiguation:");
    
    let person = Person {
        name: String::from("Alice"),
    };
    
    // Without qualification - calls inherent method
    person.greet();
    
    // With trait qualification
    Greet::greet(&person);
    Farewell::greet(&person);
    
    // Fully qualified syntax (most explicit)
    <Person as Greet>::greet(&person);
    <Person as Farewell>::greet(&person);
    
    // Associated functions require fully qualified syntax
    println!("A baby dog is called a {}", DogStruct::baby_name()); // Inherent associated function
    println!("A baby dog is called a {}", <DogStruct as AnimalNames>::baby_name()); // Trait associated function
    
    // More examples with associated functions
    trait Summary {
        fn summarize_author(&self) -> String;
        
        fn summarize() -> String {
            String::from("(Read more...)")
        }
    }
    
    struct NewsArticle {
        headline: String,
        author: String,
        content: String,
    }
    
    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
        
        fn summarize() -> String {
            String::from("Breaking news summary")
        }
    }
    
    let article = NewsArticle {
        headline: String::from("Rust 2.0 Released!"),
        author: String::from("rustacean"),
        content: String::from("The Rust team is excited to announce..."),
    };
    
    println!("Author: {}", article.summarize_author());
    println!("Default summary: {}", <NewsArticle as Summary>::summarize());
}

fn advanced_trait_patterns() {
    println!("Advanced trait patterns:");
    
    // Trait with multiple associated types
    trait Collect<T> {
        type Output;
        type Error;
        
        fn collect(items: Vec<T>) -> Result<Self::Output, Self::Error>;
    }
    
    struct StringCollector;
    
    impl Collect<&str> for StringCollector {
        type Output = String;
        type Error = String;
        
        fn collect(items: Vec<&str>) -> Result<Self::Output, Self::Error> {
            if items.is_empty() {
                Err("Cannot collect empty vector".to_string())
            } else {
                Ok(items.join(" "))
            }
        }
    }
    
    match StringCollector::collect(vec!["hello", "world"]) {
        Ok(result) => println!("Collected: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Trait with const generics and associated types
    trait FixedArray<T, const N: usize> {
        type Iterator: std::iter::Iterator<Item = T>;
        
        fn iter(&self) -> Self::Iterator;
        fn len(&self) -> usize {
            N
        }
    }
    
    #[derive(Debug)]
    struct MyArray<T, const N: usize> {
        data: [T; N],
    }
    
    impl<T, const N: usize> MyArray<T, N> {
        fn new(data: [T; N]) -> Self {
            MyArray { data }
        }
    }
    
    impl<T: Clone, const N: usize> FixedArray<T, N> for MyArray<T, N> {
        type Iterator = std::vec::IntoIter<T>;
        
        fn iter(&self) -> Self::Iterator {
            self.data.to_vec().into_iter()
        }
    }
    
    let arr = MyArray::new([1, 2, 3, 4, 5]);
    println!("Array length: {}", arr.len());
    println!("Array contents:");
    for item in arr.iter() {
        println!("  {}", item);
    }
    
    // Conditional trait implementations
    struct Pair<T> {
        x: T,
        y: T,
    }
    
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }
    
    impl<T: fmt::Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
    
    let pair = Pair::new(10, 20);
    pair.cmp_display();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_associated_types() {
        let mut counter = Counter::new(3);
        assert_eq!(counter.next(), Some(0));
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn test_default_generic_params() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };
        let result = p1 + p2;
        assert_eq!(result, Point { x: 4, y: 6 });
        
        let offset = Offset { dx: 5, dy: 10 };
        let result2 = p1 + offset;
        assert_eq!(result2, Point { x: 6, y: 12 });
    }

    #[test]
    fn test_newtype_pattern() {
        let user_id = UserId::new(123);
        let same_id = UserId::new(123);
        let different_id = UserId::new(456);
        
        assert_eq!(user_id, same_id);
        assert_ne!(user_id, different_id);
        assert_eq!(user_id.value(), 123);
    }

    #[test]
    fn test_trait_disambiguation() {
        let person = Person {
            name: String::from("Test"),
        };
        
        // These should not panic - just testing they compile and run
        person.greet();
        Greet::greet(&person);
        Farewell::greet(&person);
        <Person as Greet>::greet(&person);
        <Person as Farewell>::greet(&person);
    }

    #[test]
    fn test_custom_add() {
        let mm1 = Millimeters(1000);
        let mm2 = Millimeters(500);
        let result = mm1.custom_add(mm2);
        assert_eq!(result, Millimeters(1500));
        
        let mm3 = Millimeters(500);
        let m1 = Meters(2);
        let result2 = mm3.custom_add(m1);
        assert_eq!(result2, Millimeters(2500));
    }
}