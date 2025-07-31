use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 17.1", "Characteristics of Object-Oriented Languages");
    
    println!("🏗️ Object-Oriented Programming in Rust");
    println!();
    
    demonstrate_encapsulation();
    demonstrate_inheritance_alternative();
    demonstrate_polymorphism();
    demonstrate_rust_oop_patterns();
}

fn demonstrate_encapsulation() {
    println!("📦 Encapsulation - Hiding Implementation Details:");
    println!();
    
    println!("Rust supports encapsulation through:");
    println!("• Public/private fields and methods");
    println!("• Module system for organizing code");
    println!("• Controlled access to internal state");
    println!();
    
    // Example of a well-encapsulated struct
    #[derive(Debug)]
    pub struct BankAccount {
        account_number: String,  // private field
        balance: f64,           // private field
    }
    
    impl BankAccount {
        // Constructor (public)
        pub fn new(account_number: String, initial_balance: f64) -> BankAccount {
            if initial_balance < 0.0 {
                panic!("Initial balance cannot be negative");
            }
            
            BankAccount {
                account_number,
                balance: initial_balance,
            }
        }
        
        // Public method to get balance (read-only access)
        pub fn balance(&self) -> f64 {
            self.balance
        }
        
        // Public method to get account number (read-only access)
        pub fn account_number(&self) -> &str {
            &self.account_number
        }
        
        // Public method with business logic
        pub fn deposit(&mut self, amount: f64) -> Result<(), String> {
            if amount <= 0.0 {
                return Err("Deposit amount must be positive".to_string());
            }
            
            self.balance += amount;
            println!("  💰 Deposited ${:.2}, new balance: ${:.2}", amount, self.balance);
            Ok(())
        }
        
        // Public method with validation
        pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
            if amount <= 0.0 {
                return Err("Withdrawal amount must be positive".to_string());
            }
            
            if amount > self.balance {
                return Err("Insufficient funds".to_string());
            }
            
            self.balance -= amount;
            println!("  💸 Withdrew ${:.2}, new balance: ${:.2}", amount, self.balance);
            Ok(())
        }
        
        // Private helper method
        fn validate_transaction(&self, amount: f64) -> bool {
            amount > 0.0 && amount <= self.balance
        }
        
        // Public method using private helper
        pub fn can_withdraw(&self, amount: f64) -> bool {
            self.validate_transaction(amount)
        }
    }
    
    println!("🏦 Bank Account Example:");
    let mut account = BankAccount::new("12345".to_string(), 100.0);
    
    println!("Created account: {}", account.account_number());
    println!("Initial balance: ${:.2}", account.balance());
    
    // These work - public interface
    account.deposit(50.0).unwrap();
    account.withdraw(25.0).unwrap();
    
    println!("Can withdraw $200? {}", account.can_withdraw(200.0));
    println!("Can withdraw $50? {}", account.can_withdraw(50.0));
    
    // These would not compile - private fields
    // println!("{}", account.balance); // Error: field is private
    // account.balance = 1000.0;        // Error: field is private
    
    println!();
    println!("✅ Encapsulation Benefits:");
    println!("• Data integrity through controlled access");
    println!("• Business logic enforcement");
    println!("• Internal implementation can change without breaking clients");
    println!("• Clear public API");
    println!();
}

