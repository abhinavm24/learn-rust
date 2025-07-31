//! # Chapter 13.1: Closures: Anonymous Functions that Capture Their Environment
//! 
//! This example demonstrates:
//! - Creating closures with different syntaxes
//! - How closures capture values from their environment
//! - The three closure traits: Fn, FnMut, and FnOnce
//! - Type inference with closures
//! - Storing closures in structs and variables
//! - Using closures with iterators

use rust_book_examples::print_chapter_header;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    print_chapter_header("Chapter 13.1", "Closures: Anonymous Functions that Capture Their Environment");

    println!("=== Basic Closure Syntax ===");
    basic_closure_syntax();
    
    println!("\n=== Capturing Environment ===");
    capturing_environment();
    
    println!("\n=== Closure Type Inference ===");
    closure_type_inference();
    
    println!("\n=== Closure Traits: Fn, FnMut, FnOnce ===");
    closure_traits_examples();
    
    println!("\n=== Storing Closures ===");
    storing_closures();
    
    println!("\n=== Closures with Iterators ===");
    closures_with_iterators();
    
    println!("\n=== Practical Examples ===");
    practical_examples();
}

fn basic_closure_syntax() {
    // Different ways to define the same closure
    
    // With type annotations (most verbose)
    let add_one_verbose = |x: i32| -> i32 { x + 1 };
    
    // Without type annotations (common)
    let add_one = |x| x + 1;
    
    // Multi-line closure
    let complex_calculation = |x| {
        let doubled = x * 2;
        let squared = doubled * doubled;
        squared + 1
    };
    
    println!("add_one_verbose(5) = {}", add_one_verbose(5));
    println!("add_one(5) = {}", add_one(5));
    println!("complex_calculation(3) = {}", complex_calculation(3));
    
    // Closure with no parameters
    let greet = || println!("Hello from closure!");
    greet();
    
    // Closure with multiple parameters
    let multiply = |x, y| x * y;
    println!("multiply(4, 5) = {}", multiply(4, 5));
}

fn capturing_environment() {
    let x = 10;
    let y = 20;
    
    // Closures can capture variables from their environment
    let capture_by_reference = || println!("Captured x: {}, y: {}", x, y);
    capture_by_reference();
    
    // Still can use x and y here because they were captured by reference
    println!("x and y are still accessible: {}, {}", x, y);
    
    // Capturing by mutable reference
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("Count: {}", count);
    };
    
    increment();
    increment();
    increment();
    
    // count is accessible again after the mutable closure is done
    println!("Final count: {}", count);
    
    // Capturing by move (taking ownership)
    let name = String::from("Alice");
    let take_ownership = move || {
        println!("Moved name: {}", name);
        // name is owned by the closure now
    };
    
    take_ownership();
    // println!("name: {}", name); // This would cause a compile error!
    
    // Forcing move capture
    let vec = vec![1, 2, 3];
    let closure = move || {
        println!("Moved vector: {:?}", vec);
    };
    
    // vec is no longer accessible here
    closure();
}

fn closure_type_inference() {
    // The compiler infers closure types based on usage
    
    // This closure's type is inferred from first use
    let example_closure = |x| x;
    
    let s = example_closure(String::from("hello"));
    println!("String result: {}", s);
    
    // Once used with String, it can't be used with other types
    // let n = example_closure(5); // This would cause a compile error!
    
    // Different closure, different inference
    let num_closure = |x| x + 1;
    println!("Number result: {}", num_closure(5));
    
    // Generic function that accepts closures
    fn apply_to_3<F>(f: F) -> i32 
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }
    
    let double = |x| x * 2;
    let square = |x| x * x;
    let add_ten = |x| x + 10;
    
    println!("apply_to_3(double): {}", apply_to_3(double));
    println!("apply_to_3(square): {}", apply_to_3(square));
    println!("apply_to_3(add_ten): {}", apply_to_3(add_ten));
}

