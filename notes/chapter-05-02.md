# Chapter 5.2: An Example Program Using Structs

## Key Takeaways

### Practical Struct Usage
- Demonstrates struct evolution from tuples to named fields
- Shows how structs improve code readability and maintainability
- Illustrates debugging with `#[derive(Debug)]`
- Example builds a rectangle area calculator

### Debug Trait and Printing
- `#[derive(Debug)]` automatically implements Debug trait
- Enables printing structs with `{:?}` and `{:#?}` format specifiers
- `println!` macro can't print structs without Debug implementation
- Pretty printing with `{:#?}` for better readability

### Code Evolution Pattern
- Start with simple types (primitives, tuples)
- Identify when code becomes unclear or error-prone
- Refactor to structs for better organization
- Add debugging capabilities as needed

### Important Syntax and Operators

#### Debug Derivation
```rust
#[derive(Debug)]
struct StructName {
    field1: Type1,
    field2: Type2,
}
```

#### Debug Printing
- `println!("{:?}", instance)` - Compact debug output
- `println!("{:#?}", instance)` - Pretty-printed debug output
- `dbg!(expression)` - Debug macro that prints and returns value

### Programming Concepts Introduced
- **Incremental Refactoring**: Improving code structure step by step
- **Trait Derivation**: Automatically implementing common traits
- **Debug Output**: Essential tool for development and troubleshooting
- **Code Organization**: Using structs to group related data

### Code Examples and Patterns

#### Starting with Primitives
```rust
fn main() {
    let width1 = 30;
    let height1 = 50;
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

#### Refactoring with Tuples
```rust
fn main() {
    let rect1 = (30, 50);
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

#### Final Version with Structs
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    
    println!("rect1 is {:?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

#### Debug Printing Examples
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    // Compact debug output
    println!("rect1 is {:?}", rect1);
    // Output: rect1 is Rectangle { width: 30, height: 50 }
    
    // Pretty-printed debug output
    println!("rect1 is {:#?}", rect1);
    // Output:
    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50,
    // }
}
```

#### Using the dbg! Macro
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),  // Prints the calculation and result
        height: 50,
    };
    
    dbg!(&rect1);  // Prints the entire struct
}

// Output:
// [src/main.rs:10] 30 * scale = 60
// [src/main.rs:14] &rect1 = Rectangle {
//     width: 60,
//     height: 50,
// }
```

#### Multiple Rectangle Operations
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    println!("Area of rect1: {}", area(&rect1));
    println!("Area of rect2: {}", area(&rect2));
    println!("Area of rect3: {}", area(&rect3));
    
    println!("rect1 is {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

#### Without Debug Trait (Won't Compile)
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    // This won't compile!
    // println!("rect1 is {:?}", rect1);
    // Error: `Rectangle` doesn't implement `Debug`
}
```

#### Custom Debug Implementation (Alternative)
```rust
use std::fmt;

struct Rectangle {
    width: u32,
    height: u32,
}

impl fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rectangle")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("area", &(self.width * self.height))
            .finish()
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("rect1 is {:?}", rect1);
    // Output: rect1 is Rectangle { width: 30, height: 50, area: 1500 }
}
```

### Practical Applications
- Prototyping and development debugging
- Logging structured data
- Testing and validation output
- Development tools and utilities
- Configuration display and verification

### Benefits of This Approach
- **Incremental Development**: Start simple, add complexity gradually
- **Clear Intent**: Struct fields are self-documenting
- **Better Errors**: Compiler helps catch field-related mistakes
- **Debugging Support**: Easy to inspect struct contents
- **Maintainability**: Changes to data structure are localized

### Integration with Previous Chapters
- Uses struct definition concepts from Chapter 5.1
- Applies borrowing with function parameters
- Demonstrates ownership with struct instances
- Shows practical function design patterns

### Community Conventions and Idioms
- Always add `#[derive(Debug)]` during development
- Use `dbg!` macro for temporary debugging
- Remove debug prints before production (or use proper logging)
- Consider implementing custom Debug for sensitive data
- Use pretty printing `{:#?}` for complex nested structures

### Personal Notes
- The evolution from primitives → tuples → structs shows clear progression
- Debug trait is essential for development workflow
- `dbg!` macro is incredibly useful for understanding program flow
- Borrowing references in functions avoids unnecessary ownership transfer
- This pattern of incremental improvement is common in real Rust development

Official Chapter: https://doc.rust-lang.org/book/ch05-02-example-structs.html

---
*Completed: ✓*