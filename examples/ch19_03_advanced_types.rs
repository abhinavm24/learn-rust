//! Chapter 19.3: Advanced Types
//! 
//! This example demonstrates:
//! - Type aliases for creating synonyms
//! - The never type (!) for functions that never return
//! - Dynamically Sized Types (DST) and the Sized trait
//! - Function pointers vs closure types
//! - Working with trait objects and object safety

use rust_book_examples::print_chapter_header;
use std::fmt;

// Define macro early to avoid scope issues
macro_rules! continue_or_return {
    () => {
        0 // Return a default value instead of causing issues
    };
}

fn main() {
    print_chapter_header("Chapter 19.3", "Advanced Types");
    
    println!("=== Type Aliases ===");
    type_aliases_example();
    
    println!("\n=== The Never Type ===");
    never_type_example();
    
    println!("\n=== Dynamically Sized Types ===");
    dynamically_sized_types_example();
    
    println!("\n=== Function Pointers ===");
    function_pointers_example();
    
    println!("\n=== Advanced Type Patterns ===");
    advanced_type_patterns();
}

// Type Aliases
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

// Generic type alias
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Complex type alias to reduce repetition
type IoResult<T> = std::result::Result<T, std::io::Error>;
type NetworkCallback = Box<dyn Fn(&str) -> IoResult<String> + Send + Sync>;

fn type_aliases_example() {
    println!("Type aliases create synonyms for existing types:");
    
    // Basic type alias usage
    let x: i32 = 5;
    let y: Kilometers = 5;
    
    println!("x + y = {}", x + y); // Works because both are i32
    println!("Distance: {} kilometers", y);
    
    // Type alias doesn't create a new type, just a synonym
    let distance_km: Kilometers = 42;
    let distance_int: i32 = distance_km; // No conversion needed
    println!("Distance as i32: {}", distance_int);
    
    // Complex type aliases
    let f: Thunk = Box::new(|| println!("Hello from thunk!"));
    f();
    
    // Generic type alias
    fn might_fail(should_fail: bool) -> Result<String> {
        if should_fail {
            Err("Something went wrong".into())
        } else {
            Ok("Success!".to_string())
        }
    }
    
    match might_fail(false) {
        Ok(msg) => println!("Operation succeeded: {}", msg),
        Err(e) => println!("Operation failed: {}", e),
    }
    
    // Type alias in function signatures
    fn create_callback() -> NetworkCallback {
        Box::new(|input: &str| {
            Ok(format!("Processed: {}", input))
        })
    }
    
    let callback = create_callback();
    match callback("test data") {
        Ok(result) => println!("Callback result: {}", result),
        Err(e) => println!("Callback error: {}", e),
    }
    
    // Type alias for function pointers
    type BinaryOp = fn(i32, i32) -> i32;
    
    let add: BinaryOp = |a, b| a + b;
    let multiply: BinaryOp = |a, b| a * b;
    
    println!("3 + 4 = {}", add(3, 4));
    println!("3 * 4 = {}", multiply(3, 4));
    
    // Type alias with associated types
    trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    
    type IntIterator = dyn Iterator<Item = i32>;
    
    struct Counter {
        current: i32,
        max: i32,
    }
    
    impl Iterator for Counter {
        type Item = i32;
        
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
    
    let mut counter = Counter { current: 0, max: 3 };
    let iter: &mut IntIterator = &mut counter;
    
    println!("Iterator values:");
    while let Some(value) = iter.next() {
        println!("  {}", value);
    }
}

// Never type examples
fn never_type_example() {
    println!("The never type (!) represents computations that never return:");
    
    // panic! has type !
    fn diverging_function() -> ! {
        panic!("This function never returns!")
    }
    
    // loop without break has type !
    fn infinite_loop() -> ! {
        loop {
            println!("This runs forever!");
            std::thread::sleep(std::time::Duration::from_millis(10));
            break; // We break to avoid infinite output
        }
        unreachable!() // This makes the function return !
    }
    
    // Never type is useful in match expressions
    let input = "42";
    let number: u32 = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Failed to parse, using default");
            continue_or_return!() // Custom macro that returns !
        }
    };
    
    println!("Parsed number: {}", number);
    
    // Never type can coerce to any other type
    let x: i32 = match get_option() {
        Some(value) => value,
        None => return, // ! coerces to i32
    };
    
    println!("Value: {}", x);
    
    // Option and Result with infallible types (commented out - requires nightly)
    // fn process_result() -> std::result::Result<i32, !> {
    //     Ok(42) // This Result can never have an Err value
    // }
    
    // match process_result() {
    //     Ok(value) => println!("Infallible result: {}", value),
    //     // No need for Err case since ! means it can't happen
    // }
    
    // Function that demonstrates never type coercion
    fn example_with_never_type(condition: bool) -> i32 {
        if condition {
            42
        } else {
            panic!("This is of type !, but coerces to i32")
        }
    }
    
    println!("Example result: {}", example_with_never_type(true));
}