fn closure_traits_examples() {
    // Fn: can be called multiple times, captures by reference
    fn call_multiple_times<F>(f: F) 
    where
        F: Fn(i32) -> i32,
    {
        println!("First call: {}", f(1));
        println!("Second call: {}", f(2));
        println!("Third call: {}", f(3));
    }
    
    let multiplier = 5;
    let multiply_by_five = |x| x * multiplier;
    println!("Fn trait example:");
    call_multiple_times(multiply_by_five);
    
    // FnMut: can be called multiple times, can mutate captured variables
    fn call_and_modify<F>(mut f: F)
    where
        F: FnMut() -> i32,
    {
        println!("First call: {}", f());
        println!("Second call: {}", f());
        println!("Third call: {}", f());
    }
    
    let mut counter = 0;
    let increment_counter = || {
        counter += 1;
        counter
    };
    
    println!("\nFnMut trait example:");
    call_and_modify(increment_counter);
    
    // FnOnce: can only be called once, takes ownership of captured variables
    fn call_once<F, T>(f: F) -> T
    where
        F: FnOnce() -> T,
    {
        f()
    }
    
    let owned_data = vec![1, 2, 3, 4, 5];
    let consume_data = move || {
        println!("Processing data: {:?}", owned_data);
        owned_data.len() // Return the length, consuming the vector
    };
    
    println!("\nFnOnce trait example:");
    let length = call_once(consume_data);
    println!("Data length was: {}", length);
    // consume_data(); // This would cause a compile error - already consumed!
}

fn storing_closures() {
    // Storing closures in variables
    let add = |a, b| a + b;
    let subtract = |a, b| a - b;
    
    // Using Box<dyn Fn> to store different closures
    let operations: Vec<Box<dyn Fn(i32, i32) -> i32>> = vec![
        Box::new(add),
        Box::new(subtract),
        Box::new(|a, b| a * b), // multiply
        Box::new(|a, b| if b != 0 { a / b } else { 0 }), // divide
    ];
    
    let x = 10;
    let y = 3;
    
    println!("Operations on {} and {}:", x, y);
    for (i, op) in operations.iter().enumerate() {
        let result = op(x, y);
        let op_name = match i {
            0 => "add",
            1 => "subtract", 
            2 => "multiply",
            3 => "divide",
            _ => "unknown",
        };
        println!("  {}: {}", op_name, result);
    }
    
    // Storing closures in structs
    struct Calculator<F>
    where
        F: Fn(f64, f64) -> f64,
    {
        operation: F,
        name: String,
    }
    
    impl<F> Calculator<F>
    where
        F: Fn(f64, f64) -> f64,
    {
        fn new(operation: F, name: String) -> Self {
            Calculator { operation, name }
        }
        
        fn calculate(&self, a: f64, b: f64) -> f64 {
            (self.operation)(a, b)
        }
        
        fn name(&self) -> &str {
            &self.name
        }
    }
    
    let power_calc = Calculator::new(|a, b| a.powf(b), "Power".to_string());
    let log_calc = Calculator::new(|a, _| a.ln(), "Natural Log".to_string());
    
    println!("\nCalculator examples:");
    println!("{}: {} ^ {} = {}", power_calc.name(), 2.0, 3.0, power_calc.calculate(2.0, 3.0));
    println!("{}: ln({}) = {}", log_calc.name(), 2.718, log_calc.calculate(2.718, 0.0));
}

fn closures_with_iterators() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Using closures with iterator methods
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", evens);
    
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);
    
    // Chaining iterator operations with closures
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&x| x % 2 == 1)  // Keep odd numbers
        .map(|x| x * x)           // Square them
        .filter(|&x| x > 10)      // Keep only those > 10
        .collect();
    
    println!("Odd numbers squared and > 10: {:?}", result);
    
    // Using closures that capture environment with iterators
    let threshold = 5;
    let above_threshold: Vec<&i32> = numbers
        .iter()
        .filter(|&x| *x > threshold)
        .collect();
    
    println!("Numbers above {}: {:?}", threshold, above_threshold);
    
    // More complex example: grouping with closures
    let words = vec!["apple", "banana", "cherry", "date", "elderberry"];
    let mut groups: HashMap<usize, Vec<&str>> = HashMap::new();
    
    for word in &words {
        let len = word.len();
        groups.entry(len).or_insert_with(Vec::new).push(word);
    }
    
    println!("Words grouped by length:");
    for (len, words) in &groups {
        println!("  Length {}: {:?}", len, words);
    }
}

