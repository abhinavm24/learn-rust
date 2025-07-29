# Chapter 5: Using Structs to Structure Related Data

## Key Takeaways

### Struct Fundamentals
- **Data Grouping**: Structs group related data together
- **Custom Types**: Create meaningful custom data types
- **Memory Layout**: Efficient memory representation
- **Method Syntax**: Associate functions and methods with structs

### Struct Types
- **Named Field Structs**: Most common, fields have names
- **Tuple Structs**: Fields have types but no names
- **Unit Structs**: No fields, useful for traits
- **Ownership**: Structs can own, borrow, or reference data

### Methods and Associated Functions
- **impl Blocks**: Define methods and associated functions
- **&self Parameter**: Borrow self immutably
- **&mut self Parameter**: Borrow self mutably
- **self Parameter**: Take ownership of self
- **Associated Functions**: Don't take self, often constructors

### Design Patterns
- **Builder Pattern**: Construct complex objects step by step
- **Method Chaining**: Chain method calls for fluent APIs
- **Encapsulation**: Keep data private, expose public methods
- **Constructor Functions**: Associated functions that create instances

## Chapter Structure

### 5.1: Defining and Instantiating Structs
```rust
// Basic struct definition
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Creating instances
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// Field init shorthand
fn build_user(email: String, username: String) -> User {
    User {
        email,      // Instead of email: email,
        username,   // Instead of username: username,
        active: true,
        sign_in_count: 1,
    }
}

// Struct update syntax
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1  // Use remaining fields from user1
};

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// Unit-like structs
struct AlwaysEqual;
let subject = AlwaysEqual;
```

### 5.2: An Example Program Using Structs
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
    
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);  // Pretty print
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Using dbg! macro
fn debug_example() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),  // Prints debug info
        height: 50,
    };
    
    dbg!(&rect1);  // Prints the entire struct
}
```

### 5.3: Method Syntax
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method - takes &self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method with parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Mutable method
    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
    
    // Method that takes ownership
    fn consume(self) -> u32 {
        self.width * self.height
    }  // self is dropped here
    
    // Associated function (constructor)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    // Another associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Multiple impl blocks are allowed
impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::square(25);
    
    println!("Area: {}", rect1.area());
    println!("Can hold rect2: {}", rect1.can_hold(&rect2));
    println!("Perimeter: {}", rect1.perimeter());
}
```

## Advanced Struct Patterns

### Struct with Lifetimes
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("Important part: {}", i.part);
}
```

### Generic Structs
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
}

// Specific implementation for f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Multiple generic parameters
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MixedPoint<T, U> {
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}
```

### Struct with Methods Pattern
```rust
struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Counter {
        Counter { current: 0, max }
    }
    
    fn next(&mut self) -> Option<usize> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
    
    fn reset(&mut self) {
        self.current = 0;
    }
    
    fn is_done(&self) -> bool {
        self.current >= self.max
    }
}

// Iterator implementation
impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

fn main() {
    let mut counter = Counter::new(5);
    
    while let Some(value) = counter.next() {
        println!("Count: {}", value);
    }
    
    // Using as iterator
    let counter2 = Counter::new(3);
    let values: Vec<usize> = counter2.collect();
    println!("Collected: {:?}", values);
}
```

## Common Struct Patterns

### Builder Pattern
```rust
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: Option<u32>,
    is_admin: bool,
}

struct UserBuilder {
    name: Option<String>,
    email: Option<String>,
    age: Option<u32>,
    is_admin: bool,
}

impl UserBuilder {
    fn new() -> Self {
        UserBuilder {
            name: None,
            email: None,
            age: None,
            is_admin: false,
        }
    }
    
    fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    
    fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }
    
    fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }
    
    fn admin(mut self) -> Self {
        self.is_admin = true;
        self
    }
    
    fn build(self) -> Result<User, String> {
        let name = self.name.ok_or("Name is required")?;
        let email = self.email.ok_or("Email is required")?;
        
        Ok(User {
            name,
            email,
            age: self.age,
            is_admin: self.is_admin,
        })
    }
}

fn main() {
    let user = UserBuilder::new()
        .name("Alice")
        .email("alice@example.com")
        .age(30)
        .admin()
        .build()
        .expect("Failed to build user");
    
    println!("{:#?}", user);
}
```

