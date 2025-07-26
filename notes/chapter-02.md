# Chapter 2: Programming a Guessing Game

## Key Takeaways

### Project Setup with Cargo
- `cargo new guessing_game` creates new project
- `Cargo.toml` manages dependencies and project metadata
- Adding external crates: `rand = "0.8.5"` in `[dependencies]`
- `cargo build` downloads dependencies and compiles
- `cargo run` builds and executes in one step
- `cargo doc --open` generates and opens documentation for dependencies
- `cargo update` updates dependencies to latest compatible versions

### Variables and Mutability
- `let` declares immutable variables by default
- `let mut` creates mutable variables
- Variable shadowing: reusing variable names with new `let`
- Type inference: Rust can deduce types automatically
- Explicit typing: `let guess: u32 = ...`

### User Input and I/O
- `use std::io;` brings standard input/output into scope
- `String::new()` creates new empty String (associated function)
- `io::stdin().read_line(&mut string)` reads user input
- `read_line()` returns `Result<usize, Error>` (usize = bytes read)
- `&mut` creates mutable reference to pass to functions
- `.trim()` removes whitespace from strings
- `.parse()` converts strings to numbers (returns Result)
- `println!("You guessed: {guess}")` - variable interpolation with `{}`

### External Dependencies (Crates)
- External libraries called "crates"
- `rand` crate for random number generation
- `use rand::Rng;` brings trait into scope
- `rand::thread_rng().gen_range(1..=100)` generates random number
- Semantic versioning: `rand = "0.8.5"` gets compatible versions
- Cargo.lock ensures exact same dependency versions (reproducible builds)
- First `cargo build` downloads dependencies, subsequent builds faster

### Control Flow
- `loop` creates infinite loops
- `break` exits loops
- `match` for pattern matching and control flow
- `continue` skips to next loop iteration

### Error Handling with Result
- `Result<T, E>` enum for operations that can fail
- `.expect("message")` crashes program with message on error
- `match` on Result variants: `Ok(num)` and `Err(_)`
- `_` wildcard pattern ignores error details
- `continue` in match arms to retry on invalid input
- Graceful error handling vs crashing (.expect vs match)
- `parse()` returns `Result<T, ParseIntError>` - must handle conversion failures
- Two error handling approaches: crash fast (.expect) vs handle gracefully (match)

### Comparison and Ordering
- `use std::cmp::Ordering;` for comparison results
- `.cmp()` method compares two values
- `Ordering` enum: `Less`, `Greater`, `Equal`
- Pattern matching on comparison results

### Programming Concepts Introduced
- **Immutability by Default**: Variables immutable unless marked `mut`
- **Variable Shadowing**: Reusing variable names with new `let` declarations
- **References and Borrowing**: `&mut guess` passes mutable reference
- **Traits**: `Rng` trait provides random number methods
- **Enums**: `Result`, `Ordering` enums for type-safe error handling
- **Pattern Matching**: `match` expressions for control flow
- **Type System**: Strong static typing with inference
- **Associated Functions**: `String::new()` - function tied to type, not instance
- **Method Chaining**: `guess.trim().parse()` - fluent interface pattern
- **Cargo Package Management**: Adding external dependencies
- **Prelude**: Standard library items automatically imported

### Important Syntax and Operators
- `::` - Path separator for accessing items in modules/namespaces
  - `std::io::stdin()` - accesses stdin function in io module
  - `String::new()` - associated function (doesn't take self)
  - `rand::thread_rng()` - accesses function in rand crate
- `&` - Reference operator (borrowing)
  - `&mut guess` - mutable reference to guess variable
- `..=` - Inclusive range operator
  - `1..=100` - range from 1 to 100 (inclusive)
- `..` - Exclusive range operator
  - `1..101` - range from 1 to 100 (exclusive of 101)
- `.` - Method call operator (takes self)
  - `guess.trim().parse()` - chain method calls
- `{}` - Placeholder for string interpolation in println!
- `_` - Wildcard pattern (ignores values in match)
- `!` - Macro invocation operator
  - `println!()` - macro, not function (compile-time code generation)
- `=>` - Match arm separator
  - `Ok(num) => num,` - pattern => expression in match arms
- `mut` - Mutability keyword
  - `let mut guess` - declares mutable variable
- `let` - Variable binding keyword
  - Creates new variable binding (with shadowing capability)
- `:` - Type annotation separator
  - `let guess: u32` - explicit type specification

### Code Examples and Patterns
```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

### Variable Shadowing Example
```rust
// First guess as String
let mut guess = String::new();
io::stdin().read_line(&mut guess).expect("Failed to read line");

// Shadow with parsed u32 version
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

### Practical Applications
- Building interactive console applications
- Handling user input validation robustly
- Working with external crates and dependencies
- Implementing game loops and state management
- Converting between string and numeric types safely
- Error recovery patterns for user-facing applications

### Integration with Previous Chapters  
- Builds on Chapter 1's `println!` macro usage
- Extends basic variable concepts with mutability
- Introduces first real program beyond "Hello, world!"
- Demonstrates practical use of Cargo build system
- Shows real-world application of Rust's safety guarantees

### Community Conventions and Idioms
- Use `cargo new` for all new projects
- Prefer `match` over `.unwrap()` for error handling in production code
- Use meaningful variable names even when shadowing
- Add dependencies with specific version numbers in Cargo.toml
- Use `.trim()` when parsing user input to handle whitespace
- Prefer inclusive ranges (`..=`) for intuitive numeric ranges

### Personal Notes
- The guessing game demonstrates Rust's "fail fast" vs "handle gracefully" philosophies
- Variable shadowing is particularly useful for type conversions
- Cargo's dependency management feels similar to npm/pip but with stronger versioning
- Match expressions provide exhaustive pattern matching - compiler ensures all cases handled
- The type system prevents many runtime errors that would occur in dynamic languages

Rust Prelude: https://doc.rust-lang.org/std/prelude/index.html

---
*Completed: ✓*