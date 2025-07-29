# Chapter 5.3: Method Syntax

## Key Takeaways

### Method Fundamentals
- Methods are functions defined within the context of a struct (or enum/trait object)
- First parameter is always `self`, which represents the instance
- Called using dot notation: `instance.method()`
- Defined within `impl` (implementation) blocks

### Self Parameter Variations
- `self` - Takes ownership of the instance (consumes it)
- `&self` - Borrows the instance immutably (most common)
- `&mut self` - Borrows the instance mutably
- Rust automatically handles referencing/dereferencing

### Associated Functions
- Functions defined in `impl` blocks that don't take `self`
- Called using `::` syntax: `Type::function()`
- Often used as constructors (like `String::new()`)
- Also called "static methods" in other languages

### Important Syntax and Operators

#### Method Definition
```rust
impl StructName {
    fn method_name(&self, param: Type) -> ReturnType {
        // method body
    }
}
```

#### Associated Function Definition
```rust
impl StructName {
    fn function_name(param: Type) -> ReturnType {
        // function body
    }
}
```

#### Method Call
- `instance.method()` - Method call
- `Type::function()` - Associated function call

### Programming Concepts Introduced
- **Object-Oriented Style**: Methods provide familiar OOP syntax
- **Automatic Referencing**: Rust handles `&`, `&mut`, and `*` automatically
- **Namespacing**: Methods are associated with their types
- **Constructor Pattern**: Associated functions for instance creation

### Code Examples and Patterns

#### Basic Method Implementation
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
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
    
    println!("Area: {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
```

#### Different Self Parameter Types
```rust
impl Rectangle {
    // Immutable borrow - most common
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Mutable borrow - when you need to modify
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
    
    // Takes ownership - consumes the instance
    fn destroy(self) -> String {
        format!("Destroying rectangle {}x{}", self.width, self.height)
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 10,
        height: 20,
    };
    
    println!("Area: {}", rect.area());     // &self
    rect.double_size();                    // &mut self
    println!("New area: {}", rect.area()); // &self again
    
    let message = rect.destroy();          // self (consumes rect)
    // rect is no longer accessible here
    println!("{}", message);
}
```

#### Associated Functions (Constructors)
```rust
impl Rectangle {
    // Associated function - constructor
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }
    
    // Another constructor for squares
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // Utility function
    fn from_area_and_ratio(area: u32, ratio: f64) -> Rectangle {
        let width = (area as f64 * ratio).sqrt() as u32;
        let height = area / width;
        Rectangle { width, height }
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let square = Rectangle::square(25);
    let rect2 = Rectangle::from_area_and_ratio(100, 1.5);
    
    println!("rect1: {:#?}", rect1);
    println!("square: {:#?}", square);
    println!("rect2: {:#?}", rect2);
}
```

#### Multiple impl Blocks
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// First impl block - basic operations
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

// Second impl block - comparison operations
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    fn is_larger_than(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
}

// Third impl block - constructors
impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

#### Method Chaining Pattern
```rust
impl Rectangle {
    fn set_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    
    fn set_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
    
    fn double(mut self) -> Self {
        self.width *= 2;
        self.height *= 2;
        self
    }
}

fn main() {
    let rect = Rectangle::new(10, 20)
        .set_width(15)
        .double()
        .set_height(50);
    
    println!("Final rectangle: {:#?}", rect);
}
```

#### Automatic Referencing and Dereferencing
```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle::new(10, 20);
    let rect_ref = &rect;
    let rect_box = Box::new(rect);
    
    // All of these work the same way!
    println!("{}", rect.area());      // Direct call
    println!("{}", rect_ref.area());  // Auto-dereference
    println!("{}", rect_box.area());  // Auto-dereference through Box
    
    // Equivalent to:
    // println!("{}", (&rect).area());
    // println!("{}", (*rect_ref).area());
    // println!("{}", (*rect_box).area());
}
```

#### Real-World Example: User Account
```rust
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

impl User {
    // Constructor
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            active: true,
            sign_in_count: 0,
        }
    }
    
    // Methods
    fn sign_in(&mut self) {
        self.sign_in_count += 1;
    }
    
    fn deactivate(&mut self) {
        self.active = false;
    }
    
    fn is_active(&self) -> bool {
        self.active
    }
    
    fn change_email(&mut self, new_email: String) {
        self.email = new_email;
    }
    
    fn get_display_name(&self) -> &str {
        &self.username
    }
}

fn main() {
    let mut user = User::new(
        String::from("alice123"),
        String::from("alice@example.com")
    );
    
    user.sign_in();
    user.sign_in();
    println!("User: {:#?}", user);
}
```

### Practical Applications
- Creating intuitive APIs with method chaining
- Organizing functionality by data type
- Building constructors with associated functions
- Encapsulating state changes within methods
- Providing clean interfaces for complex operations

### Integration with Previous Chapters
- Uses struct definitions from Chapter 5.1
- Applies borrowing rules from Chapter 4.2
- Demonstrates ownership transfer with different self types
- Builds on function concepts from Chapter 3.3

### Community Conventions and Idioms
- Use `&self` for most methods (avoid unnecessary ownership transfer)
- Name constructors `new()` or describe what they create
- Use `&mut self` sparingly and document side effects
- Group related functionality in the same `impl` block
- Consider method chaining for builder-like APIs

### Benefits Over Functions
- **Namespacing**: Methods belong to their type
- **Discoverability**: IDE can suggest available methods
- **Ergonomics**: Dot notation feels natural
- **Automatic Reference Handling**: No manual `&` needed
- **Object-Oriented Feel**: Familiar to developers from other languages

### Personal Notes
- Method syntax makes Rust feel much more object-oriented
- Automatic referencing/dereferencing is incredibly convenient
- Associated functions are perfect for constructors
- Multiple `impl` blocks help organize large types
- The distinction between methods and functions becomes very clear

Official Chapter: https://doc.rust-lang.org/book/ch05-03-method-syntax.html

---
*Completed: ✓*