### State Machine Pattern
```rust
struct Locked;
struct Unlocked;

struct StateMachine<State> {
    state: std::marker::PhantomData<State>,
}

impl StateMachine<Locked> {
    fn new() -> Self {
        StateMachine {
            state: std::marker::PhantomData,
        }
    }
    
    fn unlock(self) -> StateMachine<Unlocked> {
        println!("Unlocking...");
        StateMachine {
            state: std::marker::PhantomData,
        }
    }
}

impl StateMachine<Unlocked> {
    fn lock(self) -> StateMachine<Locked> {
        println!("Locking...");
        StateMachine {
            state: std::marker::PhantomData,
        }
    }
    
    fn use_resource(&self) {
        println!("Using resource while unlocked");
    }
}
```

### Newtype Pattern
```rust
struct UserId(u32);
struct Email(String);
struct Age(u8);

impl UserId {
    fn new(id: u32) -> Self {
        UserId(id)
    }
    
    fn value(&self) -> u32 {
        self.0
    }
}

impl Email {
    fn new(email: String) -> Result<Self, String> {
        if email.contains('@') {
            Ok(Email(email))
        } else {
            Err("Invalid email format".to_string())
        }
    }
    
    fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }
}

impl Age {
    fn new(age: u8) -> Result<Self, String> {
        if age <= 150 {
            Ok(Age(age))
        } else {
            Err("Age too high".to_string())
        }
    }
    
    fn is_adult(&self) -> bool {
        self.0 >= 18
    }
}

#[derive(Debug)]
struct Person {
    id: UserId,
    email: Email,
    age: Age,
}

impl Person {
    fn new(id: u32, email: String, age: u8) -> Result<Self, String> {
        Ok(Person {
            id: UserId::new(id),
            email: Email::new(email)?,
            age: Age::new(age)?,
        })
    }
}
```

## Memory and Performance

### Memory Layout
```rust
#[repr(C)]  // Use C-style layout
struct Point {
    x: f64,  // 8 bytes
    y: f64,  // 8 bytes
}  // Total: 16 bytes

#[repr(packed)]  // No padding
struct PackedStruct {
    a: u8,   // 1 byte
    b: u32,  // 4 bytes
}  // Total: 5 bytes (instead of 8 with padding)

// Zero-sized types
struct Marker;
struct PhantomStruct<T> {
    _phantom: std::marker::PhantomData<T>,
}
```

### Performance Considerations
```rust
// Prefer borrowing in methods
impl Rectangle {
    // ✅ Good: borrows self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // ❌ Avoid: unnecessary ownership
    fn bad_area(self) -> u32 {
        self.width * self.height
    }  // self is consumed
}

// Use references for large structs
fn process_large_struct(data: &LargeStruct) {
    // Process without copying
}

// Return owned data when creating new values
impl Point {
    fn translate(mut self, dx: i32, dy: i32) -> Self {
        self.x += dx;
        self.y += dy;
        self  // Return owned, modified self
    }
}
```

## Integration with Traits

### Implementing Common Traits
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

// Custom implementations
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}
```

## Best Practices

### API Design
```rust
// ✅ Good: Accept borrowed data when possible
impl User {
    fn set_email(&mut self, email: &str) {
        self.email = email.to_string();
    }
}

// ✅ Good: Return owned data for new values
impl Rectangle {
    fn scaled(&self, factor: f64) -> Rectangle {
        Rectangle {
            width: (self.width as f64 * factor) as u32,
            height: (self.height as f64 * factor) as u32,
        }
    }
}

// ✅ Good: Use associated functions for constructors
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

### Error Handling
```rust
#[derive(Debug)]
enum UserError {
    InvalidEmail,
    AgeTooLow,
    NameTooShort,
}

impl User {
    fn new(name: String, email: String, age: u32) -> Result<Self, UserError> {
        if !email.contains('@') {
            return Err(UserError::InvalidEmail);
        }
        
        if age < 13 {
            return Err(UserError::AgeTooLow);
        }
        
        if name.len() < 2 {
            return Err(UserError::NameTooShort);
        }
        
        Ok(User {
            name,
            email,
            age,
            is_admin: false,
        })
    }
}
```

## Testing Structs
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.area(), 200);
    }
    
    #[test]
    fn test_rectangle_can_hold() {
        let larger = Rectangle::new(10, 20);
        let smaller = Rectangle::new(5, 10);
        
        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }
    
    #[test]
    fn test_user_creation() {
        let user = User::new(
            "Alice".to_string(),
            "alice@example.com".to_string(),
            25
        ).unwrap();
        
        assert_eq!(user.name, "Alice");
        assert!(!user.is_admin);
    }
}
```

Official Chapter: https://doc.rust-lang.org/book/ch05-00-structs.html

---
*Completed: ✓*