// Helper functions for never type example
fn get_option() -> Option<i32> {
    Some(10)
}

// Dynamically Sized Types
fn dynamically_sized_types_example() {
    println!("Dynamically Sized Types (DSTs) and the Sized trait:");
    
    // str is a DST - we don't know its size at compile time
    let s1: &str = "Hello";
    let s2: &str = "Hello, world!";
    
    println!("s1: '{}' (length: {})", s1, s1.len());
    println!("s2: '{}' (length: {})", s2, s2.len());
    
    // [T] is a DST - slices have unknown size at compile time
    let arr1 = [1, 2, 3];
    let arr2 = [1, 2, 3, 4, 5];
    
    let slice1: &[i32] = &arr1;
    let slice2: &[i32] = &arr2;
    
    println!("slice1: {:?} (length: {})", slice1, slice1.len());
    println!("slice2: {:?} (length: {})", slice2, slice2.len());
    
    // Trait objects are DSTs
    trait Draw {
        fn draw(&self);
    }
    
    struct Circle {
        radius: f64,
    }
    
    impl Draw for Circle {
        fn draw(&self) {
            println!("Drawing circle with radius {}", self.radius);
        }
    }
    
    struct Rectangle {
        width: f64,
        height: f64,
    }
    
    impl Draw for Rectangle {
        fn draw(&self) {
            println!("Drawing rectangle {}x{}", self.width, self.height);
        }
    }
    
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 10.0, height: 20.0 };
    
    // These are trait objects (DSTs)
    let drawable1: &dyn Draw = &circle;
    let drawable2: &dyn Draw = &rectangle;
    
    drawable1.draw();
    drawable2.draw();
    
    // The Sized trait
    fn sized_function<T: Sized>(t: T) -> T {
        t // T must have known size at compile time
    }
    
    fn maybe_sized_function<T: ?Sized>(t: &T) -> &T {
        t // T might not have known size, so we use a reference
    }
    
    let number = 42;
    let sized_result = sized_function(number);
    println!("Sized function result: {}", sized_result);
    
    let string_slice: &str = "Hello";
    let maybe_sized_result = maybe_sized_function(string_slice);
    println!("Maybe sized function result: '{}'", maybe_sized_result);
    
    // Generic functions have implicit Sized bound
    fn implicit_sized<T>(_t: T) {} // Equivalent to fn implicit_sized<T: Sized>(t: T) {}
    
    // To work with DSTs, you need ?Sized
    fn work_with_dst<T: ?Sized + fmt::Display>(t: &T) {
        println!("DST value: {}", t);
    }
    
    work_with_dst(&42); // Works with sized types
    work_with_dst("Hello"); // Works with DSTs
    
    // Box<dyn Trait> is sized even though dyn Trait is not
    let boxed_drawable: Box<dyn Draw> = Box::new(Circle { radius: 3.0 });
    boxed_drawable.draw();
    
    println!("Box<dyn Draw> has size: {} bytes", std::mem::size_of::<Box<dyn Draw>>());
    println!("&dyn Draw has size: {} bytes", std::mem::size_of::<&dyn Draw>());
    // println!("dyn Draw has size: {}", std::mem::size_of::<dyn Draw>()); // This would error
}

