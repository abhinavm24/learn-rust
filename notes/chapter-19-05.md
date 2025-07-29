# Chapter 19.5: Macros

## Key Takeaways
- **Declarative Macros**: Pattern-based code generation with `macro_rules!`
- **Procedural Macros**: Rust code that operates on syntax trees
- **Three Types**: Custom derive, attribute-like, and function-like procedural macros
- **Compile-Time**: Macros expand during compilation, not runtime
- **Metaprogramming**: Write code that writes code for reducing repetition

## Understanding Macros vs Functions

### Macros vs Functions Comparison

```rust
// Function: operates on values at runtime
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Macro: operates on code at compile time
macro_rules! add_macro {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

fn main() {
    // Function call
    let result1 = add(2, 3);
    
    // Macro invocation - expands to: 2 + 3
    let result2 = add_macro!(2, 3);
    
    println!("Function: {}, Macro: {}", result1, result2);
}
```

### Why Use Macros

```rust
// ❌ Function can't take variable number of arguments
fn print_multiple(values: &[i32]) {
    for value in values {
        println!("{}", value);
    }
}

// ✅ Macro can take variable number of arguments
macro_rules! print_multiple {
    ($($x:expr),*) => {
        $(
            println!("{}", $x);
        )*
    };
}

fn main() {
    // Function requires creating array
    print_multiple(&[1, 2, 3, 4]);
    
    // Macro can take any number of arguments directly
    print_multiple!(1, 2, 3, 4, 5, 6);
}
```

## Declarative Macros with macro_rules!

### Basic Syntax

```rust
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {}()", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

fn main() {
    say_hello!();
    foo();
    bar();
}
```

### Pattern Matching in Macros

```rust
macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e; // Force types to be integers
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

fn main() {
    calculate!(eval 1 + 2);
    calculate!(eval (1 + 2) * 3, eval 4 * 5, eval 6 + 7);
}
```

### Creating a vec! Macro Clone

```rust
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

fn main() {
    let v1: Vec<i32> = my_vec![];
    let v2 = my_vec![1, 2, 3, 4];
    let v3 = my_vec![0; 5];
    let v4 = my_vec![1, 2, 3,]; // Trailing comma allowed
    
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("v3: {:?}", v3);
    println!("v4: {:?}", v4);
}
```

### Advanced Pattern Matching

```rust
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

use std::collections::HashMap;

fn main() {
    let empty: HashMap<i32, String> = hash_map!();
    
    let scores = hash_map! {
        "Alice" => 95,
        "Bob" => 87,
        "Charlie" => 92,
    };
    
    println!("Empty: {:?}", empty);
    println!("Scores: {:?}", scores);
}
```

### Macro Pattern Types

```rust
macro_rules! test_patterns {
    // item: an item, like a function, struct, module, etc.
    ($item:item) => {
        $item
    };
    
    // block: a block expression
    ($block:block) => {
        $block
    };
    
    // stmt: a statement
    ($stmt:stmt) => {
        $stmt
    };
    
    // pat: a pattern
    ($pat:pat) => {
        match 42 {
            $pat => println!("Pattern matched!"),
            _ => println!("No match"),
        }
    };
    
    // expr: an expression
    ($expr:expr) => {
        println!("Expression result: {}", $expr);
    };
    
    // ty: a type
    ($ty:ty) => {
        {
            let _var: $ty = Default::default();
            println!("Created variable of type: {}", stringify!($ty));
        }
    };
    
    // ident: an identifier
    ($ident:ident) => {
        let $ident = "identifier";
        println!("{} = {}", stringify!($ident), $ident);
    };
    
    // path: a path (e.g., foo, ::std::mem::replace, transmute::<_, int>)
    ($path:path) => {
        println!("Path: {}", stringify!($path));
    };
    
    // meta: a meta item
    ($meta:meta) => {
        println!("Meta: {}", stringify!($meta));
    };
    
    // tt: a single token tree
    ($tt:tt) => {
        println!("Token tree: {}", stringify!($tt));
    };
}

// Using different pattern types
test_patterns!(fn example() { println!("Item pattern"); });

fn main() {
    test_patterns!({ println!("Block pattern"); });
    test_patterns!(let x = 5);
    test_patterns!(42);
    test_patterns!(Some(x));
    test_patterns!(String);
    test_patterns!(my_var);
    test_patterns!(std::collections::HashMap);
    test_patterns!(derive(Debug));
    test_patterns!(hello);
}
```

