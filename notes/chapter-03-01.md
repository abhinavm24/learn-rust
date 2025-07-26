# Chapter 3.1: Variables and Mutability

## Key Takeaways

### Variables and Immutability by Default
- Variables are immutable by default in Rust
- `let x = 5;` creates an immutable variable
- Attempting to reassign immutable variable causes compile error
- Immutability provides safety, prevents bugs, and enables optimizations
- Makes code intentions explicit and clear

### Mutability with `mut` Keyword
- `let mut x = 5;` creates a mutable variable
- Only mutable variables can be reassigned: `x = 6;`
- Mutability must be explicitly declared
- Trade-off between flexibility and safety
- Use mutability when variable needs to change

### Constants
- Declared with `const` keyword, not `let`
- Always immutable (no `mut` allowed with constants)
- Must include explicit type annotation
- Must be set to constant expression (compile-time evaluable)
- Can be declared in any scope, including global scope
- Valid for entire program runtime within their scope
- Naming convention: SCREAMING_SNAKE_CASE
- Example: `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`

### Variable Shadowing
- Creating new variable with same name using `let`
- Previous variable becomes inaccessible (shadowed)
- Different from mutability - creates entirely new variable
- Can change type when shadowing
- Useful for transformations without new variable names
- Each `let` creates a new variable in memory

### Important Syntax and Operators
- `let` - Variable declaration keyword
  - `let x = 5;` - immutable variable
  - `let mut x = 5;` - mutable variable
- `const` - Constant declaration keyword
  - `const MAX_POINTS: u32 = 100_000;` - constant with type annotation
- `mut` - Mutability modifier keyword
  - Only used with `let`, not with `const`
- `=` - Assignment operator
  - Initial assignment or reassignment for mutable variables
- `_` - Underscore separator in numeric literals
  - `100_000` for readability (equivalent to `100000`)
- `:` - Type annotation separator
  - `const NAME: type = value;` - required for constants

### Programming Concepts Introduced
- **Immutability by Default**: Rust's core safety principle
- **Explicit Mutability**: Must declare intent to modify variables
- **Compile-time Constants**: Values computed at compile time
- **Variable Shadowing**: Type-safe variable reuse pattern
- **Memory Safety**: Preventing unintended mutations
- **Zero-cost Abstractions**: Immutability enables optimizations
- **Scope-based Lifetime**: Variables live within their scope
- **Type System Integration**: Shadowing allows type changes

### Practical Applications
- Building robust, bug-resistant programs
- Functional programming patterns with immutable data
- Performance optimizations through compiler guarantees
- Clear code documentation through mutability declarations
- Safe concurrent programming foundations
- Configuration and mathematical constants
- Data transformation pipelines using shadowing

### Code Examples and Patterns

#### Basic Variable Declaration
```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; // This would cause compile error
}
```

#### Mutable Variables
```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

#### Constants
```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
}
```

#### Variable Shadowing
```rust
fn main() {
    let x = 5;
    let x = x + 1;  // Shadows previous x
    {
        let x = x * 2;  // Shadows again in inner scope
        println!("The value of x in the inner scope is: {x}"); // 12
    }
    println!("The value of x is: {x}"); // 6
}
```

#### Shadowing with Type Changes
```rust
fn main() {
    let spaces = "   ";           // String type
    let spaces = spaces.len();    // usize type (number)
    println!("Number of spaces: {spaces}");
    
    // This would NOT work with mut:
    // let mut spaces = "   ";
    // spaces = spaces.len(); // Error: can't assign usize to &str
}
```

### Integration with Previous Chapters
- Extends Chapter 1's basic variable usage
- Builds foundation for Chapter 2's mutable string handling
- Prepares for advanced ownership concepts in later chapters
- Reinforces Rust's safety-first design philosophy
- Demonstrates compile-time error prevention

### Community Conventions and Idioms
- Prefer immutable variables when possible
- Use descriptive names even for temporary variables
- Use SCREAMING_SNAKE_CASE for constants
- Use shadowing for type conversions and transformations
- Declare mutability only when necessary
- Use `_` separators in large numeric literals for readability
- Place constants at module level when used across functions

### Personal Notes
- Immutability by default feels restrictive initially but prevents many bugs
- Shadowing is powerful for data transformation pipelines
- Constants are compile-time evaluated, making them zero-cost
- The distinction between shadowing and mutability is crucial
- Rust's approach contrasts with most languages that default to mutability

Official Chapter: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

---
*Completed: ✓*