//! # Chapter 10.2: Traits: Defining Shared Behavior
//! 
//! This example demonstrates:
//! - Defining traits to specify shared behavior
//! - Implementing traits on types
//! - Default trait implementations
//! - Traits as parameters and return types
//! - Trait bounds and where clauses
//! - Associated types vs generic type parameters

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 10.2", "Traits: Defining Shared Behavior");

    println!("=== Basic Trait Definition and Implementation ===");
    basic_trait_examples();
    
    println!("\n=== Default Trait Implementations ===");
    default_implementation_examples();
    
    println!("\n=== Traits as Parameters ===");
    trait_parameter_examples();
    
    println!("\n=== Returning Types that Implement Traits ===");
    return_trait_examples();
    
    println!("\n=== Trait Bounds ===");
    trait_bound_examples();
    
    println!("\n=== Associated Types ===");
    associated_type_examples();
    
    println!("\n=== Standard Library Traits ===");
    standard_library_traits();
}

fn basic_trait_examples() {
    // Define a trait
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    
    // Implement the trait for a struct
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
    }
    
    // Using the implemented traits
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("1 new tweet: {}", tweet.summarize());
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    
    println!("New article available! {}", article.summarize());
}

fn default_implementation_examples() {
    // Trait with default implementation
    pub trait Display {
        fn show(&self) -> String {
            String::from("(Default display)")
        }
        
        // Method that calls other method
        fn show_with_prefix(&self) -> String {
            format!("Display: {}", self.show())
        }
    }
    
    // Struct that uses default implementation
    pub struct DefaultUser {
        pub name: String,
    }
    
    impl Display for DefaultUser {}
    
    // Struct that overrides default implementation
    pub struct CustomUser {
        pub name: String,
        pub age: u32,
    }
    
    impl Display for CustomUser {
        fn show(&self) -> String {
            format!("{} (age {})", self.name, self.age)
        }
    }
    
    let default_user = DefaultUser {
        name: String::from("Alice"),
    };
    
    let custom_user = CustomUser {
        name: String::from("Bob"),
        age: 30,
    };
    
    println!("Default user: {}", default_user.show());
    println!("Default user with prefix: {}", default_user.show_with_prefix());
    println!("Custom user: {}", custom_user.show());
    println!("Custom user with prefix: {}", custom_user.show_with_prefix());
    
    // Trait with some default implementations
    pub trait Drawable {
        fn draw(&self);
        
        fn outline(&self) {
            println!("Drawing outline...");
            self.draw();
        }
    }
    
    pub struct Circle {
        radius: f64,
    }
    
    impl Drawable for Circle {
        fn draw(&self) {
            println!("Drawing a circle with radius {}", self.radius);
        }
    }
    
    let circle = Circle { radius: 5.0 };
    circle.draw();
    circle.outline();
}

fn trait_parameter_examples() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    
    pub struct Article {
        content: String,
    }
    
    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("Article: {}", self.content)
        }
    }
    
    // Trait as parameter (impl Trait syntax)
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    
    // Trait bound syntax (equivalent to above)
    pub fn notify_verbose<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    
    // Multiple trait bounds
    pub fn notify_and_display<T: Summary + std::fmt::Display>(item: &T) {
        println!("Breaking news! {}", item.summarize());
        println!("Display: {}", item);
    }
    
    // Where clause for complex bounds
    pub fn some_function<T, U>(t: &T, u: &U) -> String
    where
        T: std::fmt::Display + Clone,
        U: Clone + std::fmt::Debug,
    {
        format!("t: {}, u: {:?}", t, u.clone())
    }
    
    let article = Article {
        content: String::from("Important news content"),
    };
    
    notify(&article);
    notify_verbose(&article);
    
    // Using where clause function
    let result = some_function(&"hello", &vec![1, 2, 3]);
    println!("Function result: {}", result);
}

fn return_trait_examples() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    
    pub struct NewsArticle {
        pub content: String,
    }
    
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("News: {}", self.content)
        }
    }
    
    pub struct Tweet {
        pub content: String,
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("Tweet: {}", self.content)
        }
    }
    
    // Returning trait objects (dynamic dispatch)
    fn get_summarizable(use_tweet: bool) -> Box<dyn Summary> {
        if use_tweet {
            Box::new(Tweet {
                content: String::from("This is a tweet"),
            })
        } else {
            Box::new(NewsArticle {
                content: String::from("This is news"),
            })
        }
    }
    
    // Using impl Trait in return position (static dispatch)
    fn returns_summarizable() -> impl Summary {
        Tweet {
            content: String::from("This is a tweet with impl Trait"),
        }
    }
    
    let summarizable1 = get_summarizable(true);
    println!("Dynamic dispatch: {}", summarizable1.summarize());
    
    let summarizable2 = get_summarizable(false);
    println!("Dynamic dispatch: {}", summarizable2.summarize());
    
    let summarizable3 = returns_summarizable();
    println!("Static dispatch: {}", summarizable3.summarize());
}

