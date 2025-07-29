# Chapter 10.2: Traits - Defining Shared Behavior

## Key Takeaways

### Trait Fundamentals
- **Traits**: Define shared behavior that multiple types can implement
- **Interface-like**: Similar to interfaces in other languages but more powerful
- **Composition**: Enable composition over inheritance patterns
- **Compile-time Polymorphism**: Type checking ensures trait methods exist

### Trait Benefits
- **Code Reuse**: Define behavior once, implement for many types
- **Abstraction**: Program against interfaces, not concrete types
- **Extensibility**: Add new behavior to existing types
- **Generic Constraints**: Specify what capabilities a generic type must have

### Important Syntax

```rust
// Define trait
trait TraitName {
    fn method_name(&self) -> ReturnType;
}

// Implement trait
impl TraitName for TypeName {
    fn method_name(&self) -> ReturnType {
        // implementation
    }
}

// Trait bounds
fn function<T: TraitName>(param: T) {}
fn function(param: impl TraitName) {}
```

### Code Examples and Patterns

#### Basic Trait Definition and Implementation
```rust
pub trait Summary {
    fn summarize(&self) -> String;
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

#### Default Implementations
```rust
pub trait Summary {
    fn summarize_author(&self) -> String;
    
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

#### Traits as Parameters
```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Equivalent trait bound syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
pub fn notify(item: &(impl Summary + Display)) {
    println!("{}", item);
}

// Where clauses for complex bounds
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // implementation
}
```

#### Returning Trait Objects
```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```

#### Blanket Implementations
```rust
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        // implementation that calls fmt::Display
    }
}
```

### Practical Applications
- Defining common behavior across types (Display, Debug, Clone)
- Generic programming with trait bounds
- Plugin systems and extensible architectures
- API design with flexible parameter types
- Standard library traits (Iterator, From, Into)

### Integration with Previous Chapters
- Works with generics for flexible, reusable code
- Enables safe generic programming with constraints
- Used extensively in error handling and collections
- Foundation for advanced Rust patterns

### Community Conventions
- Use descriptive trait names (Summary, Display, Iterator)
- Keep traits focused on single responsibilities
- Provide default implementations when sensible
- Use trait bounds to make generic code more expressive

Official Chapter: https://doc.rust-lang.org/book/ch10-02-traits.html

---
*Completed: ✓*