## Procedural Macros

### Setting Up a Procedural Macro Crate

```toml
# Cargo.toml
[package]
name = "my_macro"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
syn = "2.0"
quote = "1.0"
proc-macro2 = "1.0"
```

### Custom Derive Macros

```rust
// lib.rs - The procedural macro crate
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    
    let expanded = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    
    TokenStream::from(expanded)
}

// Usage in another crate
trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Waffles;

fn main() {
    Pancakes::hello_macro();
    Waffles::hello_macro();
}
```

### Attribute-like Macros

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn show_streams(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("args: \"{}\"", args.to_string());
    println!("input: \"{}\"", input.to_string());
    input
}

#[proc_macro_attribute]
pub fn timing(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    
    let expanded = quote! {
        #fn_vis #fn_sig {
            let start = std::time::Instant::now();
            let result = (|| #fn_body)();
            let duration = start.elapsed();
            println!("Function {} took: {:?}", stringify!(#fn_name), duration);
            result
        }
    };
    
    TokenStream::from(expanded)
}

// Usage
#[show_streams(foo)]
fn invoke1() {}

#[timing]
fn slow_function() -> i32 {
    std::thread::sleep(std::time::Duration::from_millis(100));
    42
}

fn main() {
    invoke1();
    let result = slow_function();
    println!("Result: {}", result);
}
```

### Function-like Macros

```rust
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let sql_query = input.to_string();
    
    // In a real implementation, you'd parse the SQL
    // For this example, we'll just create a simple wrapper
    let expanded = quote! {
        {
            let query = #sql_query;
            println!("Executing SQL: {}", query);
            // Return a mock result
            "SQL executed successfully"
        }
    };
    
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

// Usage
fn main() {
    let result = sql!(SELECT * FROM users WHERE id = 1);
    println!("Result: {}", result);
    
    make_answer!();
    println!("The answer is: {}", answer());
}
```

## Advanced Macro Techniques

### Recursive Macros

```rust
macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32, 2u32, 3u32));
    println!("{}", find_min!(5u32, 2u32, 1u32, 3u32));
}
```

### Generating Repeated Code

```rust
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

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    
    println!("p + 3.0 = {:?}", p + 3.0);
    println!("p - 1.0 = {:?}", p - 1.0);
    println!("p * 2.0 = {:?}", p * 2.0);
    println!("p / 2.0 = {:?}", p / 2.0);
}
```

### DSL (Domain Specific Language) Creation

```rust
macro_rules! html {
    // Text content
    ($text:literal) => {
        format!("{}", $text)
    };
    
    // Self-closing tag
    ($tag:ident) => {
        format!("<{} />", stringify!($tag))
    };
    
    // Tag with content
    ($tag:ident { $($content:tt)* }) => {
        format!("<{}>{}</{}>", 
                stringify!($tag), 
                html!($($content)*), 
                stringify!($tag))
    };
    
    // Multiple elements
    ($($element:tt)*) => {
        {
            let mut result = String::new();
            $(
                result.push_str(&html!($element));
            )*
            result
        }
    };
}

fn main() {
    let document = html! {
        html {
            head {
                title { "My Page" }
            }
            body {
                h1 { "Welcome" }
                p { "This is a paragraph." }
                br
                p { "Another paragraph." }
            }
        }
    };
    
    println!("{}", document);
}
```

### Debugging Macros

```rust
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

fn main() {
    debug_print!("Hello, {}!", "world");
    
    let x = 5;
    let y = trace!(x + 3);
    println!("y = {}", y);
}
```

## Macro Hygiene and Scoping

### Variable Hygiene

```rust
macro_rules! using_a {
    ($e:expr) => {
        {
            let a = 42;  // This 'a' is hygienic
            println!("Macro a: {}", a);
            $e
        }
    }
}

fn main() {
    let a = 13;
    
    let result = using_a!(
        {
            println!("User a: {}", a);  // This refers to outer 'a'
            a * 2
        }
    );
    
    println!("Result: {}", result);
    println!("Original a: {}", a);
}
```

### Breaking Hygiene When Needed

```rust
macro_rules! declare_var {
    ($name:ident, $value:expr) => {
        let $name = $value;
    };
}

