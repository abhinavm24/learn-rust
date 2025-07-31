//! Chapter 19.5: Macros
//! 
//! This example demonstrates macro programming in Rust:
//! - Declarative macros with `macro_rules!`
//! - Pattern matching in macros
//! - Variable argument macros
//! - Creating DSLs (Domain Specific Languages)
//! - Macro hygiene and scoping
//! - Best practices and common pitfalls

use rust_book_examples::print_chapter_header;
use std::collections::HashMap;

// === MACRO DEFINITIONS (must come before usage) ===

// Basic macros
macro_rules! say_hello {
    () => {
        println!("Hello from macro!");
    };
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {}()", stringify!($func_name));
        }
    };
}

macro_rules! add_macro {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

// Pattern matching macros
macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: i32 = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    };
    
    (eval $e:expr, $(eval $es:expr),+) => {
        {
            calculate!(eval $e);
            calculate!($(eval $es),+);
        }
    };
}

macro_rules! test_patterns {
    // expr: an expression
    ($expr:expr) => {
        println!("Expression result: {}", $expr);
    };
    
    // ident: an identifier  
    ($ident:ident) => {
        let $ident = "identifier value";
        println!("{} = {}", stringify!($ident), $ident);
    };
    
    // ty: a type
    ($ty:ty) => {
        {
            let _var: $ty = Default::default();
            println!("Created variable of type: {}", stringify!($ty));
        }
    };
    
    // pat: a pattern
    ($pat:pat) => {
        match Some(42) {
            $pat => println!("Pattern {} matched!", stringify!($pat)),
            _ => println!("Pattern {} didn't match", stringify!($pat)),
        }
    };
    
    // block: a block expression
    ($block:block) => {
        println!("Executing block:");
        $block
    };
    
    // path: a path
    ($path:path) => {
        println!("Path: {}", stringify!($path));
    };
}

// Variadic macros
macro_rules! my_vec {
    // Empty vector
    () => {
        Vec::new()
    };
    
    // Single element, repeated
    ($elem:expr; $n:expr) => {
        {
            let mut temp_vec = Vec::new();
            temp_vec.resize($n, $elem);
            temp_vec
        }
    };
    
    // Multiple elements
    ($($x:expr),+ $(,)?) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )+
            temp_vec
        }
    };
}

macro_rules! print_multiple {
    ($($x:expr),*) => {
        $(
            println!("Value: {}", $x);
        )*
    };
}

macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

// Advanced pattern macros
macro_rules! hash_map {
    // Empty map
    () => {
        std::collections::HashMap::new()
    };
    
    // With key-value pairs
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )+
            map
        }
    };
}

// Macro for implementing multiple operators
macro_rules! impl_ops {
    ($($op:ident, $method:ident, $symbol:tt);+) => {
        $(
            impl std::ops::$op<f64> for Point {
                type Output = Point;
                
                fn $method(self, rhs: f64) -> Point {
                    Point {
                        x: self.x $symbol rhs,
                        y: self.y $symbol rhs,
                    }
                }
            }
        )+
    };
}

// DSL macros - simplified to avoid recursion issues
macro_rules! html {
    // Text content
    ($text:literal) => {
        format!("{}", $text)
    };
    
    // Self-closing tag
    ($tag:ident) => {
        format!("<{} />", stringify!($tag))
    };
    
    // Tag with text content
    ($tag:ident { $text:literal }) => {
        format!("<{}>{}</{}>", stringify!($tag), $text, stringify!($tag))
    };
}

macro_rules! json_like {
    // Base case: empty object
    () => {
        String::from("{}")
    };
    
    // Key-value pairs
    ($($key:literal: $value:expr),+ $(,)?) => {
        {
            let mut result = String::from("{");
            let mut first = true;
            $(
                if !first {
                    result.push_str(", ");
                }
                first = false;
                result.push_str(&format!("\"{}\": {}", $key, 
                    match stringify!($value) {
                        s if s.starts_with('"') => s.to_string(),
                        _ => format!("{}", $value),
                    }
                ));
            )+
            result.push('}');
            result
        }
    };
}

