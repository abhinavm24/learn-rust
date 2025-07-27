# Chapter 3.3: Functions

## Key Takeaways

### Function Fundamentals
- Functions are fundamental building blocks in Rust programs
- `main` function is entry point of every executable Rust program
- Functions can be defined anywhere in the code (before or after usage)
- Rust uses snake_case naming convention for function names
- Functions enable code reuse, organization, and abstraction
- All function parameters must have explicit type annotations

### Function Declaration Syntax
- Declared with `fn` keyword followed by function name
- Parameters defined in parentheses with type annotations
- Function body enclosed in curly braces `{}`
- Return type specified with `->` arrow syntax after parameters
- Example: `fn function_name(param: type) -> return_type { body }`

### Parameters and Arguments
- **Parameters**: Variables defined in function signature
- **Arguments**: Actual values passed when calling function
- Every parameter must have explicit type annotation
- Multiple parameters separated by commas
- Rust compiler cannot infer parameter types (unlike variables)
- Parameters are immutable by default in function scope

### Statements vs Expressions
- **Statements**: Instructions that perform action, do not return value
  - Variable declarations: `let y = 6;`
  - Function definitions: `fn main() {}`
  - Cannot assign statements to variables
- **Expressions**: Evaluate to a value
  - Math operations: `5 + 6`
  - Function calls: `another_function(5)`
  - Scope blocks: `{ let x = 3; x + 1 }`
  - Do NOT end with semicolons
- Adding semicolon to expression turns it into statement

### Return Values
- Functions can return values to calling code
- Return type declared after `->` in function signature
- Return value is final expression in function body (no semicolon)
- `return` keyword enables early returns from function
- Unit type `()` is implicit return when no value specified
- Must match declared return type exactly

### Important Syntax and Operators

#### Function Declaration
- `fn` - Function declaration keyword
  - `fn main() {}` - main function (entry point)
  - `fn another_function() {}` - custom function
- `->` - Return type annotation arrow
  - `fn five() -> i32` - function returning i32
- `()` - Parameter list parentheses (empty or with parameters)
- `{}` - Function body braces

#### Parameter Syntax
- `:` - Type annotation separator for parameters
  - `fn print_labeled_measurement(value: i32, unit_label: char)`
- `,` - Parameter separator
  - Multiple parameters: `param1: type1, param2: type2`

#### Return Syntax
- Implicit return: Last expression without semicolon
- `return` - Explicit return keyword for early returns
  - `return value;` - early return with semicolon
- `;` - Statement terminator (converts expressions to statements)

### Programming Concepts Introduced
- **Function Definition and Declaration**: Creating reusable code blocks
- **Parameter Passing**: Sending data into functions
- **Return Values**: Getting data back from functions  
- **Scope**: Function-local variable scope
- **Expression-based Programming**: Last expression as return value
- **Type Safety**: Explicit parameter typing prevents errors
- **Code Organization**: Breaking programs into logical functions
- **Immutable Parameters**: Function parameters cannot be modified by default

### Code Examples and Patterns

#### Basic Function Definition
```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

#### Functions with Parameters
```rust
fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

#### Statements vs Expressions
```rust
fn main() {
    let y = 6; // Statement - does not return value
    
    // This would cause error - statements don't return values:
    // let x = (let y = 6);
    
    // Expression example - scope block returns value
    let y = {
        let x = 3;
        x + 1  // No semicolon - this is an expression
    };
    
    println!("The value of y is: {y}"); // y = 4
}
```

#### Functions with Return Values
```rust
fn five() -> i32 {
    5  // Implicit return - no semicolon
}

fn plus_one(x: i32) -> i32 {
    x + 1  // Implicit return
}

fn early_return_example(x: i32) -> i32 {
    if x < 0 {
        return 0;  // Early return with semicolon
    }
    x * 2  // Implicit return
}

fn main() {
    let x = five();
    println!("The value of x is: {x}");
    
    let x = plus_one(5);
    println!("The value of x is: {x}");
}
```

#### Common Mistake - Adding Semicolon
```rust
fn plus_one(x: i32) -> i32 {
    x + 1;  // Error! Semicolon makes this a statement
}
// This would cause compile error: mismatched types
// Expected i32, found ()
```

#### Expressions in Different Contexts
```rust
fn main() {
    // Function call expression
    let result = plus_one(5);
    
    // Math expression
    let sum = 5 + 6;
    
    // Scope block expression
    let complex_calculation = {
        let a = 10;
        let b = 20;
        a + b + 5  // Returns 35
    };
    
    // Conditional expression
    let number = if true { 5 } else { 6 };
}
```

### Practical Applications
- Code organization and modularity
- Reducing code duplication through reusable functions
- Creating clean, readable program structure
- Parameter validation and data transformation
- Building libraries and APIs with well-defined interfaces
- Implementing mathematical operations and algorithms
- Creating helper functions for common tasks

### Integration with Previous Chapters
- Builds on Chapter 3.1's variable concepts within function scope
- Uses Chapter 3.2's data types for parameter and return type annotations
- Extends Chapter 2's program structure beyond simple main function
- Prepares for more complex program organization in later chapters
- Demonstrates Rust's expression-oriented programming paradigm

### Community Conventions and Idioms
- Use snake_case for function names: `calculate_area`, `get_user_input`
- Keep functions focused on single responsibility
- Prefer expressions over statements when returning values
- Use descriptive parameter names that indicate purpose
- Return meaningful values rather than printing from within functions
- Place helper functions after main function or in separate modules
- Use `->` return type annotation even for simple return types

### Personal Notes
- The distinction between statements and expressions is crucial for understanding Rust
- Implicit returns feel unusual coming from languages requiring explicit return
- Type annotations for all parameters enforces clear function contracts
- Expression-based programming leads to more functional programming style
- Function placement flexibility helps with code organization
- Missing semicolon errors are common when learning return value syntax

Official Chapter: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html

---
*Completed: ✓*