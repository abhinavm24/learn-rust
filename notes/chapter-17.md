# Chapter 17: Object-Oriented Programming Features

## Key Takeaways

### Core Concepts
- **Rust's OOP Philosophy**: Composition over inheritance with trait-based polymorphism
- **Encapsulation Through Privacy**: Module system and visibility controls for data hiding
- **Polymorphism via Traits**: Interface-based programming without traditional inheritance
- **Dynamic Dispatch**: Runtime method resolution through trait objects
- **Object Safety**: Constraints on traits that can be used as trait objects
- **State Pattern**: Encoding state machines through type-safe state transitions

### Important Syntax and Operators
- `pub struct Name { private_field: Type }` - Encapsulation with public/private fields
- `Box<dyn Trait>` - Trait objects for dynamic dispatch
- `impl Trait for Type` - Implementing behavior for types
- `&dyn Trait` - Reference to trait object
- `Vec<Box<dyn Trait>>` - Collections of different types sharing common behavior
- `self: Box<Self>` - Taking ownership of boxed self in trait methods
- `<'a>` - Lifetime parameters in trait method signatures

### Programming Concepts Introduced
- **Interface-Based Design**: Programming to interfaces rather than concrete implementations
- **Runtime Polymorphism**: Method resolution determined at runtime through vtables
- **Type Erasure**: Abstracting away concrete types behind trait interfaces
- **State Machine Encoding**: Using types and traits to represent state transitions
- **Plugin Architecture**: Extensible systems where behavior can be added dynamically

## Code Examples and Patterns

### Encapsulation and Data Abstraction
```rust
pub struct AveragedCollection {
    list: Vec<i32>,        // Private - internal implementation detail
    average: f64,          // Private - cached calculation
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }

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

    // Private method - implementation detail
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn encapsulation_example() {
    let mut collection = AveragedCollection::new();
    collection.add(1);
    collection.add(2);
    collection.add(3);
    
    println!("Average: {}", collection.average());
    // collection.list is not accessible - encapsulation enforced
}
```

### Polymorphism with Trait Objects
```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();  // Dynamic dispatch
        }
    }
}

// Different types implementing the same trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button: {} ({}x{})", self.label, self.width, self.height);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing select box ({}x{}) with {} options", 
                 self.width, self.height, self.options.len());
    }
}

fn polymorphism_example() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();  // Calls draw() on each component polymorphically
}
```

### State Pattern Implementation
```rust
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""  // Default implementation returns empty string
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self  // Stay in Draft state
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self  // Stay in PendingReview state
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self  // Stay in Published state
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self  // Stay in Published state
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content  // Only published posts show content
    }
}

fn state_pattern_example() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());  // Draft posts show no content

    post.request_review();
    assert_eq!("", post.content());  // Pending review posts show no content

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());  // Published!
}
```

### Advanced Trait Object Patterns
```rust
// Plugin system using trait objects
pub trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self, input: &str) -> String;
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager { plugins: vec![] }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn execute_all(&self, input: &str) -> Vec<String> {
        self.plugins
            .iter()
            .map(|plugin| {
                println!("Executing plugin: {}", plugin.name());
                plugin.execute(input)
            })
            .collect()
    }
}

// Concrete plugin implementations
struct UppercasePlugin;

impl Plugin for UppercasePlugin {
    fn name(&self) -> &str {
        "Uppercase Converter"
    }

    fn execute(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

struct ReversePlugin;

impl Plugin for ReversePlugin {
    fn name(&self) -> &str {
        "String Reverser"
    }

    fn execute(&self, input: &str) -> String {
        input.chars().rev().collect()
    }
}

fn plugin_system_example() {
    let mut manager = PluginManager::new();
    
    manager.register(Box::new(UppercasePlugin));
    manager.register(Box::new(ReversePlugin));
    
    let results = manager.execute_all("Hello World");
    for result in results {
        println!("Result: {}", result);
    }
}
```

## Practical Applications
- Building extensible application architectures with plugin systems
- Implementing GUI frameworks with different widget types
- Creating game engines with polymorphic entity behavior
- Developing compiler architectures with different AST node types
- Building web frameworks with middleware chains
- Implementing design patterns like Strategy, State, and Command

## Integration with Previous Chapters
- **Prerequisites**: Traits (Chapter 10), smart pointers (Chapter 15, especially Box<T>)
- **Builds On**: Module system (Chapter 7) for encapsulation, error handling (Chapter 9) for robust APIs
- **Connections**: Enables complex system design, prepares for advanced architectural patterns

## Community Conventions and Idioms
- Prefer composition over inheritance - use traits to define shared behavior
- Use trait objects when you need heterogeneous collections of types
- Keep trait objects object-safe by avoiding Self return types and generics
- Use `Box<dyn Trait>` for owned trait objects, `&dyn Trait` for borrowed ones
- Consider static dispatch (generics) over dynamic dispatch when performance is critical
- Design traits with focused, cohesive responsibilities (Interface Segregation Principle)

## Personal Notes
- Rust's approach to OOP is different but more flexible than traditional inheritance
- Trait objects come with runtime cost - use judiciously in performance-critical code
- The state pattern in Rust can be more type-safe than in other languages
- Learning to think in terms of traits rather than inheritance is a mental shift
- Dynamic dispatch enables powerful plugin architectures and extensible systems
- Object safety rules can be restrictive but prevent runtime errors common in other languages

Official Chapter: https://doc.rust-lang.org/book/ch17-00-oop.html

---
*Completed: ✓*