// Function pointers
fn function_pointers_example() {
    println!("Function pointers vs closures:");
    
    // Function pointer type
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    
    // Function pointers implement all closure traits
    let answer = do_twice(add_one, 5);
    println!("do_twice(add_one, 5) = {}", answer);
    
    // Function pointers vs closures
    let closure = |x| x + 1;
    let answer2 = do_twice(closure, 5); // Closure coerces to function pointer
    println!("do_twice(closure, 5) = {}", answer2);
    
    // Function that accepts different callable types
    fn call_with_different_types() {
        fn process_with_fn<F>(f: F, x: i32) -> i32 
        where F: Fn(i32) -> i32 {
            f(x)
        }
        
        fn process_with_fn_pointer(f: fn(i32) -> i32, x: i32) -> i32 {
            f(x)
        }
        
        let multiply_by_2 = |x| x * 2;
        
        println!("With Fn trait: {}", process_with_fn(multiply_by_2, 5));
        println!("With Fn trait (function): {}", process_with_fn(add_one, 5));
        
        println!("With fn pointer: {}", process_with_fn_pointer(add_one, 5));
        // This would work too because closures that don't capture can coerce:
        println!("With fn pointer (closure): {}", process_with_fn_pointer(|x| x * 3, 5));
    }
    
    call_with_different_types();
    
    // Using function pointers with collections
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string()) // Closure
        .collect();
    
    println!("Strings from closure: {:?}", list_of_strings);
    
    let list_of_strings2: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string) // Function pointer (method reference)
        .collect();
    
    println!("Strings from function pointer: {:?}", list_of_strings2);
    
    // Enum variants as function pointers
    #[derive(Debug)]
    enum Status {
        Value(u32),
    }
    
    let list_of_statuses: Vec<Status> = (0u32..20)
        .map(Status::Value) // Constructor function pointer
        .collect();
    
    println!("First few statuses: {:?}", &list_of_statuses[..3]);
    
    // Function that returns function pointers
    fn get_operation(op: char) -> fn(i32, i32) -> i32 {
        match op {
            '+' => |a, b| a + b,
            '-' => |a, b| a - b,
            '*' => |a, b| a * b,
            '/' => |a, b| a / b,
            _ => |_, _| 0,
        }
    }
    
    let add_op = get_operation('+');
    let multiply_op = get_operation('*');
    
    println!("10 + 5 = {}", add_op(10, 5));
    println!("10 * 5 = {}", multiply_op(10, 5));
}

