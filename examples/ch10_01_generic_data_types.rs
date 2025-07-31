//! # Chapter 10.1: Generic Data Types
//! 
//! This example demonstrates:
//! - Generic functions to reduce code duplication
//! - Generic structs with type parameters
//! - Generic enums (like Option<T> and Result<T, E>)
//! - Generic method implementations
//! - Performance considerations of generics

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 10.1", "Generic Data Types");

    println!("=== Generic Functions ===");
    generic_function_examples();
    
    println!("\n=== Generic Structs ===");
    generic_struct_examples();
    
    println!("\n=== Generic Enums ===");
    generic_enum_examples();
    
    println!("\n=== Generic Methods ===");
    generic_method_examples();
    
    println!("\n=== Multiple Type Parameters ===");
    multiple_type_parameter_examples();
    
    println!("\n=== Performance of Generics ===");
    performance_examples();
}

fn generic_function_examples() {
    // Before generics: separate functions for each type
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
    
    // After generics: one function for any comparable type
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    
    // Using the functions
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number (i32 function): {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char (char function): {}", result);
    
    // Using the generic function for both types
    let result = largest(&number_list);
    println!("The largest number (generic function): {}", result);
    
    let result = largest(&char_list);
    println!("The largest char (generic function): {}", result);
    
    // Generic function with multiple parameters
    fn compare_and_display<T: PartialOrd + std::fmt::Display>(x: T, y: T) {
        if x > y {
            println!("{} is greater than {}", x, y);
        } else {
            println!("{} is less than or equal to {}", x, y);
        }
    }
    
    compare_and_display(5, 3);
    compare_and_display(1.5, 2.8);
    compare_and_display('a', 'z');
}

fn generic_struct_examples() {
    // Generic struct with one type parameter
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    
    // Both coordinates must be the same type
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    
    println!("Integer point: {:?}", integer_point);
    println!("Float point: {:?}", float_point);
    
    // Generic struct with multiple type parameters
    #[derive(Debug)]
    struct PointMixed<T, U> {
        x: T,
        y: U,
    }
    
    // Coordinates can be different types
    let mixed_point = PointMixed { x: 5, y: 4.0 };
    let string_point = PointMixed { x: "hello", y: 'c' };
    
    println!("Mixed point: {:?}", mixed_point);
    println!("String point: {:?}", string_point);
    
    // Generic struct for a simple container
    #[derive(Debug)]
    struct Container<T> {
        items: Vec<T>,
    }
    
    impl<T> Container<T> {
        fn new() -> Self {
            Container { items: Vec::new() }
        }
        
        fn add(&mut self, item: T) {
            self.items.push(item);
        }
        
        fn get(&self, index: usize) -> Option<&T> {
            self.items.get(index)
        }
        
        fn len(&self) -> usize {
            self.items.len()
        }
    }
    
    let mut string_container = Container::new();
    string_container.add("hello".to_string());
    string_container.add("world".to_string());
    
    println!("String container: {:?}", string_container);
    println!("First item: {:?}", string_container.get(0));
    
    let mut number_container = Container::new();
    number_container.add(42);
    number_container.add(100);
    
    println!("Number container: {:?}", number_container);
    println!("Container length: {}", number_container.len());
}

fn generic_enum_examples() {
    // Demonstrating how Option<T> works (built-in generic enum)
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent number: {:?}", absent_number);
    
    // Custom generic enum
    #[derive(Debug)]
    enum MyResult<T, E> {
        Success(T),
        Failure(E),
    }
    
    let success: MyResult<i32, String> = MyResult::Success(42);
    let failure: MyResult<i32, String> = MyResult::Failure("Something went wrong".to_string());
    
    println!("Success result: {:?}", success);
    println!("Failure result: {:?}", failure);
    
    // Generic enum for a linked list
    #[derive(Debug)]
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Generic linked list: {:?}", list);
    
    // Generic enum for a tree
    #[derive(Debug)]
    enum BinaryTree<T> {
        Empty,
        Node {
            value: T,
            left: Box<BinaryTree<T>>,
            right: Box<BinaryTree<T>>,
        },
    }
    
    let tree = BinaryTree::Node {
        value: 5,
        left: Box::new(BinaryTree::Node {
            value: 3,
            left: Box::new(BinaryTree::Empty),
            right: Box::new(BinaryTree::Empty),
        }),
        right: Box::new(BinaryTree::Node {
            value: 7,
            left: Box::new(BinaryTree::Empty),
            right: Box::new(BinaryTree::Empty),
        }),
    };
    
    println!("Binary tree: {:?}", tree);
}

fn generic_method_examples() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    
    // Methods on generic structs
    impl<T> Point<T> {
        fn new(x: T, y: T) -> Self {
            Point { x, y }
        }
        
        fn x(&self) -> &T {
            &self.x
        }
        
        fn y(&self) -> &T {
            &self.y
        }
    }
    
    // Methods only for specific types
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    
    let point = Point::new(5, 10);
    println!("Point: {:?}", point);
    println!("x coordinate: {}", point.x());
    println!("y coordinate: {}", point.y());
    
    let float_point = Point::new(3.0, 4.0);
    println!("Float point: {:?}", float_point);
    println!("Distance from origin: {}", float_point.distance_from_origin());
    
    // Generic methods with different type parameters
    struct Pair<T, U> {
        first: T,
        second: U,
    }
    
    impl<T, U> Pair<T, U> {
        fn new(first: T, second: U) -> Self {
            Pair { first, second }
        }
        
        // Method with additional generic type parameter
        fn mix_with<V, W>(self, other: Pair<V, W>) -> Pair<T, W> {
            Pair {
                first: self.first,
                second: other.second,
            }
        }
    }
    
    let pair1 = Pair::new(5, "hello");
    let pair2 = Pair::new(true, 3.14);
    let mixed = pair1.mix_with(pair2);
    
    println!("Mixed pair: first = {}, second = {}", mixed.first, mixed.second);
}