fn demonstrate_inheritance_alternative() {
    println!("🧬 Inheritance vs Composition in Rust:");
    println!();
    
    println!("Rust doesn't have classical inheritance, but provides:");
    println!("• Composition - structs containing other structs");
    println!("• Traits - shared behavior");
    println!("• Default implementations - code reuse");
    println!();
    
    // Base functionality through traits
    trait Drawable {
        fn draw(&self);
        
        // Default implementation
        fn describe(&self) -> String {
            "A drawable object".to_string()
        }
    }
    
    trait Movable {
        fn move_to(&mut self, x: f64, y: f64);
        fn get_position(&self) -> (f64, f64);
    }
    
    // Composition: Position as a separate struct
    #[derive(Debug, Clone)]
    struct Position {
        x: f64,
        y: f64,
    }
    
    impl Position {
        fn new(x: f64, y: f64) -> Self {
            Position { x, y }
        }
    }
    
    // Different shapes using composition and traits
    struct Circle {
        position: Position,
        radius: f64,
    }
    
    impl Circle {
        fn new(x: f64, y: f64, radius: f64) -> Self {
            Circle {
                position: Position::new(x, y),
                radius,
            }
        }
    }
    
    impl Drawable for Circle {
        fn draw(&self) {
            println!("  🔵 Drawing circle at ({:.1}, {:.1}) with radius {:.1}", 
                    self.position.x, self.position.y, self.radius);
        }
        
        fn describe(&self) -> String {
            format!("Circle with radius {:.1}", self.radius)
        }
    }
    
    impl Movable for Circle {
        fn move_to(&mut self, x: f64, y: f64) {
            println!("  🔵 Moving circle from ({:.1}, {:.1}) to ({:.1}, {:.1})", 
                    self.position.x, self.position.y, x, y);
            self.position.x = x;
            self.position.y = y;
        }
        
        fn get_position(&self) -> (f64, f64) {
            (self.position.x, self.position.y)
        }
    }
    
    struct Rectangle {
        position: Position,
        width: f64,
        height: f64,
    }
    
    impl Rectangle {
        fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
            Rectangle {
                position: Position::new(x, y),
                width,
                height,
            }
        }
    }
    
    impl Drawable for Rectangle {
        fn draw(&self) {
            println!("  🟦 Drawing rectangle at ({:.1}, {:.1}) with size {:.1}x{:.1}", 
                    self.position.x, self.position.y, self.width, self.height);
        }
        
        fn describe(&self) -> String {
            format!("Rectangle {}x{}", self.width, self.height)
        }
    }
    
    impl Movable for Rectangle {
        fn move_to(&mut self, x: f64, y: f64) {
            println!("  🟦 Moving rectangle from ({:.1}, {:.1}) to ({:.1}, {:.1})", 
                    self.position.x, self.position.y, x, y);
            self.position.x = x;
            self.position.y = y;
        }
        
        fn get_position(&self) -> (f64, f64) {
            (self.position.x, self.position.y)
        }
    }
    
    println!("🎨 Shape System Example:");
    
    let mut circle = Circle::new(0.0, 0.0, 5.0);
    let mut rectangle = Rectangle::new(10.0, 10.0, 8.0, 6.0);
    
    println!("Initial shapes:");
    println!("  {}: {}", circle.describe(), circle.describe());
    circle.draw();
    
    println!("  {}: {}", rectangle.describe(), rectangle.describe());
    rectangle.draw();
    
    println!();
    println!("Moving shapes:");
    circle.move_to(3.0, 4.0);
    rectangle.move_to(15.0, 20.0);
    
    println!();
    println!("Drawing moved shapes:");
    circle.draw();
    rectangle.draw();
    
    println!();
    println!("🔧 Composition Benefits:");
    println!("• Explicit relationships - no hidden inheritance");
    println!("• Flexible - can combine different components");
    println!("• Testable - components can be tested independently");
    println!("• Maintainable - changes are localized");
    println!();
}

fn demonstrate_polymorphism() {
    println!("🎭 Polymorphism with Trait Objects:");
    println!();
    
    println!("Rust achieves polymorphism through:");
    println!("• Static dispatch - generics with trait bounds");
    println!("• Dynamic dispatch - trait objects (dyn Trait)");
    println!();
    
    // Define a common interface
    trait Animal {
        fn name(&self) -> &str;
        fn make_sound(&self) -> &str;
        
        fn introduce(&self) {
            println!("  🐾 Hi, I'm {} and I go '{}'", self.name(), self.make_sound());
        }
    }
    
    // Different implementations
    struct Dog {
        name: String,
    }
    
    impl Dog {
        fn new(name: &str) -> Self {
            Dog { name: name.to_string() }
        }
    }
    
    impl Animal for Dog {
        fn name(&self) -> &str {
            &self.name
        }
        
        fn make_sound(&self) -> &str {
            "Woof!"
        }
    }
    
    struct Cat {
        name: String,
    }
    
    impl Cat {
        fn new(name: &str) -> Self {
            Cat { name: name.to_string() }
        }
    }
    
    impl Animal for Cat {
        fn name(&self) -> &str {
            &self.name
        }
        
        fn make_sound(&self) -> &str {
            "Meow!"
        }
    }
    
    struct Cow {
        name: String,
    }
    
    impl Cow {
        fn new(name: &str) -> Self {
            Cow { name: name.to_string() }
        }
    }
    
    impl Animal for Cow {
        fn name(&self) -> &str {
            &self.name
        }
        
        fn make_sound(&self) -> &str {
            "Moo!"
        }
    }
    
    println!("🐕 Static Dispatch with Generics:");
    
    fn pet_animal<T: Animal>(animal: &T) {
        println!("  ✋ Petting {}", animal.name());
        animal.introduce();
    }
    
    let dog = Dog::new("Buddy");
    let cat = Cat::new("Whiskers");
    
    pet_animal(&dog);
    pet_animal(&cat);
    
    println!();
    println!("🎪 Dynamic Dispatch with Trait Objects:");
    
    // Vector of trait objects - different types, same interface
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog::new("Rex")),
        Box::new(Cat::new("Mittens")),
        Box::new(Cow::new("Bessie")),
    ];
    
    println!("  🎵 Animal chorus:");
    for animal in &animals {
        animal.introduce();
    }
    
    // Function accepting trait objects
    fn make_animals_perform(animals: &[Box<dyn Animal>]) {
        println!("  🎪 Performance time!");
        for (i, animal) in animals.iter().enumerate() {
            println!("    Act {}: {}", i + 1, animal.name());
            println!("    Sound: {}", animal.make_sound());
        }
    }
    
    make_animals_perform(&animals);
    
    println!();
    println!("⚡ Static vs Dynamic Dispatch:");
    println!("Static Dispatch (Generics):");
    println!("  • Compile-time polymorphism");
    println!("  • Zero runtime cost");
    println!("  • Code specialization for each type");
    println!("  • Larger binary size");
    println!();
    println!("Dynamic Dispatch (Trait Objects):");
    println!("  • Runtime polymorphism");
    println!("  • Small runtime cost (vtable lookup)");
    println!("  • Same code for all types");
    println!("  • Smaller binary size");
    println!("  • Enables collections of different types");
    println!();
}