fn trait_bound_examples() {
    // Using trait bounds to conditionally implement methods
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
    
    // This implementation is only available when T implements Display + PartialOrd
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
    
    let pair = Pair::new(5, 10);
    pair.cmp_display();
    
    let string_pair = Pair::new("hello", "world");
    string_pair.cmp_display();
    
    // Blanket implementations - implementing a trait for any type that implements another trait
    trait MyDisplay {
        fn my_display(&self) -> String;
    }
    
    // Implement MyDisplay for any type that implements Display
    impl<T: Display> MyDisplay for T {
        fn my_display(&self) -> String {
            format!("MyDisplay: {}", self)
        }
    }
    
    let number = 42;
    println!("{}", number.my_display());
    
    let text = "hello";
    println!("{}", text.my_display());
}

fn associated_type_examples() {
    // Associated types allow us to define placeholder types within trait definitions
    pub trait Iterator {
        type Item; // Associated type
        
        fn next(&mut self) -> Option<Self::Item>;
    }
    
    // Implementation with concrete associated type
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
        type Item = usize; // Concrete associated type
        
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
    
    let mut counter = Counter::new(3);
    while let Some(value) = counter.next() {
        println!("Counter value: {}", value);
    }
    
    // Another example: Associated types vs generics
    trait Add<Rhs = Self> {
        type Output;
        
        fn add(self, rhs: Rhs) -> Self::Output;
    }
    
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl Add for Point {
        type Output = Point;
        
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1.add(p2);
    println!("Point addition: {:?}", p3);
}

fn standard_library_traits() {
    println!("=== Common Standard Library Traits ===");
    
    // Debug trait
    #[derive(Debug)]
    struct DebugExample {
        name: String,
        value: i32,
    }
    
    let debug_item = DebugExample {
        name: String::from("test"),
        value: 42,
    };
    println!("Debug output: {:?}", debug_item);
    
    // Clone trait
    #[derive(Clone, Debug)]
    struct CloneExample {
        data: Vec<i32>,
    }
    
    let original = CloneExample {
        data: vec![1, 2, 3],
    };
    let cloned = original.clone();
    println!("Original: {:?}", original);
    println!("Cloned: {:?}", cloned);
    
    // PartialEq and Eq traits
    #[derive(Debug, PartialEq, Eq)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };
    let person2 = Person {
        name: String::from("Alice"),
        age: 30,
    };
    let person3 = Person {
        name: String::from("Bob"),
        age: 25,
    };
    
    println!("person1 == person2: {}", person1 == person2);
    println!("person1 == person3: {}", person1 == person3);
    
    // PartialOrd and Ord traits
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Student {
        grade: u32,
        name: String,
    }
    
    let mut students = vec![
        Student { grade: 85, name: String::from("Alice") },
        Student { grade: 90, name: String::from("Bob") },
        Student { grade: 78, name: String::from("Charlie") },
    ];
    
    students.sort();
    println!("Sorted students: {:?}", students);
    
    // From and Into traits
    struct Number {
        value: i32,
    }
    
    impl From<i32> for Number {
        fn from(value: i32) -> Self {
            Number { value }
        }
    }
    
    // Into is automatically implemented when From is implemented
    let num1: Number = Number::from(42);
    let num2: Number = 24.into();
    
    println!("Number from 42: {}", num1.value);
    println!("Number from 24: {}", num2.value);
    
    // Display trait
    use std::fmt;
    
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    
    let point = Point { x: 3, y: 4 };
    println!("Point display: {}", point);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_implementation() {
        trait Testable {
            fn test_value(&self) -> i32;
        }
        
        struct TestStruct {
            value: i32,
        }
        
        impl Testable for TestStruct {
            fn test_value(&self) -> i32 {
                self.value * 2
            }
        }
        
        let test = TestStruct { value: 21 };
        assert_eq!(test.test_value(), 42);
    }

    #[test]
    fn test_default_implementation() {
        trait DefaultTest {
            fn default_method(&self) -> String {
                String::from("default")
            }
        }
        
        struct TestStruct;
        impl DefaultTest for TestStruct {}
        
        let test = TestStruct;
        assert_eq!(test.default_method(), "default");
    }

    #[test]
    fn test_trait_bounds() {
        fn add_and_display<T: std::ops::Add<Output = T> + std::fmt::Display + Copy>(a: T, b: T) -> String {
            let result = a + b;
            format!("{}", result)
        }
        
        assert_eq!(add_and_display(2, 3), "5");
        assert_eq!(add_and_display(1.5, 2.5), "4");
    }

    #[test]
    fn test_associated_types() {
        trait Container {
            type Item;
            fn get(&self) -> &Self::Item;
        }
        
        struct IntContainer {
            value: i32,
        }
        
        impl Container for IntContainer {
            type Item = i32;
            
            fn get(&self) -> &Self::Item {
                &self.value
            }
        }
        
        let container = IntContainer { value: 42 };
        assert_eq!(*container.get(), 42);
    }
}