fn main() {
    declare_var!(x, 42);
    println!("x = {}", x);  // This works - macro introduces 'x' into scope
}
```

## Real-World Macro Examples

### Builder Pattern Generator

```rust
macro_rules! builder {
    (
        struct $name:ident {
            $(
                $field:ident: $field_type:ty
            ),* $(,)?
        }
    ) => {
        #[derive(Default)]
        struct $name {
            $(
                $field: Option<$field_type>,
            )*
        }
        
        paste::paste! {
            impl $name {
                fn new() -> Self {
                    Self::default()
                }
                
                $(
                    fn $field(mut self, value: $field_type) -> Self {
                        self.$field = Some(value);
                        self
                    }
                )*
                
                fn build(self) -> Result<[<$name Built>], String> {
                    Ok([<$name Built>] {
                        $(
                            $field: self.$field.ok_or(concat!("Missing field: ", stringify!($field)))?,
                        )*
                    })
                }
            }
            
            struct [<$name Built>] {
                $(
                    $field: $field_type,
                )*
            }
        }
    };
}

// Note: This example requires the 'paste' crate for identifier concatenation
// In a real implementation, you'd use proc macros for this

macro_rules! simple_builder {
    (
        struct $name:ident {
            $($field:ident: $field_type:ty),* $(,)?
        }
    ) => {
        struct $name {
            $(
                $field: $field_type,
            )*
        }
        
        impl $name {
            $(
                fn $field(mut self, value: $field_type) -> Self {
                    self.$field = value;
                    self
                }
            )*
        }
    };
}

simple_builder! {
    struct User {
        name: String,
        age: u32,
        email: String,
    }
}

fn main() {
    let user = User {
        name: String::from(""),
        age: 0,
        email: String::from(""),
    }
    .name("Alice".to_string())
    .age(30)
    .email("alice@example.com".to_string());
    
    println!("User: {} ({}), {}", user.name, user.age, user.email);
}
```

### Testing Macro

```rust
macro_rules! test_cases {
    (
        $test_name:ident: $func:ident {
            $(
                $input:expr => $expected:expr
            ),* $(,)?
        }
    ) => {
        #[cfg(test)]
        mod $test_name {
            use super::*;
            
            $(
                #[test]
                fn test() {
                    assert_eq!($func($input), $expected);
                }
            )*
        }
    };
}

fn double(x: i32) -> i32 {
    x * 2
}

fn add_one(x: i32) -> i32 {
    x + 1
}

// Generate test cases
test_cases! {
    double_tests: double {
        0 => 0,
        1 => 2,
        5 => 10,
        -3 => -6,
    }
}

// Note: This macro would generate multiple tests with the same name
// A real implementation would use proc macros or paste to generate unique names

fn main() {
    println!("Tests are defined - run with 'cargo test'");
}
```

## Best Practices and Common Pitfalls

### Macro Best Practices

```rust
// ✅ Good: Clear naming and documentation
/// Creates a vector with debug output
macro_rules! debug_vec {
    ($($x:expr),*) => {
        {
            let vec = vec![$($x),*];
            println!("Created vector: {:?}", vec);
            vec
        }
    };
}

// ✅ Good: Handle edge cases
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

// ❌ Bad: Multiple evaluation of arguments
macro_rules! bad_max {
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }  // $a and $b evaluated multiple times
    };
}

// ✅ Good: Evaluate arguments once
macro_rules! good_max {
    ($a:expr, $b:expr) => {
        {
            let a_val = $a;
            let b_val = $b;
            if a_val > b_val { a_val } else { b_val }
        }
    };
}

fn main() {
    let v = debug_vec![1, 2, 3];
    println!("{:?}", v);
    
    println!("Division: {:?}", safe_divide!(10, 2));
    println!("Division by zero: {:?}", safe_divide!(10, 0));
    
    let mut x = 1;
    
    // This would increment x multiple times with bad_max!
    println!("Max: {}", good_max!(x += 1; x, 5));
    println!("x after macro: {}", x);  // x is only incremented once
}
```

Macros are powerful tools for metaprogramming in Rust, enabling code generation, DSL creation, and reducing boilerplate while maintaining compile-time safety and performance.