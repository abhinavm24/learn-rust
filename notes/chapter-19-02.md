# Chapter 19.2: Advanced Traits

## Key Takeaways
- **Associated Types**: Type placeholders within traits for cleaner interfaces
- **Default Generic Type Parameters**: Reduce boilerplate in trait definitions
- **Supertraits**: Require implementing another trait as prerequisite
- **Newtype Pattern**: Wrapper types to implement external traits on external types
- **Disambiguation**: Fully qualified syntax for calling methods

## Associated Types vs Generic Types

### Associated Types Basics

```rust
pub trait Iterator {
    type Item;  // Associated type
    
    fn next(&mut self) -> Option<Self::Item>;
}

// Implementation specifies the associated type
impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Implementation details
        None
    }
}

struct Counter;
```

### Comparing with Generics

```rust
// With generics (allows multiple implementations)
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

// Could implement for same type with different T
impl Iterator<String> for Counter {
    fn next(&mut self) -> Option<String> { None }
}

impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> { None }
}

// With associated types (only one implementation per type)
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// Only one implementation possible
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> { None }
}
```

### When to Use Associated Types

```rust
// ✅ Good: Conceptually one output type per implementor
trait Collect {
    type Output;
    fn collect(self) -> Self::Output;
}

// ❌ Confusing: Multiple possible output types
trait Collect<T> {
    fn collect(self) -> T;
}
```

## Default Generic Type Parameters

### Basic Default Parameters

```rust
// Default Rhs to Self
trait Add<Rhs=Self> {
    type Output;
    
    fn add(self, rhs: Rhs) -> Self::Output;
}

// Using the default
impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;  // Uses default Rhs=Self
    println!("{:?}", p3);
}
```

### Custom Right-Hand Side Types

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, PartialEq)]
struct Meters(u32);

// Add Meters to Millimeters
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    let mm = Millimeters(1000);
    let m = Meters(1);
    let result = mm + m;
    println!("{:?}", result);  // Millimeters(2000)
}
```

## Supertraits

### Basic Supertrait Requirement

```rust
// OutlinePrint requires Display to be implemented
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();  // Can use Display methods
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

// Must implement Display first
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Now can implement OutlinePrint
impl OutlinePrint for Point {}

fn main() {
    let point = Point { x: 1, y: 3 };
    point.outline_print();
}
```

### Multiple Supertraits

```rust
use std::fmt::{Debug, Display};

trait Summary: Debug + Display + Clone {
    fn summarize(&self) -> String {
        format!("Summary of {}", self)  // Can use Display
    }
}

#[derive(Debug, Clone)]
struct Article {
    title: String,
    content: String,
}

impl Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

impl Summary for Article {}

fn main() {
    let article = Article {
        title: "Rust Programming".to_string(),
        content: "Learning Rust is fun!".to_string(),
    };
    
    println!("{}", article.summarize());
    println!("{:?}", article);  // Can use Debug
    let _cloned = article.clone();  // Can use Clone
}
```

## The Newtype Pattern

### Implementing External Traits on External Types

```rust
use std::fmt;

// Can't directly implement Display for Vec<String>
// because both are external to our crate

// Solution: Newtype wrapper
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

### Transparent Wrapper with Deref

```rust
use std::ops::Deref;

struct MyVec<T>(Vec<T>);

impl<T> MyVec<T> {
    fn new() -> MyVec<T> {
        MyVec(Vec::new())
    }
}

// Implement Deref to make it transparent
impl<T> Deref for MyVec<T> {
    type Target = Vec<T>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let mut my_vec = MyVec::new();
    my_vec.0.push(1);  // Direct access
    
    // Or use deref coercion
    let len = my_vec.len();  // Works due to Deref
    println!("Length: {}", len);
}
```

### Type Safety with Newtypes

```rust
#[derive(Debug, PartialEq, PartialOrd)]
struct UserId(u32);

#[derive(Debug, PartialEq, PartialOrd)]
struct PostId(u32);

fn get_user(id: UserId) -> String {
    format!("User {}", id.0)
}

fn get_post(id: PostId) -> String {
    format!("Post {}", id.0)
}

fn main() {
    let user_id = UserId(42);
    let post_id = PostId(42);
    
    println!("{}", get_user(user_id));
    // println!("{}", get_user(post_id));  // ❌ Compile error!
    println!("{}", get_post(post_id));
}
```

## Fully Qualified Syntax for Disambiguation

### Method Name Conflicts

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;
    
    // Calls Human's implementation
    person.fly();
    
    // Disambiguate with explicit syntax
    Pilot::fly(&person);
    Wizard::fly(&person);
    
    // Fully qualified syntax
    <Human as Pilot>::fly(&person);
    <Human as Wizard>::fly(&person);
}
```

### Associated Function Conflicts

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    // Calls Dog's implementation
    println!("A baby dog is called a {}", Dog::baby_name());
    
    // Need fully qualified syntax for trait method
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    
    // This won't work because no self parameter
    // println!("A baby dog is called a {}", Animal::baby_name());
}
```

### Complex Disambiguation

