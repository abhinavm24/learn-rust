//! # Chapter 10.3: Validating References with Lifetimes
//! 
//! This example demonstrates:
//! - What lifetimes are and why they're needed
//! - Lifetime annotation syntax
//! - Generic lifetime parameters in function signatures
//! - Lifetime annotations in struct definitions
//! - Lifetime elision rules
//! - The static lifetime
//! - Generic type parameters, trait bounds, and lifetimes together

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 10.3", "Validating References with Lifetimes");

    println!("=== Understanding Lifetimes ===");
    lifetime_basics();
    
    println!("\n=== Generic Lifetimes in Functions ===");
    function_lifetime_examples();
    
    println!("\n=== Lifetime Annotations in Structs ===");
    struct_lifetime_examples();
    
    println!("\n=== Lifetime Elision Rules ===");
    lifetime_elision_examples();
    
    println!("\n=== The Static Lifetime ===");
    static_lifetime_examples();
    
    println!("\n=== Combining Generics, Traits, and Lifetimes ===");
    combined_examples();
}

fn lifetime_basics() {
    println!("Lifetimes ensure that references remain valid for as long as needed.");
    println!("Every reference has a lifetime, which is the scope for which it's valid.");
    println!();
    
    // This example shows the concept of lifetimes
    let r: Option<&i32> = None;  // Declare 'r' with type annotation
    {
        let x = 5;        // 'x' comes into scope
        // r = &x;        // This would fail: 'x' doesn't live long enough
        println!("x = {}", x);
    }                     // 'x' goes out of scope
    // println!("r: {}", r); // This would fail: 'r' would be a dangling reference
    
    // This works because both variables have compatible lifetimes
    let x = 5;            // 'x' has a long lifetime
    let r = &x;           // 'r' borrows 'x' for a shorter or equal lifetime
    println!("r: {}", r); // This is valid
    
    println!("The borrow checker ensures references don't outlive their data.");
}

fn function_lifetime_examples() {
    // Function that needs lifetime annotations
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
    
    // Example showing how lifetimes work with different scopes
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
        // 'result' is valid here because both inputs are valid
    }
    
    // Function that doesn't need lifetime annotation (returns one input)
    fn first_word<'a>(s: &'a str) -> &'a str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
    
    let sentence = "hello world";
    let word = first_word(sentence);
    println!("First word: '{}'", word);
    
    // Function that always returns a reference to the first parameter
    fn longest_with_announcement<'a>(
        x: &'a str,
        y: &'a str,  // Both parameters need the same lifetime
        ann: &str,
    ) -> &'a str {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y  // This would cause a compile error if we tried to return y
        }
    }
    
    // Actually, let's fix that function:
    fn longest_with_announcement_fixed<'a>(
        x: &'a str,
        y: &'a str,  // Both parameters need the same lifetime
        ann: &str,
    ) -> &'a str {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let result = longest_with_announcement_fixed("hello", "world", "Today is announcement day!");
    println!("Longest with announcement: '{}'", result);
}

