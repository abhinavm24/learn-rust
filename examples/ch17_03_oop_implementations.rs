//! Chapter 17.3: Implementing an Object-Oriented Design Pattern
//! 
//! This example demonstrates:
//! - State pattern implementation in Rust
//! - Encapsulation and internal state management
//! - Type-safe state transitions
//! - Alternative approaches to OOP patterns in Rust

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 17.3", "Implementing an Object-Oriented Design Pattern");
    
    println!("=== Traditional State Pattern ===");
    traditional_state_pattern();
    
    println!("\n=== Rust-idiomatic State Pattern ===");
    rust_idiomatic_state_pattern();
    
    println!("\n=== Type-Safe State Machine ===");
    type_safe_state_machine();
    
    println!("\n=== Enum-Based State Pattern ===");
    enum_based_state_pattern();
}

// Traditional OOP State Pattern Implementation
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str;
    fn reject(self: Box<Self>) -> Box<dyn State>;
}

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
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
    
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
    
    pub fn get_state_name(&self) -> &str {
        match self.state.as_ref() {
            Some(_state) => {
                // Simplified state name determination
                "Draft/PendingReview/Published"
            }
            None => "Unknown"
        }
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // Can't approve a draft
    }
    
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        "" // Draft content is not visible
    }
    
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self // Can't reject a draft
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // Already pending review
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        "" // Pending content is not visible
    }
    
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // Can't request review of published post
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // Already approved
    }
    
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

fn traditional_state_pattern() {
    let mut post = Post::new();
    
    post.add_text("I ate a salad for lunch today");
    println!("Initial state: {}", post.get_state_name());
    println!("Content (should be empty): '{}'", post.content());
    
    post.request_review();
    println!("After request review: {}", post.get_state_name());
    println!("Content (should be empty): '{}'", post.content());
    
    post.approve();
    println!("After approve: {}", post.get_state_name());
    println!("Content (should be visible): '{}'", post.content());
    
    // Test rejection workflow
    let mut post2 = Post::new();
    post2.add_text("This post will be rejected");
    post2.request_review();
    post2.reject();
    println!("After rejection, state: {}", post2.get_state_name());
}

// Rust-idiomatic State Pattern (encoding states in the type system)
pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

pub struct PublishedPost {
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
    pub fn approve(self) -> PublishedPost {
        PublishedPost {
            content: self.content,
        }
    }
    
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

impl PublishedPost {
    pub fn content(&self) -> &str {
        &self.content
    }
}

fn rust_idiomatic_state_pattern() {
    let mut post = DraftPost::new();
    post.add_text("I ate a salad for lunch today");
    
    let post = post.request_review();
    println!("Post is now in pending review state");
    
    let post = post.approve();
    println!("Published content: '{}'", post.content());
    
    // Demonstrate rejection workflow
    let mut post2 = DraftPost::new();
    post2.add_text("This will be rejected");
    let post2 = post2.request_review();
    let post2 = post2.reject(); // Back to draft
    // post2.content(); // This would be a compile error - draft has no content method
    println!("Post rejected and back to draft state");
}

// Type-safe state machine with more complex transitions
#[derive(Debug, Clone)]
pub struct StateMachine<S> {
    state: S,
    data: String,
}

#[derive(Debug, Clone)]
pub struct Idle;

#[derive(Debug, Clone)]
pub struct Processing;

#[derive(Debug, Clone)]
pub struct Completed;

#[derive(Debug, Clone)]
pub struct Failed {
    error: String,
}

impl StateMachine<Idle> {
    pub fn new(data: String) -> Self {
        StateMachine {
            state: Idle,
            data,
        }
    }
    
    pub fn start_processing(self) -> StateMachine<Processing> {
        println!("Starting to process: {}", self.data);
        StateMachine {
            state: Processing,
            data: self.data,
        }
    }
}

impl StateMachine<Processing> {
    pub fn complete(self) -> StateMachine<Completed> {
        println!("Processing completed for: {}", self.data);
        StateMachine {
            state: Completed,
            data: self.data,
        }
    }
    
    pub fn fail(self, error: String) -> StateMachine<Failed> {
        println!("Processing failed for: {} with error: {}", self.data, error);
        StateMachine {
            state: Failed { error },
            data: self.data,
        }
    }
}

impl StateMachine<Completed> {
    pub fn get_result(&self) -> &str {
        &self.data
    }
    
    pub fn reset(self) -> StateMachine<Idle> {
        StateMachine {
            state: Idle,
            data: self.data,
        }
    }
}

impl StateMachine<Failed> {
    pub fn get_error(&self) -> &str {
        &self.state.error
    }
    
    pub fn retry(self) -> StateMachine<Processing> {
        println!("Retrying processing for: {}", self.data);
        StateMachine {
            state: Processing,
            data: self.data,
        }
    }
    