```rust
trait A {
    fn method(&self) -> String {
        String::from("A")
    }
}

trait B {
    fn method(&self) -> String {
        String::from("B")
    }
}

trait C: A + B {
    fn method(&self) -> String {
        // Need to disambiguate which parent method to call
        format!("{} and {}", A::method(self), B::method(self))
    }
}

struct MyStruct;

impl A for MyStruct {}
impl B for MyStruct {}

impl C for MyStruct {
    fn method(&self) -> String {
        format!("C: {}", <Self as A>::method(self))
    }
}

fn main() {
    let s = MyStruct;
    println!("{}", C::method(&s));
}
```

## Advanced Associated Types

### Generic Associated Types (GATs)

```rust
trait Iterator {
    type Item<'a> where Self: 'a;  // Generic associated type
    
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

struct WindowedIterator<T> {
    data: Vec<T>,
    window_size: usize,
    position: usize,
}

impl<T> Iterator for WindowedIterator<T> {
    type Item<'a> = &'a [T] where Self: 'a;
    
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.position + self.window_size <= self.data.len() {
            let window = &self.data[self.position..self.position + self.window_size];
            self.position += 1;
            Some(window)
        } else {
            None
        }
    }
}
```

### Associated Types with Bounds

```rust
trait Collect<T> {
    type Output: IntoIterator<Item = T>;  // Bound on associated type
    
    fn collect(self) -> Self::Output;
}

struct NumberCollector;

impl Collect<i32> for NumberCollector {
    type Output = Vec<i32>;  // Vec implements IntoIterator
    
    fn collect(self) -> Self::Output {
        vec![1, 2, 3, 4, 5]
    }
}

fn use_collector<C, T>(collector: C) -> impl IntoIterator<Item = T>
where
    C: Collect<T>,
{
    collector.collect()
}
```

## Trait Objects with Associated Types

### Object-Safe Traits

```rust
// ❌ Not object-safe due to associated type
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// Can't create: Box<dyn Iterator>

// ✅ Make it object-safe by constraining the associated type
trait IteratorObjectSafe {
    fn next_string(&mut self) -> Option<String>;
}

// Or use generics for object safety
trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

### Workarounds for Associated Types in Trait Objects

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// Create separate trait for dynamic dispatch
trait DynIterator {
    fn next_item(&mut self) -> Option<Box<dyn std::any::Any>>;
}

// Implement for any Iterator
impl<I: Iterator> DynIterator for I {
    fn next_item(&mut self) -> Option<Box<dyn std::any::Any>> {
        self.next().map(|item| Box::new(item) as Box<dyn std::any::Any>)
    }
}
```

## Practical Examples

### Repository Pattern with Associated Types

```rust
trait Repository {
    type Entity;
    type Id;
    type Error;
    
    fn find_by_id(&self, id: Self::Id) -> Result<Option<Self::Entity>, Self::Error>;
    fn save(&mut self, entity: Self::Entity) -> Result<Self::Id, Self::Error>;
    fn delete(&mut self, id: Self::Id) -> Result<(), Self::Error>;
}

struct User {
    id: u64,
    name: String,
}

struct UserRepository {
    users: std::collections::HashMap<u64, User>,
    next_id: u64,
}

impl Repository for UserRepository {
    type Entity = User;
    type Id = u64;
    type Error = String;
    
    fn find_by_id(&self, id: Self::Id) -> Result<Option<Self::Entity>, Self::Error> {
        Ok(self.users.get(&id).cloned())
    }
    
    fn save(&mut self, mut entity: Self::Entity) -> Result<Self::Id, Self::Error> {
        if entity.id == 0 {
            entity.id = self.next_id;
            self.next_id += 1;
        }
        
        let id = entity.id;
        self.users.insert(id, entity);
        Ok(id)
    }
    
    fn delete(&mut self, id: Self::Id) -> Result<(), Self::Error> {
        self.users.remove(&id);
        Ok(())
    }
}
```

### Builder Pattern with Associated Types

```rust
trait Builder {
    type Output;
    
    fn build(self) -> Self::Output;
}

struct UserBuilder {
    name: Option<String>,
    email: Option<String>,
    age: Option<u32>,
}

impl UserBuilder {
    fn new() -> Self {
        UserBuilder {
            name: None,
            email: None,
            age: None,
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
}

struct User {
    name: String,
    email: String,
    age: u32,
}

impl Builder for UserBuilder {
    type Output = Result<User, String>;
    
    fn build(self) -> Self::Output {
        let name = self.name.ok_or("Name is required")?;
        let email = self.email.ok_or("Email is required")?;
        let age = self.age.ok_or("Age is required")?;
        
        Ok(User { name, email, age })
    }
}

fn main() {
    let user = UserBuilder::new()
        .name("Alice")
        .email("alice@example.com")
        .age(30)
        .build();
    
    match user {
        Ok(u) => println!("Created user: {} ({})", u.name, u.age),
        Err(e) => println!("Error: {}", e),
    }
}
```

Advanced traits provide powerful abstractions while maintaining type safety and performance. They enable clean APIs and flexible designs without runtime overhead.