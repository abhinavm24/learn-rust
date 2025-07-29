# Chapter 3: Common Programming Concepts

## Key Takeaways

### Variables and Mutability
- **Immutable by Default**: Variables are immutable unless explicitly marked with `mut`
- **Constants**: Global constants declared with `const` keyword
- **Shadowing**: Declaring new variables with same name in same scope
- **Memory Safety**: Rust prevents data races through ownership system

### Data Types
- **Scalar Types**: integers, floating-point, booleans, characters
- **Compound Types**: tuples and arrays
- **Type Inference**: Rust can infer types but sometimes needs annotation
- **Type Safety**: Strong static typing prevents many runtime errors

### Functions
- **fn Keyword**: All functions declared with `fn`
- **Parameters vs Arguments**: Functions have parameters, calls pass arguments
- **Return Values**: Functions can return values, expressions vs statements
- **Snake Case**: Rust convention for function and variable names

### Control Flow
- **if Expressions**: Conditionals that can return values
- **Loops**: `loop`, `while`, and `for` constructs
- **Pattern Matching**: Early introduction to Rust's powerful matching
- **Expression-Oriented**: Most constructs are expressions, not statements

## Chapter Structure

### 3.1: Variables and Mutability
```rust
let x = 5;           // Immutable
let mut y = 5;       // Mutable
const MAX_POINTS: u32 = 100_000;  // Constant

// Shadowing
let x = x + 1;       // New variable, same name
let x = x * 2;       // Another new variable
```

### 3.2: Data Types
```rust
// Scalar types
let guess: u32 = "42".parse().expect("Not a number!");
let x = 2.0;         // f64 by default
let y: f32 = 3.0;    // f32 explicitly
let t = true;        // bool
let c = 'z';         // char

// Compound types
let tup: (i32, f64, u8) = (500, 6.4, 1);
let a = [1, 2, 3, 4, 5];  // Array
```

### 3.3: Functions
```rust
fn main() {
    println!("Hello, world!");
    another_function(5);
    let x = five();
    let y = plus_one(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5  // Expression, no semicolon
}

fn plus_one(x: i32) -> i32 {
    x + 1  // Expression returns value
}
```

### 3.4: Comments
```rust
// This is a single-line comment

/* This is a multi-line comment
   spanning multiple lines */

/// Documentation comment for the following item
fn documented_function() {
    //! Inner documentation comment
}
```

### 3.5: Control Flow
```rust
// if expressions
let number = 6;
if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else {
    println!("number is not divisible by 4 or 3");
}

// Using if in a let statement
let condition = true;
let number = if condition { 5 } else { 6 };

// Loops
loop {
    println!("again!");
    break;  // Exit the loop
}

let mut number = 3;
while number != 0 {
    println!("{}!", number);
    number -= 1;
}

let a = [10, 20, 30, 40, 50];
for element in a.iter() {
    println!("the value is: {}", element);
}

for number in (1..4).rev() {
    println!("{}!", number);
}
```

## Learning Progression

### Building Blocks
Chapter 3 introduces the fundamental building blocks that every Rust program uses:
- How to declare and use variables safely
- Rust's type system and how it prevents errors
- Function syntax and return values
- Basic control flow structures

### Key Concepts for Future Chapters
- **Ownership Preview**: Immutability by default sets up ownership concepts
- **Type Safety**: Strong typing foundation for more complex types
- **Expression vs Statement**: Critical for understanding Rust syntax
- **Pattern Matching**: Basic patterns prepare for advanced matching

### Practical Applications
```rust
// Temperature conversion function
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// Fibonacci sequence
fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// Array processing
fn sum_array(arr: [i32; 5]) -> i32 {
    let mut sum = 0;
    for element in arr.iter() {
        sum += element;
    }
    sum
}
```

## Integration with Previous Chapters
- **Chapter 1**: Sets up the development environment used here
- **Chapter 2**: Guessing game used these concepts in practice
- **Future Chapters**: Every subsequent chapter builds on these fundamentals

## Common Patterns Introduced
- **Variable Shadowing**: Transforming values while keeping same name
- **Expression-Based Logic**: Using if/match as expressions
- **Iterator Patterns**: for loops with .iter() and ranges
- **Function Design**: Pure functions that take input and return output

## Memory and Performance Notes
- **Stack Allocation**: All basic types in this chapter are stack-allocated
- **Copy Semantics**: Scalar types implement Copy trait
- **No Garbage Collection**: Manual memory management concepts introduced
- **Zero-Cost Abstractions**: Control flow compiles to efficient machine code

## Best Practices Established
- Use `let` by default, `let mut` only when mutation needed
- Prefer expressions over statements when possible
- Use descriptive function and variable names
- Handle all branches in conditional logic
- Use appropriate loop constructs for different scenarios

## Debugging and Testing
- **println! Macro**: Basic debugging output
- **panic! Behavior**: What happens when programs encounter errors
- **Compile-Time Checks**: Type system catches errors before runtime
- **Testing Functions**: All code examples can be tested with unit tests

Official Chapter: https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html

---
*Completed: ✓*