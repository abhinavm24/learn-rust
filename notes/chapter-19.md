# Chapter 19: Advanced Features

## Key Takeaways

### Core Concepts
- **Unsafe Rust**: Breaking out of Rust's safety guarantees for performance and interoperability
- **Advanced Trait Patterns**: Associated types, default parameters, supertraits, and orphan rules
- **Advanced Type System Features**: Type aliases, never type, DSTs, and function pointers
- **Macro Programming**: Code generation through declarative and procedural macros
- **Foreign Function Interface**: Interacting with code written in other languages
- **Performance Optimization**: Zero-cost abstractions and compile-time code generation

### Important Syntax and Operators
- `unsafe { }` - Block containing potentially unsafe operations
- `*const T` and `*mut T` - Raw pointer types for manual memory management
- `type Alias = ActualType` - Create type synonyms for complex types
- `!` - Never type for functions that never return
- `dyn Trait` - Trait objects for dynamic dispatch
- `macro_rules!` - Declarative macro definition
- `#[derive(Trait)]` - Procedural macro for automatic trait implementation
- `extern "C"` - Foreign function interface declarations

### Programming Concepts Introduced
- **Memory Safety vs Performance Trade-offs**: When and how to use unsafe code
- **Metaprogramming**: Writing code that generates other code
- **Foreign Function Interface (FFI)**: Language interoperability mechanisms
- **Advanced Type Theory**: Sophisticated type system features for library design
- **Compile-time Code Generation**: Reducing runtime overhead through macros

## Code Examples and Patterns

### Unsafe Rust Fundamentals
```rust
fn main() {
    let mut num = 5;
    
    // Creating raw pointers (safe)
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    // Dereferencing raw pointers (unsafe)
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        
        // Modifying through mutable raw pointer
        *r2 = 10;
        println!("num is now: {}", num);
    }
}
```

### Advanced Trait Patterns with Associated Types
```rust
// Associated types for cleaner API design
trait Iterator {
    type Item;  // Associated type
    
    fn next(&mut self) -> Option<Self::Item>;
}

// Implementation with concrete associated type
struct Counter {
    current: usize,
    max: usize,
}

impl Iterator for Counter {
    type Item = usize;  // Concrete type for associated type
    
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

// Default generic type parameters
trait Add<Rhs = Self> {  // Default to Self
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

// Supertraits - requiring other trait implementations
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();  // Can use Display's to_string
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

### Advanced Type System Features
```rust
// Type aliases for complex types
type Thunk = Box<dyn Fn() + Send + 'static>;
type Result<T> = std::result::Result<T, std::io::Error>;

// Function pointers
fn add_one(x: i32) -> i32 { x + 1 }
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);  // Output: 12
}

// Never type usage
fn bar() -> ! {
    panic!("This function never returns!");
}

// Using never type in match expressions
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,  // continue has type !
};
```

### Declarative Macros
```rust
// Custom vec! macro implementation
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// More complex macro with different patterns
macro_rules! hash_map {
    ($( $key:expr => $val:expr ),*) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

fn main() {
    let v = my_vec![1, 2, 3];
    let map = hash_map!["one" => 1, "two" => 2];
}
```

### Foreign Function Interface (FFI)
```rust
extern "C" {
    fn abs(input: i32) -> i32;  // Declare C function
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// Calling Rust from other languages
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Safe wrapper around unsafe operations
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

## Practical Applications
- Building high-performance libraries that need to optimize critical paths
- Creating safe abstractions around unsafe system calls or C libraries
- Implementing data structures that require manual memory management
- Writing procedural macros to reduce boilerplate code
- Building domain-specific languages embedded in Rust
- Creating zero-cost abstractions for performance-critical applications

## Integration with Previous Chapters
- **Prerequisites**: Deep understanding of ownership (Chapter 4), traits (Chapter 10), and smart pointers (Chapter 15)
- **Builds On**: Error handling patterns (Chapter 9) for managing unsafe operations
- **Connections**: Uses generics extensively, prepares for building complex libraries and systems

## Community Conventions and Idioms
- Use unsafe code sparingly and always provide safe wrappers
- Document unsafe code extensively with safety invariants
- Prefer procedural macros over declarative macros for complex code generation
- Use `#![forbid(unsafe_code)]` in crates that shouldn't contain unsafe code
- Follow the "unsafe superpowers" principle: unsafe gives you 5 abilities, use them wisely
- Isolate unsafe code in small, well-tested functions with safe interfaces

## Personal Notes
- Advanced features are powerful but require deep understanding of Rust's guarantees
- Most applications don't need these features - start with safe Rust first
- Unsafe code requires careful reasoning about memory safety and data races
- Macros can make code harder to debug - use judiciously
- FFI is essential for interoperating with existing C/C++ codebases
- These features enable building the standard library and high-performance crates

Official Chapter: https://doc.rust-lang.org/book/ch19-00-advanced-features.html

---
*Completed: ✓*