//! # Chapter 15.2: Treating Smart Pointers Like Regular References with the Deref Trait
//! 
//! This example demonstrates:
//! - Implementing the Deref trait for custom smart pointers
//! - Deref coercion and automatic dereferencing
//! - How the dereference operator (*) works
//! - Making smart pointers behave like regular references
//! 
//! Run this example with: `cargo run --example ch15_02_deref_trait`

use rust_book_examples::print_chapter_header;
use std::ops::Deref;

fn main() {
    print_chapter_header("Chapter 15.2", "Treating Smart Pointers Like Regular References with the Deref Trait");

    println!("The Deref trait allows smart pointers to act like regular references!");
    println!();

    demonstrate_basic_dereferencing();
    demonstrate_custom_smart_pointer();
    demonstrate_deref_coercion();
    demonstrate_multiple_deref_levels();
    demonstrate_method_calls();
}

/// Demonstrates basic dereferencing with Box
fn demonstrate_basic_dereferencing() {
    println!("=== Basic Dereferencing ===");
    
    // Regular reference
    let x = 5;
    let y = &x;
    
    println!("Regular reference:");
    println!("  x = {}", x);
    println!("  y = &x = {:p}", y);
    println!("  *y = {}", *y);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
    
    // Box smart pointer (heap allocation)
    let x = 5;
    let y = Box::new(x);
    
    println!("\nBox smart pointer:");
    println!("  x = {}", x);
    println!("  y = Box::new(x)");
    println!("  *y = {}", *y);  // Deref trait makes this work!
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
    
    println!();
}

/// Custom smart pointer implementation
#[derive(Debug)]
struct MyBox<T> {
    value: T,
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox { value: x }
    }
}

// Implementing Deref trait to make MyBox work with * operator
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// Demonstrates our custom smart pointer
fn demonstrate_custom_smart_pointer() {
    println!("=== Custom Smart Pointer (MyBox) ===");
    
    let x = 5;
    let y = MyBox::new(x);
    
    println!("MyBox smart pointer:");
    println!("  x = {}", x);
    println!("  y = MyBox::new(x)");
    println!("  *y = {}", *y);  // Our Deref implementation makes this work!
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
    
    // What happens behind the scenes when we write *y:
    // 1. Rust calls y.deref() to get &5
    // 2. Then applies the * operator to the reference
    // Equivalent to: *(y.deref())
    
    println!("\nBehind the scenes:");
    println!("  *y is equivalent to *(y.deref())");
    println!("  y.deref() returns: {:p}", y.deref());
    println!("  *(y.deref()) = {}", *(y.deref()));
    
    println!();
}

/// Demonstrates deref coercion - automatic conversion from smart pointer to reference
fn demonstrate_deref_coercion() {
    println!("=== Deref Coercion ===");
    
    // Function that expects a string slice
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }
    
    // MyBox<String> can be automatically coerced to &str
    let m = MyBox::new(String::from("Rust"));
    
    println!("Deref coercion in action:");
    println!("  MyBox<String> -> &String -> &str");
    
    // This works because of deref coercion:
    // MyBox<String> -> &String (via Deref) -> &str (via Deref on String)
    hello(&m);
    
    // Without deref coercion, we'd need to write:
    hello(&(*m)[..]);
    println!("  Without coercion: hello(&(*m)[..])");
    println!("  With coercion:    hello(&m)");
    
    // More examples of deref coercion
    let boxed_string = Box::new(String::from("boxed"));
    hello(&boxed_string);  // Box<String> -> &String -> &str
    
    let my_string = MyBox::new(String::from("my string"));
    let string_len = my_string.len();  // Calls String::len() directly!
    println!("  Length of MyBox<String>: {}", string_len);
    
    println!();
}

/// Demonstrates multiple levels of dereferencing
fn demonstrate_multiple_deref_levels() {
    println!("=== Multiple Deref Levels ===");
    
    // Nested smart pointers
    let x = 42;
    let y = MyBox::new(x);
    let z = MyBox::new(y);
    let w = MyBox::new(z);
    
    println!("Nested smart pointers:");
    println!("  x = {}", x);
    println!("  y = MyBox::new(x)");
    println!("  z = MyBox::new(y)");
    println!("  w = MyBox::new(z)");
    
    // Rust automatically dereferences through multiple levels
    println!("\nAutomatic multi-level dereferencing:");
    println!("  *w = {:?}", *w);    // MyBox<MyBox<MyBox<i32>>> -> i32
    println!("  **w = {:?}", **w);  // MyBox<MyBox<i32>> -> MyBox<i32>
    
    // Function that takes a reference to i32
    fn print_number(n: &i32) {
        println!("Number: {}", n);
    }
    
    print_number(&w);  // MyBox<MyBox<MyBox<i32>>> -> &i32 automatically!
    
    println!();
}

