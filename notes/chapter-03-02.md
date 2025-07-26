# Chapter 3.2: Data Types

## Key Takeaways

### Static Type System
- Rust is statically typed - all variable types known at compile time
- Compiler must know types of all variables to compile
- Type inference allows omitting type annotations when compiler can deduce
- Must provide type annotations when multiple types possible
- Compile-time type checking prevents runtime type errors

### Scalar Types - Single Values
Represent single values, four primary scalar types in Rust.

#### Integer Types
- Signed integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- Number indicates bits used: `i32` = 32-bit signed integer
- `isize` and `usize` depend on computer architecture (32/64-bit)
- **Default**: `i32` (generally fastest, even on 64-bit systems)
- **Range calculation**: signed from -(2^(n-1)) to 2^(n-1) - 1
- **Range calculation**: unsigned from 0 to 2^n - 1

#### Integer Literals
- **Decimal**: `98_222` (underscores for readability)
- **Hexadecimal**: `0xff`
- **Octal**: `0o77`
- **Binary**: `0b1111_0000`
- **Byte (u8 only)**: `b'A'`

#### Integer Overflow
- **Debug mode**: Panic on overflow (program crashes)
- **Release mode**: Two's complement wrapping (255u8 + 1 = 0)
- **Explicit handling methods**:
  - `wrapping_*` methods: `wrapping_add()`, `wrapping_sub()`
  - `checked_*` methods: Return `Option<T>` (`None` on overflow)
  - `overflowing_*` methods: Return value and boolean indicating overflow
  - `saturating_*` methods: Clamp at min/max values

#### Floating-Point Types
- `f32` (32 bits, single precision)
- `f64` (64 bits, double precision) - **default type**
- IEEE-754 standard compliant
- All floating-point types are signed
- Capable of representing both positive and negative decimal numbers

#### Boolean Type
- `bool` type with two values: `true` and `false`
- One byte in size
- Used in conditionals like `if` expressions

#### Character Type
- `char` type represents Unicode Scalar Value
- **Size**: 4 bytes (can represent more than ASCII)
- **Specified**: Single quotes `'z'`, `'ℤ'`, `'😻'`
- **Range**: U+0000 to U+D7FF and U+E000 to U+10FFFF
- More than "character" concept in other languages

### Compound Types - Multiple Values
Group multiple values into one type.

#### Tuple Type
- Groups values of different types together
- Fixed length once declared
- **Syntax**: `(type1, type2, type3)`
- **Access**: Destructuring or dot notation with index
- **Unit tuple**: `()` represents empty value/empty return type

#### Array Type
- Every element must have same type
- Fixed length (unlike vectors)
- Data allocated on stack (not heap)
- **Syntax**: `[type; length]` or `[value; count]`
- **Access**: Indexing with `array[index]`
- **Runtime bounds checking**: Panic on invalid index access
- Useful when you want data on stack or ensure fixed number of elements

### Important Syntax and Operators

#### Type Annotations
- `:` - Type annotation separator
  - `let x: i32 = 5;` - explicit type declaration
  - `let guess: u32 = "42".parse().expect("Not a number!");`

#### Numeric Literals
- `_` - Underscore separator for readability in numbers
  - `98_222`, `1_000`, `0b1111_0000`
- `0x` - Hexadecimal prefix (`0xff`)
- `0o` - Octal prefix (`0o77`)
- `0b` - Binary prefix (`0b1111_0000`)
- `b` - Byte literal prefix (`b'A'`)

#### Array Syntax
- `[` `]` - Array declaration brackets
  - `[1, 2, 3, 4, 5]` - array with explicit values
  - `[3; 5]` - array with 5 elements, all value 3
  - `array[0]` - array indexing (zero-based)

#### Tuple Syntax
- `(` `)` - Tuple declaration parentheses
  - `(500, 6.4, 1)` - tuple with mixed types
  - `tup.0` - tuple element access by index
- Destructuring assignment with `let (x, y, z) = tup;`

#### Arithmetic Operators
- `+` - Addition
- `-` - Subtraction  
- `*` - Multiplication
- `/` - Division (integer division for integer types)
- `%` - Remainder (modulo)

### Programming Concepts Introduced
- **Static Typing**: Compile-time type safety
- **Type Inference**: Automatic type deduction by compiler
- **Type Annotations**: Explicit type specifications when needed
- **Memory Layout**: Stack vs heap allocation patterns
- **Bounds Checking**: Runtime array access validation
- **Unicode Support**: Full international character support
- **Pattern Matching**: Destructuring for compound types
- **Integer Overflow Handling**: Explicit control over overflow behavior
- **Two's Complement**: Binary representation understanding

### Code Examples and Patterns

#### Basic Type Declarations
```rust
fn main() {
    // Integer types
    let x = 2.0; // f64 by default
    let y: f32 = 3.0; // f32 with type annotation
    
    // Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation
    
    // Character
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

#### Integer Literals and Operations
```rust
fn main() {
    // Integer literal formats
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    
    // Arithmetic operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    let remainder = 43 % 5;
}
```

#### Tuple Usage
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // Destructuring
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    
    // Index access
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
}
```

#### Array Usage
```rust
fn main() {
    // Array declaration
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June",
                  "July", "August", "September", "October", "November", "December"];
    
    // Array with type annotation
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Array with same value
    let a = [3; 5]; // equivalent to [3, 3, 3, 3, 3]
    
    // Array access
    let first = a[0];
    let second = a[1];
}
```

#### Type Annotations in Practice
```rust
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    
    // Without type annotation, this would fail to compile
    // let guess = "42".parse().expect("Not a number!"); // Error!
}
```

### Practical Applications
- Numeric computations with appropriate precision control
- Text processing with full Unicode support
- Data structure design using tuples for related data
- Fixed-size collections with arrays for performance
- Type-safe APIs preventing common programming errors
- Memory-efficient programming with stack allocation
- Cross-platform development with consistent integer sizes

### Integration with Previous Chapters
- Builds on Chapter 3.1's variable concepts with typing
- Extends Chapter 2's string parsing with type annotations
- Prepares for more complex data structures in later chapters
- Reinforces Rust's compile-time safety guarantees
- Demonstrates zero-cost abstractions in practice

### Community Conventions and Idioms
- Use `i32` for general integer usage (default choice)
- Use `usize` for array/vector indexing and lengths
- Use `f64` for floating-point calculations (default precision)
- Use type annotations when compiler needs disambiguation
- Use underscores in numeric literals for readability
- Prefer arrays for fixed-size data, vectors for dynamic
- Use tuples for simple grouped data, structs for complex

### Personal Notes
- Static typing catches errors early but requires more explicit declarations
- The distinction between stack and heap allocation becomes important for performance
- Unicode char support is more comprehensive than many languages
- Array bounds checking at runtime provides safety with some performance cost
- Integer overflow handling gives explicit control over edge cases
- Type inference reduces verbosity while maintaining safety

Official Chapter: https://doc.rust-lang.org/book/ch03-02-data-types.html

---
*Completed: ✓*