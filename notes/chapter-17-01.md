# Chapter 17.1: Characteristics of Object-Oriented Languages

## Key Takeaways
- **Object-Oriented Programming (OOP) Definition**: A programming paradigm that uses objects containing data (attributes) and code (methods) to design applications and computer programs
- **Rust's OOP Approach**: Rust is not traditionally object-oriented but provides features that enable object-oriented design patterns
- **Core OOP Characteristics**: Objects, encapsulation, and inheritance (Rust supports some but not all)

## Objects Contain Data and Behavior

### Traditional OOP Objects
```rust
// Rust structs can act like objects
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

### Using Objects
```rust
fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    
    println!("Area: {}", rect1.area());
    println!("Can hold: {}", rect1.can_hold(&rect2));
}
```

## Encapsulation
- **Privacy Rules**: Rust uses `pub` keyword to control access
- **Data Hiding**: Internal implementation details are hidden from external code

### Encapsulation Example
```rust
pub struct AveragedCollection {
    list: Vec<i32>,        // Private field
    average: f64,          // Private field
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }
    
    pub fn average(&self) -> f64 {
        self.average
    }
    
    fn update_average(&mut self) {  // Private method
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

## Inheritance vs Composition
- **Traditional Inheritance**: Rust does not support class-based inheritance
- **Trait-Based Polymorphism**: Rust uses traits for shared behavior
- **Composition Over Inheritance**: Rust encourages composition patterns

### Trait-Based Shared Behavior
```rust
pub trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn summarize_author(&self) -> String {
        format!("(Read more from {}...)", self.summarize())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

## Rust's OOP Characteristics

### What Rust Has
1. **Objects**: Structs with data and methods
2. **Encapsulation**: Privacy controls with `pub`
3. **Polymorphism**: Trait objects and generics

### What Rust Lacks
1. **Classical Inheritance**: No class hierarchies
2. **Method Overriding**: No traditional inheritance-based overriding
3. **Constructor/Destructor Patterns**: Uses associated functions and `Drop` trait

## Alternative Patterns

### Composition Pattern
```rust
struct Engine {
    horsepower: u32,
}

impl Engine {
    fn start(&self) {
        println!("Engine with {} HP started", self.horsepower);
    }
}

struct Car {
    engine: Engine,
    make: String,
    model: String,
}

impl Car {
    fn new(make: String, model: String, horsepower: u32) -> Car {
        Car {
            engine: Engine { horsepower },
            make,
            model,
        }
    }
    
    fn start(&self) {
        println!("Starting {} {}", self.make, self.model);
        self.engine.start();
    }
}
```

## Integration with Previous Concepts
- **Ownership System**: OOP patterns must work within Rust's ownership rules
- **Borrowing**: Methods take `&self`, `&mut self`, or `self` appropriately
- **Lifetimes**: Object references must have valid lifetimes
- **Error Handling**: OOP methods should return `Result<T, E>` when appropriate

## Best Practices
1. **Prefer Composition**: Use struct composition over trait inheritance
2. **Design for Interfaces**: Define behavior through traits
3. **Encapsulate Carefully**: Make fields private and provide controlled access
4. **Use Associated Functions**: For constructor-like patterns
5. **Implement Drop**: For cleanup when objects go out of scope

## Common Patterns
- **Builder Pattern**: For complex object construction
- **Strategy Pattern**: Using trait objects
- **State Pattern**: Encoding states in the type system
- **Factory Pattern**: Using associated functions

This approach to OOP in Rust emphasizes safety, performance, and explicit design choices while providing the benefits of object-oriented programming paradigms.