// Hygiene macros
macro_rules! using_a {
    ($e:expr) => {
        {
            let a = 42;  // This 'a' is hygienic - doesn't conflict with outer 'a'
            println!("Macro's 'a': {}", a);
            $e
        }
    }
}

macro_rules! declare_var {
    ($name:ident, $value:expr) => {
        let $name = $value;
    };
}

// Best practices macros
macro_rules! debug_vec {
    ($($x:expr),*) => {
        {
            let vec = vec![$($x),*];
            println!("Created debug vector with {} elements", vec.len());
            vec
        }
    };
}

macro_rules! safe_divide {
    ($a:expr, $b:expr) => {
        {
            let denominator = $b;
            if denominator == 0 {
                None
            } else {
                Some($a / denominator)
            }
        }
    };
}

macro_rules! good_max {
    ($a:expr, $b:expr) => {
        {
            let a_val = $a;
            let b_val = $b;
            if a_val > b_val { a_val } else { b_val }
        }
    };
}

macro_rules! debug_print {
    ($($arg:tt)*) => {
        {
            println!("Debug: {}", format!($($arg)*));
            println!("File: {}, Line: {}", file!(), line!());
        }
    };
}

macro_rules! trace {
    ($expr:expr) => {
        {
            let result = $expr;
            println!("TRACE: {} = {:?}", stringify!($expr), result);
            result
        }
    };
}

// === STRUCT DEFINITIONS ===

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl_ops! {
    Add, add, +;
    Sub, sub, -;
    Mul, mul, *;
    Div, div, /
}

// Generate functions via macro
create_function!(foo);
create_function!(bar);

fn main() {
    print_chapter_header("Chapter 19.5", "Macros");
    
    basic_macros_demo();
    pattern_matching_demo();
    variadic_macros_demo();
    advanced_patterns_demo();
    dsl_creation_demo();
    hygiene_demo();
    best_practices_demo();
}

/// Demonstrates basic macro syntax and usage
fn basic_macros_demo() {
    println!("\n=== Basic Macros ===");
    
    // Simple macro without parameters
    say_hello!();
    
    // Macro that creates functions
    foo();
    bar();
    
    // Macro vs function comparison
    println!("\n--- Macro vs Function ---");
    let result1 = add_function(2, 3);
    let result2 = add_macro!(2, 3);
    println!("Function result: {}, Macro result: {}", result1, result2);
}

fn add_function(a: i32, b: i32) -> i32 {
    a + b
}

/// Demonstrates pattern matching in macros
fn pattern_matching_demo() {
    println!("\n=== Pattern Matching in Macros ===");
    
    calculate!(eval 1 + 2);
    calculate!(eval (1 + 2) * 3, eval 4 * 5, eval 6 + 7);
    
    // Testing different pattern types  
    println!("\n--- Different Pattern Types ---");
    test_patterns!(42);                           // expr pattern
    
    // Demonstrate stringify with ident patterns
    println!("Identifier: {}", stringify!(my_variable));
    println!("Type: {}", stringify!(String));
    println!("Path: {}", stringify!(std::collections::HashMap));
    
    // Block pattern demonstration  
    println!("Executing block:");
    { println!("Block executed directly!"); }
}