fn struct_lifetime_examples() {
    // Struct that holds a reference needs lifetime annotations
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    impl<'a> ImportantExcerpt<'a> {
        // Method that returns a reference with the same lifetime as self
        fn level(&self) -> i32 {
            3
        }
        
        // Method with lifetime annotation
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("Important excerpt: {:?}", i);
    println!("Level: {}", i.level());
    
    let returned_part = i.announce_and_return_part("Here's an important excerpt");
    println!("Returned part: '{}'", returned_part);
    
    // Example with multiple lifetime parameters
    struct DoubleReference<'a, 'b> {
        first: &'a str,
        second: &'b str,
    }
    
    let string1 = String::from("first");
    let string2 = String::from("second");
    
    let double_ref = DoubleReference {
        first: &string1,
        second: &string2,
    };
    
    println!("Double reference: first='{}', second='{}'", double_ref.first, double_ref.second);
}

fn lifetime_elision_examples() {
    println!("=== Lifetime Elision Rules ===");
    println!("The compiler can infer lifetimes in many cases using these rules:");
    println!("1. Each parameter that's a reference gets its own lifetime parameter");
    println!("2. If there's one input lifetime, it's assigned to all output lifetimes");
    println!("3. If there's a &self parameter, its lifetime is assigned to all outputs");
    println!();
    
    // Rule 1 & 2: One input parameter -> lifetime can be elided
    fn first_word_elided(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    
    // This is equivalent to:
    // fn first_word_explicit<'a>(s: &'a str) -> &'a str { ... }
    
    let test_string = "hello world";
    println!("First word (elided): '{}'", first_word_elided(test_string));
    
    // Rule 3: Methods with &self can often elide lifetimes
    struct StringHolder {
        content: String,
    }
    
    impl StringHolder {
        // Lifetime elided - returns reference with same lifetime as &self
        fn get_content(&self) -> &str {
            &self.content
        }
        
        // Multiple parameters - can't elide, but &self rule applies to return
        fn get_content_or_default<'a>(&'a self, default: &'a str) -> &'a str {
            if self.content.is_empty() {
                default
            } else {
                &self.content
            }
        }
    }
    
    let holder = StringHolder {
        content: String::from("Some content"),
    };
    
    println!("Content: '{}'", holder.get_content());
    println!("Content or default: '{}'", holder.get_content_or_default("default"));
    
    // Cases where elision doesn't work - need explicit lifetimes
    fn longest_explicit<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
    
    println!("Longest (explicit): '{}'", longest_explicit("hello", "world"));
}

fn static_lifetime_examples() {
    // String literals have 'static lifetime
    let s: &'static str = "I have a static lifetime.";
    println!("Static string: '{}'", s);
    
    // Static variables also have 'static lifetime
    static GLOBAL_STRING: &str = "This is a global string";
    println!("Global string: '{}'", GLOBAL_STRING);
    
    // Function that requires 'static lifetime
    fn takes_static_str(s: &'static str) -> &'static str {
        s
    }
    
    let static_result = takes_static_str("This literal has static lifetime");
    println!("Static result: '{}'", static_result);
    
    // Be careful with 'static - it means the reference is valid for the entire program
    // Don't use it unless you actually need it for the entire program duration
    
    // Example of when you might use 'static in error handling
    fn get_error_message() -> &'static str {
        "Something went wrong"
    }
    
    println!("Error message: '{}'", get_error_message());
}

fn combined_examples() {
    println!("=== Combining Generics, Trait Bounds, and Lifetimes ===");
    
    // Function with generic type parameter, trait bound, and lifetime parameter
    use std::fmt::Display;
    
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
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let announcement = "Today is someone's birthday!";
    
    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        announcement,
    );
    println!("The longest string is '{}'", result);
    
    // Struct with generics, traits, and lifetimes
    struct DataHolder<'a, T>
    where
        T: Display + Clone,
    {
        data: &'a T,
        name: String,
    }
    
    impl<'a, T> DataHolder<'a, T>
    where
        T: Display + Clone,
    {
        fn new(data: &'a T, name: String) -> Self {
            DataHolder { data, name }
        }
        
        fn display_data(&self) {
            println!("{}: {}", self.name, self.data);
        }
        
        fn get_data_copy(&self) -> T {
            self.data.clone()
        }
    }
    
    let number = 42;
    let holder = DataHolder::new(&number, "My Number".to_string());
    holder.display_data();
    
    let copied_data = holder.get_data_copy();
    println!("Copied data: {}", copied_data);
    
    // Example with trait objects and lifetimes
    trait Drawable {
        fn draw(&self) -> String;
    }
    
    struct Circle {
        radius: f64,
    }
    
    impl Drawable for Circle {
        fn draw(&self) -> String {
            format!("Circle with radius {}", self.radius)
        }
    }
    
    struct Square {
        side: f64,
    }
    
    impl Drawable for Square {
        fn draw(&self) -> String {
            format!("Square with side {}", self.side)
        }
    }
    
    // Function that takes a trait object with a lifetime
    fn draw_shape<'a>(shape: &'a dyn Drawable) -> &'a dyn Drawable {
        println!("Drawing: {}", shape.draw());
        shape
    }
    
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 3.0 };
    
    let drawn_circle = draw_shape(&circle);
    let drawn_square = draw_shape(&square);
    
    println!("Drawn circle: {}", drawn_circle.draw());
    println!("Drawn square: {}", drawn_square.draw());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetime_function() {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() { x } else { y }
        }
        
        assert_eq!(longest("hello", "world"), "hello");
        assert_eq!(longest("rust", "programming"), "programming");
    }

    #[test]
    fn test_lifetime_struct() {
        struct TextHolder<'a> {
            text: &'a str,
        }
        
        let content = String::from("test content");
        let holder = TextHolder { text: &content };
        
        assert_eq!(holder.text, "test content");
    }

    #[test]
    fn test_lifetime_elision() {
        fn get_first_word(s: &str) -> &str {
            s.split_whitespace().next().unwrap_or("")
        }
        
        assert_eq!(get_first_word("hello world"), "hello");
        assert_eq!(get_first_word("rust"), "rust");
    }

    #[test]
    fn test_static_lifetime() {
        fn get_static() -> &'static str {
            "static string"
        }
        
        assert_eq!(get_static(), "static string");
    }

    #[test]
    fn test_combined_generics_traits_lifetimes() {
        use std::fmt::Display;
        
        fn process_data<'a, T: Display + Clone>(data: &'a T) -> T {
            println!("Processing: {}", data);
            data.clone()
        }
        
        let original = 42;
        let processed = process_data(&original);
        
        assert_eq!(processed, 42);
    }
}