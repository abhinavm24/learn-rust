# Chapter 17.3: Implementing an Object-Oriented Design Pattern

## Key Takeaways
- **State Pattern**: Encapsulates varying behavior based on internal state changes
- **Rust Implementation**: Uses trait objects and state transitions to model state machines
- **Type-Driven Design**: Alternative approach using Rust's type system for compile-time state enforcement
- **Trade-offs**: Runtime flexibility vs compile-time safety and performance

## The State Pattern

### Blog Post State Machine
A blog post can be in different states with different behaviors:
- **Draft**: Can be edited, not visible publicly
- **PendingReview**: Cannot be edited, awaiting approval
- **Published**: Cannot be edited, visible publicly

### Traditional State Pattern Implementation

#### State Trait Definition
```rust
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""  // Default implementation returns empty string
    }
}
```

#### Post Implementation
```rust
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
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}
```

#### State Implementations

##### Draft State
```rust
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self  // No change - drafts can't be approved directly
    }
}
```

##### PendingReview State
```rust
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self  // Already pending review
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}
```

##### Published State
```rust
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self  // Published posts don't go back to review
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self  // Already approved
    }
    
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content  // Only published posts show content
    }
}
```

### Using the State Pattern
```rust
fn main() {
    let mut post = Post::new();
    
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());  // Draft state - no content shown
    
    post.request_review();
    assert_eq!("", post.content());  // Pending review - no content shown
    
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());  // Published - content shown
}
```

## Enhanced State Pattern

### Adding Rejection Capability
```rust
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;  // New method
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

impl Post {
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

// Update PendingReview to handle rejection
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})  // Rejected posts go back to draft
    }
}
```

### Multiple Approval Requirements
```rust
struct PendingReview {
    approvals: u32,
}

impl State for PendingReview {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.approvals >= 1 {  // Require 2 approvals
            Box::new(Published {})
        } else {
            Box::new(PendingReview {
                approvals: self.approvals + 1,
            })
        }
    }
    
    // ... other methods
}
```

## Type-Driven State Pattern

### Using Types to Encode States
```rust
pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

pub struct Post {
    content: String,
}

impl DraftPost {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
    
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

impl Post {
    pub fn content(&self) -> &str {
        &self.content
    }
}
```

### Using Type-Driven Approach
```rust
fn main() {
    let mut post = DraftPost::new();
    
    post.add_text("I ate a salad for lunch today");
    
    let post = post.request_review();  // Consumes DraftPost, returns PendingReviewPost
    
    let post = post.approve();  // Consumes PendingReviewPost, returns Post
    
    assert_eq!("I ate a salad for lunch today", post.content());
}

// Compile-time error prevention:
// let mut post = DraftPost::new();
// post.approve();  // ❌ Compile error - DraftPost doesn't have approve method
```

## Advanced State Patterns

### State with Additional Data
```rust
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str { "" }
    fn edit_allowed(&self) -> bool { false }
    fn state_name(&self) -> &'static str;
}

struct Draft {
    edit_count: u32,
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { reviewer: None })
    }
    
    fn edit_allowed(&self) -> bool {
        true
    }
    
    fn state_name(&self) -> &'static str {
        "Draft"
    }
}

struct PendingReview {
    reviewer: Option<String>,
}

impl State for PendingReview {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published { 
            published_date: std::time::SystemTime::now(),
        })
    }
    
    fn state_name(&self) -> &'static str {
        "Pending Review"
    }
}

struct Published {
    published_date: std::time::SystemTime,
}

impl State for Published {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    
    fn state_name(&self) -> &'static str {
        "Published"
    }
}
```

## Comparison: Runtime vs Compile-time States

### Runtime State Pattern (Trait Objects)
**Advantages:**
- Flexible state transitions at runtime
- Easy to add new states
- Familiar OOP pattern

**Disadvantages:**
- Runtime overhead (dynamic dispatch)
- Possible runtime errors
- Less type safety

### Compile-time State Pattern (Types)  
**Advantages:**
- Zero runtime cost
- Compile-time state verification
- Impossible states are unrepresentable

**Disadvantages:**
- Less flexible
- More complex type signatures
- Harder to add dynamic behavior

## Integration with Previous Concepts
- **Ownership**: State transitions consume and return owned values
- **Trait Objects**: Enable runtime polymorphism for state behavior
- **Pattern Matching**: Can be used with enums for simpler state machines
- **Error Handling**: State transitions can return `Result<State, Error>`

## Best Practices
1. **Choose Appropriate Pattern**: Runtime flexibility vs compile-time safety
2. **Model Invalid States**: Make impossible states unrepresentable
3. **Clear State Transitions**: Document allowed state changes
4. **Handle Edge Cases**: Consider all possible state transition scenarios
5. **Use Enums for Simple States**: For finite, known state sets

## Alternative: Enum-Based State Machine
```rust
#[derive(Debug, PartialEq)]
pub enum PostState {
    Draft,
    PendingReview,
    Published,
}

pub struct Post {
    state: PostState,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: PostState::Draft,
            content: String::new(),
        }
    }
    
    pub fn request_review(&mut self) {
        match self.state {
            PostState::Draft => self.state = PostState::PendingReview,
            _ => {} // No change for other states
        }
    }
    
    pub fn approve(&mut self) {
        match self.state {
            PostState::PendingReview => self.state = PostState::Published,
            _ => {} // No change for other states
        }
    }
    
    pub fn content(&self) -> &str {
        match self.state {
            PostState::Published => &self.content,
            _ => "",
        }
    }
}
```

This approach to state management demonstrates how Rust can implement traditional OOP patterns while providing more type-safe alternatives.