/// Demonstrates variadic macros and repetition patterns
fn variadic_macros_demo() {
    println!("\n=== Variadic Macros ===");
    
    // Clone of vec! macro
    let v1: Vec<i32> = my_vec![];
    let v2 = my_vec![1, 2, 3, 4];
    let v3 = my_vec![0; 5];
    let v4 = my_vec![1, 2, 3,]; // Trailing comma allowed
    
    println!("Empty vector: {:?}", v1);
    println!("List vector: {:?}", v2);
    println!("Repeat vector: {:?}", v3);
    println!("Trailing comma: {:?}", v4);
    
    // Variable arguments
    println!("\n--- Variable Arguments ---");
    print_multiple!(1, 2, 3, 4, 5, 6);
    
    // Finding minimum
    println!("\n--- Recursive Macros ---");
    println!("Min of 1: {}", find_min!(1u32));
    println!("Min of 1,2,3: {}", find_min!(1u32, 2u32, 3u32));
    println!("Min of 5,2,1,3: {}", find_min!(5u32, 2u32, 1u32, 3u32));
}

/// Demonstrates advanced macro patterns
fn advanced_patterns_demo() {
    println!("\n=== Advanced Patterns ===");
    
    // HashMap creation macro
    let empty: HashMap<i32, String> = hash_map!();
    
    let scores = hash_map! {
        "Alice" => 95,
        "Bob" => 87,
        "Charlie" => 92,
    };
    
    println!("Empty map: {:?}", empty);
    println!("Scores: {:?}", scores);
    
    // Code generation for traits
    println!("\n--- Code Generation ---");
    let p = Point { x: 1.0, y: 2.0 };
    
    println!("Original point: {:?}", p);
    println!("p + 3.0 = {:?}", p + 3.0);
    println!("p - 1.0 = {:?}", p - 1.0);
    println!("p * 2.0 = {:?}", p * 2.0);
    println!("p / 2.0 = {:?}", p / 2.0);
}

/// Demonstrates creating a simple DSL with macros
fn dsl_creation_demo() {
    println!("\n=== DSL Creation ===");
    
    // Demonstrate simple HTML generation
    let title = html!(title { "My Rust Page" });
    let heading = html!(h1 { "Welcome to Rust Macros!" });
    let paragraph = html!(p { "This HTML was generated by a Rust macro." });
    let break_tag = html!(br);
    
    let document = format!("<html><head>{}</head><body>{}{}{}</body></html>", 
                          title, heading, paragraph, break_tag);
    
    println!("Generated HTML:");
    println!("{}", document);
    
    // JSON-like syntax
    println!("\n--- JSON-like DSL ---");
    let config = json_like! {
        "name": "MyApp",
        "version": "1.0.0",
        "debug": true,
        "max_connections": 100
    };
    
    println!("Config: {}", config);
}

/// Demonstrates macro hygiene and scoping
fn hygiene_demo() {
    println!("\n=== Macro Hygiene ===");
    
    let a = 13;
    
    let result = using_a!(
        {
            println!("User's 'a': {}", a);  // This refers to outer 'a'
            a * 2
        }
    );
    
    println!("Result: {}", result);
    println!("Original 'a': {}", a);
    
    // Variable declaration in macro
    println!("\n--- Variable Declaration ---");
    declare_var!(x, 42);
    println!("Declared x = {}", x);  // This works - macro introduces 'x' into scope
    
    declare_var!(message, "Hello from macro variable!");
    println!("{}", message);
}

/// Demonstrates best practices and common pitfalls
fn best_practices_demo() {
    println!("\n=== Best Practices ===");
    
    // Good: Clear naming and documentation
    let v = debug_vec![1, 2, 3, 4];
    println!("Created vector: {:?}", v);
    
    // Good: Handle edge cases
    println!("Division: {:?}", safe_divide!(10, 2));
    println!("Division by zero: {:?}", safe_divide!(10, 0));
    
    // Good: Evaluate arguments once to avoid side effects
    let mut x = 1;
    println!("x before macro: {}", x);
    
    // This would increment x multiple times with bad_max!
    let result = good_max!({x += 1; x}, 5);
    println!("Max result: {}", result);
    println!("x after macro: {}", x);  // x is only incremented once
    
    // Debugging utilities
    println!("\n--- Debugging Utilities ---");
    debug_print!("Hello, {}!", "world");
    
    let y = trace!(x + 3);
    println!("Final y = {}", y);
}