fn demonstrate_rust_oop_patterns() {
    println!("🦀 Rust-Specific OOP Patterns:");
    println!();
    
    // Builder pattern
    println!("1. 🏗️ Builder Pattern:");
    
    #[derive(Debug)]
    struct HttpRequest {
        method: String,
        url: String,
        headers: Vec<(String, String)>,
        body: Option<String>,
    }
    
    struct HttpRequestBuilder {
        method: Option<String>,
        url: Option<String>,
        headers: Vec<(String, String)>,
        body: Option<String>,
    }
    
    impl HttpRequestBuilder {
        fn new() -> Self {
            HttpRequestBuilder {
                method: None,
                url: None,
                headers: Vec::new(),
                body: None,
            }
        }
        
        fn method(mut self, method: &str) -> Self {
            self.method = Some(method.to_string());
            self
        }
        
        fn url(mut self, url: &str) -> Self {
            self.url = Some(url.to_string());
            self
        }
        
        fn header(mut self, key: &str, value: &str) -> Self {
            self.headers.push((key.to_string(), value.to_string()));
            self
        }
        
        fn body(mut self, body: &str) -> Self {
            self.body = Some(body.to_string());
            self
        }
        
        fn build(self) -> Result<HttpRequest, String> {
            let method = self.method.ok_or("Method is required")?;
            let url = self.url.ok_or("URL is required")?;
            
            Ok(HttpRequest {
                method,
                url,
                headers: self.headers,
                body: self.body,
            })
        }
    }
    
    let request = HttpRequestBuilder::new()
        .method("POST")
        .url("https://api.example.com/users")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token123")
        .body(r#"{"name": "John", "email": "john@example.com"}"#)
        .build()
        .unwrap();
    
    println!("  Built request: {:#?}", request);
    
    println!();
    println!("2. 🎯 State Pattern with Type System:");
    
    // States as types
    struct Draft;
    struct PendingReview;
    struct Published;
    
    struct Post<State> {
        content: String,
        state: State,
    }
    
    impl Post<Draft> {
        fn new() -> Post<Draft> {
            Post {
                content: String::new(),
                state: Draft,
            }
        }
        
        fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }
        
        fn request_review(self) -> Post<PendingReview> {
            Post {
                content: self.content,
                state: PendingReview,
            }
        }
    }
    
    impl Post<PendingReview> {
        fn approve(self) -> Post<Published> {
            Post {
                content: self.content,
                state: Published,
            }
        }
        
        fn reject(self) -> Post<Draft> {
            Post {
                content: self.content,
                state: Draft,
            }
        }
    }
    
    impl Post<Published> {
        fn content(&self) -> &str {
            &self.content
        }
    }
    
    println!("  📝 Blog post workflow:");
    let mut post = Post::new();
    post.add_text("This is my first blog post!");
    println!("    Created draft post");
    
    let post = post.request_review();
    println!("    Requested review");
    
    let post = post.approve();
    println!("    Approved for publishing");
    println!("    Published content: '{}'", post.content());
    
    // These operations are not available in wrong states (compile-time safety):
    // post.add_text("more text"); // Error: method not available on Published
    // let content = draft_post.content(); // Error: method not available on Draft
    
    println!();
    println!("🌟 Rust OOP Advantages:");
    println!("• Zero-cost abstractions");
    println!("• Memory safety without garbage collector");
    println!("• Compile-time enforcement of invariants");
    println!("• Explicit control over data layout and access");
    println!("• Composition over inheritance promotes flexibility");
    println!("• Trait system enables powerful abstractions");
    
    println!();
    println!("🎯 When to Use OOP Patterns in Rust:");
    println!("• Encapsulation: Always - use pub/private visibility");
    println!("• Inheritance: Rarely - prefer composition and traits");
    println!("• Polymorphism: Often - use traits and generics");
    println!("• Design patterns: Adapt to Rust's ownership model");
}