fn multiple_type_parameter_examples() {
    // Struct with multiple generic parameters
    #[derive(Debug)]
    struct Triple<T, U, V> {
        first: T,
        second: U,
        third: V,
    }
    
    impl<T, U, V> Triple<T, U, V> {
        fn new(first: T, second: U, third: V) -> Self {
            Triple { first, second, third }
        }
        
        fn get_first(&self) -> &T {
            &self.first
        }
        
        fn get_second(&self) -> &U {
            &self.second
        }
        
        fn get_third(&self) -> &V {
            &self.third
        }
    }
    
    let triple = Triple::new(42, "world", true);
    println!("Triple: {:?}", triple);
    println!("First: {}", triple.get_first());
    println!("Second: {}", triple.get_second());
    println!("Third: {}", triple.get_third());
    
    // Generic function with multiple type parameters
    fn compare_types<T, U>(t: T, u: U) -> String 
    where
        T: std::fmt::Debug,
        U: std::fmt::Debug,
    {
        format!("T = {:?}, U = {:?}", t, u)
    }
    
    let result = compare_types(100, "test");
    println!("Comparison: {}", result);
    
    let result = compare_types(vec![1, 2, 3], 3.14);
    println!("Comparison: {}", result);
}

fn performance_examples() {
    println!("=== Performance: Monomorphization ===");
    println!("Generics in Rust have zero runtime cost!");
    println!("The compiler generates specific code for each concrete type used.");
    println!();
    
    // This generic function...
    fn generic_function<T: std::fmt::Display>(value: T) {
        println!("Value: {}", value);
    }
    
    // ...becomes multiple specific functions at compile time:
    // fn generic_function_i32(value: i32) { println!("Value: {}", value); }
    // fn generic_function_string(value: String) { println!("Value: {}", value); }
    
    generic_function(42);
    generic_function("hello".to_string());
    
    println!("The compiler generated separate optimized functions for i32 and String!");
    
    // Example: Generic collections
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let strings: Vec<String> = vec!["a".to_string(), "b".to_string()];
    
    println!("Numbers: {:?}", numbers);
    println!("Strings: {:?}", strings);
    
    println!("\nBoth Vec<i32> and Vec<String> are compiled to optimized, type-specific code.");
    println!("No runtime overhead for using generics!");
    
    // Demonstrating that the same generic code works with different types
    fn process_collection<T: std::fmt::Debug>(items: &[T]) {
        println!("Processing {} items: {:?}", items.len(), items);
    }
    
    process_collection(&numbers);
    process_collection(&strings);
    process_collection(&[true, false, true]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_functions() {
        fn identity<T>(x: T) -> T {
            x
        }
        
        assert_eq!(identity(42), 42);
        assert_eq!(identity("hello"), "hello");
        assert_eq!(identity(true), true);
    }

    #[test]
    fn test_generic_structs() {
        #[derive(Debug, PartialEq)]
        struct Wrapper<T> {
            value: T,
        }
        
        let int_wrapper = Wrapper { value: 42 };
        let string_wrapper = Wrapper { value: "test".to_string() };
        
        assert_eq!(int_wrapper.value, 42);
        assert_eq!(string_wrapper.value, "test");
    }

    #[test]
    fn test_generic_enums() {
        enum Either<L, R> {
            Left(L),
            Right(R),
        }
        
        let left: Either<i32, String> = Either::Left(42);
        let right: Either<i32, String> = Either::Right("hello".to_string());
        
        match left {
            Either::Left(value) => assert_eq!(value, 42),
            Either::Right(_) => panic!("Expected Left"),
        }
        
        match right {
            Either::Left(_) => panic!("Expected Right"),
            Either::Right(value) => assert_eq!(value, "hello"),
        }
    }

    #[test]
    fn test_generic_methods() {
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
            
            fn len(&self) -> usize {
                self.items.len()
            }
        }
        
        let mut container = Container::new();
        container.push(1);
        container.push(2);
        container.push(3);
        
        assert_eq!(container.len(), 3);
    }
}