    pub fn reset(self) -> StateMachine<Idle> {
        StateMachine {
            state: Idle,
            data: self.data,
        }
    }
}

fn type_safe_state_machine() {
    let machine = StateMachine::new("important data".to_string());
    let machine = machine.start_processing();
    
    // Simulate successful completion
    let machine = machine.complete();
    println!("Result: {}", machine.get_result());
    
    // Reset and try failure path
    let machine = machine.reset();
    let machine = machine.start_processing();
    let machine = machine.fail("network error".to_string());
    println!("Error: {}", machine.get_error());
    
    // Retry after failure
    let machine = machine.retry();
    let machine = machine.complete();
    println!("Retry successful, result: {}", machine.get_result());
}

// Enum-based state pattern (simpler but less type-safe)
#[derive(Debug, Clone)]
pub enum BlogPostState {
    Draft,
    PendingReview,
    Published,
}

#[derive(Debug)]
pub struct BlogPost {
    state: BlogPostState,
    content: String,
}

impl BlogPost {
    pub fn new() -> Self {
        BlogPost {
            state: BlogPostState::Draft,
            content: String::new(),
        }
    }
    
    pub fn add_text(&mut self, text: &str) {
        match self.state {
            BlogPostState::Draft => {
                self.content.push_str(text);
            }
            _ => {
                println!("Cannot add text to post in {:?} state", self.state);
            }
        }
    }
    
    pub fn request_review(&mut self) {
        self.state = match self.state {
            BlogPostState::Draft => BlogPostState::PendingReview,
            _ => {
                println!("Cannot request review from {:?} state", self.state);
                self.state.clone()
            }
        };
    }
    
    pub fn approve(&mut self) {
        self.state = match self.state {
            BlogPostState::PendingReview => BlogPostState::Published,
            _ => {
                println!("Cannot approve from {:?} state", self.state);
                self.state.clone()
            }
        };
    }
    
    pub fn reject(&mut self) {
        self.state = match self.state {
            BlogPostState::PendingReview | BlogPostState::Published => BlogPostState::Draft,
            _ => {
                println!("Cannot reject from {:?} state", self.state);
                self.state.clone()
            }
        };
    }
    
    pub fn content(&self) -> &str {
        match self.state {
            BlogPostState::Published => &self.content,
            _ => "",
        }
    }
    
    pub fn get_state(&self) -> &BlogPostState {
        &self.state
    }
}

fn enum_based_state_pattern() {
    let mut post = BlogPost::new();
    
    post.add_text("I learned about state patterns today");
    println!("State: {:?}", post.get_state());
    println!("Content: '{}'", post.content());
    
    post.request_review();
    println!("State after review request: {:?}", post.get_state());
    
    // Try to add text (should fail)
    post.add_text(" - Additional text");
    
    post.approve();
    println!("State after approval: {:?}", post.get_state());
    println!("Published content: '{}'", post.content());
    
    // Test rejection
    post.reject();
    println!("State after rejection: {:?}", post.get_state());
    println!("Content after rejection: '{}'", post.content());
}

// Additional pattern: State with data transformation
#[derive(Debug)]
pub struct DataProcessor<T> {
    data: T,
}

impl DataProcessor<String> {
    pub fn new(data: String) -> Self {
        DataProcessor { data }
    }
    
    pub fn parse_to_number(self) -> Result<DataProcessor<i32>, String> {
        match self.data.parse::<i32>() {
            Ok(num) => Ok(DataProcessor { data: num }),
            Err(_) => Err(format!("Failed to parse '{}' as number", self.data)),
        }
    }
}

impl DataProcessor<i32> {
    pub fn double(self) -> DataProcessor<i32> {
        DataProcessor {
            data: self.data * 2,
        }
    }
    
    pub fn to_string(self) -> DataProcessor<String> {
        DataProcessor {
            data: self.data.to_string(),
        }
    }
    
    pub fn get_value(&self) -> i32 {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traditional_state_pattern() {
        let mut post = Post::new();
        post.add_text("Test content");
        
        assert_eq!(post.content(), "");
        
        post.request_review();
        assert_eq!(post.content(), "");
        
        post.approve();
        assert_eq!(post.content(), "Test content");
    }

    #[test]
    fn test_rust_idiomatic_pattern() {
        let mut post = DraftPost::new();
        post.add_text("Test content");
        
        let post = post.request_review();
        let post = post.approve();
        
        assert_eq!(post.content(), "Test content");
    }

    #[test]
    fn test_state_machine() {
        let machine = StateMachine::new("test".to_string());
        let machine = machine.start_processing();
        let machine = machine.complete();
        
        assert_eq!(machine.get_result(), "test");
    }

    #[test]
    fn test_enum_based_pattern() {
        let mut post = BlogPost::new();
        post.add_text("Test");
        post.request_review();
        post.approve();
        
        assert_eq!(post.content(), "Test");
        assert!(matches!(post.get_state(), BlogPostState::Published));
    }

    #[test]
    fn test_data_transformation() {
        let processor = DataProcessor::new("42".to_string());
        let processor = processor.parse_to_number().unwrap();
        let processor = processor.double();
        
        assert_eq!(processor.get_value(), 84);
    }
}