fn advanced_type_patterns() {
    println!("Advanced type patterns:");
    
    // Type aliases with trait bounds
    trait Drawable {
        fn draw(&self);
    }
    
    trait Clickable {
        fn click(&self);
    }
    
    // This needs a supertrait approach in stable Rust
    trait Interactive: Drawable + Clickable + Send {}
    type InteractiveElement = Box<dyn Interactive>;
    
    struct Button {
        label: String,
    }
    
    impl Drawable for Button {
        fn draw(&self) {
            println!("Drawing button: {}", self.label);
        }
    }
    
    impl Clickable for Button {
        fn click(&self) {
            println!("Button '{}' clicked!", self.label);
        }
    }
    
    // Implement Interactive for Button
    impl Interactive for Button {}
    
    let button: InteractiveElement = Box::new(Button {
        label: "OK".to_string(),
    });
    
    button.draw();
    button.click();
    
    // Higher-Ranked Trait Bounds (HRTB)
    fn higher_ranked_example() {
        // This function can accept any closure that works with any lifetime
        fn call_with_any_lifetime<F>(f: F) 
        where 
            F: for<'a> Fn(&'a str) -> &'a str,
        {
            let local_string = "hello";
            let result = f(local_string);
            println!("Result: {}", result);
        }
        
        call_with_any_lifetime(|s| s);
        call_with_any_lifetime(|s| if s.len() > 3 { s } else { "short" });
    }
    
    higher_ranked_example();
    
    // Associated types with bounds
    trait CollectInto<T> {
        type Output: IntoIterator<Item = T>;
        
        fn collect_into(self) -> Self::Output;
    }
    
    impl<T> CollectInto<T> for Vec<T> {
        type Output = Vec<T>;
        
        fn collect_into(self) -> Self::Output {
            self
        }
    }
    
    let numbers = vec![1, 2, 3, 4, 5];
    let collected = numbers.collect_into();
    
    println!("Collected numbers:");
    for num in collected {
        println!("  {}", num);
    }
    
    // Phantom types
    use std::marker::PhantomData;
    
    struct Container<T> {
        data: Vec<u8>,
        _phantom: PhantomData<T>,
    }
    
    impl<T> Container<T> {
        fn new() -> Self {
            Container {
                data: Vec::new(),
                _phantom: PhantomData,
            }
        }
        
        fn add_bytes(&mut self, bytes: Vec<u8>) {
            self.data.extend(bytes);
        }
    }
    
    // Different types even though they have the same runtime representation
    let mut int_container: Container<i32> = Container::new();
    let mut string_container: Container<String> = Container::new();
    
    int_container.add_bytes(vec![1, 2, 3, 4]);
    string_container.add_bytes(vec![72, 101, 108, 108, 111]); // "Hello"
    
    println!("Int container has {} bytes", int_container.data.len());
    println!("String container has {} bytes", string_container.data.len());
    
    // Zero-sized types
    struct EmptyStruct;
    struct EmptyTuple();
    
    println!("EmptyStruct size: {} bytes", std::mem::size_of::<EmptyStruct>());
    println!("EmptyTuple size: {} bytes", std::mem::size_of::<EmptyTuple>());
    println!("() size: {} bytes", std::mem::size_of::<()>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_aliases() {
        let distance: Kilometers = 100;
        let regular_int: i32 = distance; // Should work without conversion
        assert_eq!(regular_int, 100);
    }

    #[test]
    fn test_function_pointers() {
        fn double(x: i32) -> i32 {
            x * 2
        }
        
        fn apply_op(f: fn(i32) -> i32, x: i32) -> i32 {
            f(x)
        }
        
        assert_eq!(apply_op(double, 5), 10);
    }

    #[test]
    fn test_dst_with_slices() {
        let arr = [1, 2, 3, 4, 5];
        let slice: &[i32] = &arr;
        
        fn process_slice<T: ?Sized>(s: &T) -> &T {
            s
        }
        
        let result = process_slice(slice);
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_never_type_coercion() {
        fn might_panic(should_panic: bool) -> i32 {
            if should_panic {
                // In a real test we wouldn't panic, but this shows the type relationship
                return 42; // Instead of panic!() which returns !
            }
            100
        }
        
        assert_eq!(might_panic(false), 100);
        assert_eq!(might_panic(true), 42);
    }

    #[test]
    fn test_phantom_types() {
        use std::marker::PhantomData;
        
        struct TypedId<T> {
            id: u64,
            _phantom: PhantomData<T>,
        }
        
        impl<T> TypedId<T> {
            fn new(id: u64) -> Self {
                TypedId {
                    id,
                    _phantom: PhantomData,
                }
            }
        }
        
        struct User;
        struct Product;
        
        let user_id = TypedId::<User>::new(123);
        let product_id = TypedId::<Product>::new(456);
        
        assert_eq!(user_id.id, 123);
        assert_eq!(product_id.id, 456);
        
        // These would be different types even though they have the same runtime representation
        assert_eq!(std::mem::size_of::<TypedId<User>>(), std::mem::size_of::<TypedId<Product>>());
    }
}