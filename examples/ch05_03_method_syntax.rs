use rust_book_examples::print_chapter_header;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// First impl block - basic operations
impl Rectangle {
    // Associated function (constructor)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    // Another constructor for squares
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // Method with immutable borrow - most common
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method with immutable borrow
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    // Method that takes another Rectangle parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    fn is_larger_than(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
}

// Second impl block - modification methods
impl Rectangle {
    // Method with mutable borrow
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
    
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    
    fn set_height(&mut self, height: u32) {
        self.height = height;
    }
    
    // Method that takes ownership (consumes self)
    fn destroy(self) -> String {
        format!("Destroying rectangle {}x{} with area {}", 
                self.width, self.height, self.area())
    }
}

// Third impl block - method chaining pattern
impl Rectangle {
    fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    
    fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
    
    fn doubled(mut self) -> Self {
        self.width *= 2;
        self.height *= 2;
        self
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

impl User {
    // Associated function constructor
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            active: true,
            sign_in_count: 0,
        }
    }
    
    // Method with mutable borrow
    fn sign_in(&mut self) {
        self.sign_in_count += 1;
        println!("{} signed in (count: {})", self.username, self.sign_in_count);
    }
    
    fn deactivate(&mut self) {
        self.active = false;
        println!("{} has been deactivated", self.username);
    }
    
    fn activate(&mut self) {
        self.active = true;
        println!("{} has been activated", self.username);
    }
    
    // Method with immutable borrow
    fn is_active(&self) -> bool {
        self.active
    }
    
    fn get_display_name(&self) -> &str {
        &self.username
    }
    
    fn change_email(&mut self, new_email: String) {
        println!("Changing email from {} to {}", self.email, new_email);
        self.email = new_email;
    }
}

fn main() {
    print_chapter_header("Chapter 5.3", "Method Syntax");
    
    // === ASSOCIATED FUNCTIONS (CONSTRUCTORS) ===
    println!("\n=== Associated Functions (Constructors) ===");
    let rect1 = Rectangle::new(30, 50);
    let square = Rectangle::square(25);
    println!("rect1: {:?}", rect1);
    println!("square: {:?}", square);
    
    // === BASIC METHODS ===
    println!("\n=== Basic Methods ===");
    println!("rect1 area: {}", rect1.area());
    println!("rect1 perimeter: {}", rect1.perimeter());
    println!("square area: {}", square.area());
    
    // === METHODS WITH PARAMETERS ===
    println!("\n=== Methods with Parameters ===");
    println!("Can rect1 hold square? {}", rect1.can_hold(&square));
    println!("Is rect1 larger than square? {}", rect1.is_larger_than(&square));
    
    let small_rect = Rectangle::new(10, 15);
    println!("Can rect1 hold small_rect? {}", rect1.can_hold(&small_rect));
    
    // === MUTABLE METHODS ===
    println!("\n=== Mutable Methods ===");
    let mut rect2 = Rectangle::new(20, 30);
    println!("rect2 before: {:?}", rect2);
    
    rect2.double_size();
    println!("rect2 after doubling: {:?}", rect2);
    
    rect2.set_width(100);
    rect2.set_height(80);
    println!("rect2 after setting dimensions: {:?}", rect2);
    
    // === METHOD CHAINING ===
    println!("\n=== Method Chaining ===");
    let rect3 = Rectangle::new(5, 10)
        .with_width(15)
        .doubled()
        .with_height(50);
    println!("rect3 after chaining: {:?}", rect3);
    
    // === AUTOMATIC REFERENCING ===
    println!("\n=== Automatic Referencing and Dereferencing ===");
    let rect4 = Rectangle::new(12, 18);
    let rect4_ref = &rect4;
    let rect4_box = Box::new(Rectangle::new(12, 18));
    
    // All of these work the same way due to automatic referencing!
    println!("Direct call: {}", rect4.area());
    println!("Through reference: {}", rect4_ref.area());
    println!("Through Box: {}", rect4_box.area());
    
    // === USER EXAMPLE ===
    println!("\n=== User Example ===");
    let mut user = User::new(
        String::from("alice123"),
        String::from("alice@example.com")
    );
    
    println!("New user: {:#?}", user);
    
    user.sign_in();
    user.sign_in();
    user.change_email(String::from("alice.new@example.com"));
    
    println!("Is active: {}", user.is_active());
    println!("Display name: {}", user.get_display_name());
    
    user.deactivate();
    println!("Is active after deactivation: {}", user.is_active());
    
    user.activate();
    println!("Final user state: {:#?}", user);
    
    // === OWNERSHIP TRANSFER (CONSUMING METHODS) ===
    println!("\n=== Ownership Transfer ===");
    let rect5 = Rectangle::new(25, 35);
    println!("Before destruction: {:?}", rect5);
    
    let destruction_message = rect5.destroy();
    println!("{}", destruction_message);
    // rect5 is no longer accessible here - it was consumed by destroy()
    
    // === COMPARING FUNCTION VS METHOD SYNTAX ===
    println!("\n=== Comparing Function vs Method Syntax ===");
    let rect6 = Rectangle::new(40, 60);
    
    // Method syntax (preferred)
    println!("Method syntax area: {}", rect6.area());
    
    // Function syntax (equivalent but less ergonomic)
    println!("Function syntax area: {}", Rectangle::area(&rect6));
}