fn practical_examples() {
    // Example 1: Memoization with closures
    fn memoize<F, K, V>(mut f: F) -> impl FnMut(K) -> V
    where
        F: FnMut(K) -> V,
        K: Clone + std::hash::Hash + Eq,
        V: Clone,
    {
        let mut cache: HashMap<K, V> = HashMap::new();
        move |key: K| {
            if let Some(value) = cache.get(&key) {
                value.clone()
            } else {
                let value = f(key.clone());
                cache.insert(key, value.clone());
                value
            }
        }
    }
    
    let expensive_function = |n: i32| {
        println!("Computing expensive function for {}", n);
        thread::sleep(Duration::from_millis(100)); // Simulate expensive computation
        n * n * n // Cube the number
    };
    
    let mut memoized = memoize(expensive_function);
    
    println!("\nMemoization example:");
    println!("First call f(5): {}", memoized(5));  // Will compute
    println!("Second call f(5): {}", memoized(5)); // Will use cache
    println!("First call f(3): {}", memoized(3));  // Will compute
    println!("Second call f(3): {}", memoized(3)); // Will use cache
    
    // Example 2: Event handling with closures
    struct Button {
        label: String,
        on_click: Box<dyn FnMut()>,
    }
    
    impl Button {
        fn new<F>(label: String, on_click: F) -> Self
        where
            F: FnMut() + 'static,
        {
            Button {
                label,
                on_click: Box::new(on_click),
            }
        }
        
        fn click(&mut self) {
            println!("Button '{}' clicked!", self.label);
            (self.on_click)();
        }
    }
    
    let mut click_count = 0;
    let mut button = Button::new(
        "Counter".to_string(),
        move || {
            click_count += 1;
            println!("  Click count: {}", click_count);
        }
    );
    
    println!("\nEvent handling example:");
    button.click();
    button.click();
    button.click();
    
    // Example 3: Configuration with closures
    struct Config {
        debug: bool,
        max_connections: usize,
    }
    
    fn create_logger(config: Config) -> impl Fn(&str) {
        move |message: &str| {
            if config.debug {
                println!("[DEBUG] {}", message);
            }
        }
    }
    
    fn create_connection_manager(config: Config) -> impl FnMut() -> bool {
        let mut current_connections = 0;
        move || {
            if current_connections < config.max_connections {
                current_connections += 1;
                println!("Connection accepted. Total: {}/{}", current_connections, config.max_connections);
                true
            } else {
                println!("Connection rejected. At max capacity: {}", config.max_connections);
                false
            }
        }
    }
    
    let config = Config {
        debug: true,
        max_connections: 3,
    };
    
    let logger = create_logger(Config { debug: true, max_connections: 0 });
    let mut conn_manager = create_connection_manager(config);
    
    println!("\nConfiguration example:");
    logger("System starting up");
    conn_manager();
    conn_manager();
    conn_manager();
    conn_manager(); // This should be rejected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_closure() {
        let add_one = |x| x + 1;
        assert_eq!(add_one(5), 6);
    }

    #[test]
    fn test_capturing_by_reference() {
        let x = 10;
        let capture_x = || x;
        assert_eq!(capture_x(), 10);
        assert_eq!(x, 10); // x is still accessible
    }

    #[test]
    fn test_capturing_by_move() {
        let x = String::from("hello");
        let capture_x = move || x;
        assert_eq!(capture_x(), "hello");
        // x is no longer accessible here
    }

    #[test]
    fn test_fn_trait() {
        fn call_twice<F>(f: F) -> (i32, i32)
        where
            F: Fn(i32) -> i32,
        {
            (f(1), f(2))
        }
        
        let double = |x| x * 2;
        assert_eq!(call_twice(double), (2, 4));
    }

    #[test]
    fn test_fnmut_trait() {
        fn call_and_accumulate<F>(mut f: F) -> Vec<i32>
        where
            F: FnMut() -> i32,
        {
            vec![f(), f(), f()]
        }
        
        let mut counter = 0;
        let increment = || {
            counter += 1;
            counter
        };
        
        assert_eq!(call_and_accumulate(increment), vec![1, 2, 3]);
    }

    #[test]
    fn test_closures_with_iterators() {
        let numbers = vec![1, 2, 3, 4, 5];
        let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
        
        let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();
        assert_eq!(evens, vec![&2, &4]);
    }
}