/// Demonstrates calling methods through deref
fn demonstrate_method_calls() {
    println!("=== Method Calls Through Deref ===");
    
    let words = MyBox::new(vec!["hello", "world", "rust"]);
    
    println!("MyBox<Vec<&str>> method calls:");
    
    // These method calls work because of deref coercion
    // MyBox<Vec<&str>> -> &Vec<&str> -> Vec<&str> methods available
    println!("  Length: {}", words.len());
    println!("  First element: {:?}", words.get(0));
    println!("  Is empty: {}", words.is_empty());
    
    // Iterator methods also work
    println!("  All words:");
    for (i, word) in words.iter().enumerate() {
        println!("    {}: {}", i, word);
    }
    
    // String example
    let text = MyBox::new(String::from("Hello, World!"));
    println!("\nMyBox<String> method calls:");
    println!("  Length: {}", text.len());
    println!("  Uppercase: {}", text.to_uppercase());
    println!("  Contains 'World': {}", text.contains("World"));
    
    println!();
}

// === ADVANCED DEREF EXAMPLES ===

/// Example of deref coercion with function parameters
fn function_parameter_coercion() {
    println!("=== Function Parameter Coercion ===");
    
    // Functions with different parameter types
    fn takes_str_slice(s: &str) {
        println!("String slice: {}", s);
    }
    
    fn takes_string_ref(s: &String) {
        println!("String reference: {}", s);
    }
    
    let my_string = MyBox::new(String::from("coercion test"));
    
    // Both calls work due to deref coercion
    takes_string_ref(&my_string);  // MyBox<String> -> &String
    takes_str_slice(&my_string);   // MyBox<String> -> &String -> &str
    
    println!();
}

/// Custom smart pointer for demonstration
struct SmartString {
    data: String,
    access_count: std::cell::Cell<usize>,
}

impl SmartString {
    fn new(s: &str) -> Self {
        SmartString {
            data: s.to_string(),
            access_count: std::cell::Cell::new(0),
        }
    }
    
    fn access_count(&self) -> usize {
        self.access_count.get()
    }
}

impl Deref for SmartString {
    type Target = String;
    
    fn deref(&self) -> &Self::Target {
        // Increment access count each time we deref
        let count = self.access_count.get();
        self.access_count.set(count + 1);
        &self.data
    }
}

/// Demonstrates custom behavior in deref
#[allow(dead_code)]
fn demonstrate_custom_deref_behavior() {
    println!("=== Custom Deref Behavior ===");
    
    let smart = SmartString::new("tracked string");
    
    println!("Smart string with access tracking:");
    println!("  Content: {}", &*smart);  // Triggers deref
    println!("  Access count: {}", smart.access_count());
    
    println!("  Length: {}", smart.len());  // Triggers deref
    println!("  Access count: {}", smart.access_count());
    
    let _substring = &smart[0..7];  // Triggers deref
    println!("  Access count after substring: {}", smart.access_count());
}

// === DEREF TRAIT VARIANTS ===

/// Demonstrates DerefMut for mutable dereferencing
struct MyMutBox<T> {
    value: T,
}

impl<T> MyMutBox<T> {
    fn new(x: T) -> MyMutBox<T> {
        MyMutBox { value: x }
    }
}

impl<T> Deref for MyMutBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> std::ops::DerefMut for MyMutBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[allow(dead_code)]
fn demonstrate_deref_mut() {
    println!("=== DerefMut Trait ===");
    
    let mut mutable_box = MyMutBox::new(String::from("mutable"));
    
    println!("Before mutation: {}", *mutable_box);
    
    // DerefMut allows us to get mutable reference
    mutable_box.push_str(" content");
    
    println!("After mutation: {}", *mutable_box);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mybox_deref() {
        let x = 5;
        let y = MyBox::new(x);
        
        assert_eq!(5, *y);
        assert_eq!(&5, y.deref());
    }

    #[test]
    fn test_deref_coercion() {
        fn takes_str_slice(s: &str) -> usize {
            s.len()
        }
        
        let my_string = MyBox::new(String::from("hello"));
        assert_eq!(5, takes_str_slice(&my_string));
    }

    #[test]
    fn test_nested_deref() {
        let nested = MyBox::new(MyBox::new(42));
        assert_eq!(42, *nested);
    }

    #[test]
    fn test_method_calls() {
        let words = MyBox::new(vec!["rust", "is", "awesome"]);
        
        assert_eq!(3, words.len());
        assert_eq!(Some(&"rust"), words.get(0));
        assert!(!words.is_empty());
    }

    #[test]
    fn test_smart_string() {
        let smart = SmartString::new("test");
        
        // Access the string content
        let _ = &*smart;
        assert_eq!(1, smart.access_count());
        
        // Access again
        let _ = smart.len();
        assert_eq!(2, smart.access_count());
    }

    #[test]
    fn test_deref_mut() {
        let mut mutable_box = MyMutBox::new(String::from("hello"));
        
        mutable_box.push_str(" world");
        assert_eq!("hello world", &*mutable_box);
    }
}