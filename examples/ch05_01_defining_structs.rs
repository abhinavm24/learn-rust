use rust_book_examples::print_chapter_header;

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct UnitLikeStruct;

fn main() {
    print_chapter_header("Chapter 5.1", "Defining and Instantiating Structs");
    
    // === BASIC STRUCT INSTANTIATION ===
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    println!("User email: {}", user1.email);
    println!("User is active: {}", user1.active);
    
    // === MUTABLE STRUCTS ===
    let mut user2 = User {
        active: true,
        username: String::from("anotheruser456"),
        email: String::from("another@example.com"),
        sign_in_count: 1,
    };
    
    user2.email = String::from("updated@example.com");
    user2.sign_in_count += 1;
    
    println!("Updated user email: {}", user2.email);
    println!("Sign in count: {}", user2.sign_in_count);
    
    // === CONSTRUCTOR FUNCTIONS ===
    let user3 = build_user(
        String::from("constructor@example.com"),
        String::from("constructor_user")
    );
    println!("User created via constructor: {}", user3.username);
    
    let user4 = build_user_shorthand(
        String::from("shorthand@example.com"),
        String::from("shorthand_user")
    );
    println!("User created via shorthand: {}", user4.username);
    
    // === STRUCT UPDATE SYNTAX ===
    let user5 = User {
        email: String::from("updated@example.com"),
        ..user4  // Use remaining fields from user4
    };
    println!("User5 email: {}", user5.email);
    println!("User5 username (from user4): {}", user5.username);
    
    // === TUPLE STRUCTS ===
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("Black color: {:?}", black);
    println!("Origin point: {:?}", origin);
    println!("Black red component: {}", black.0);
    println!("Origin x coordinate: {}", origin.0);
    
    // === UNIT-LIKE STRUCTS ===
    let subject = UnitLikeStruct;
    println!("Unit-like struct: {:?}", subject);
    
    // === DIFFERENT FIELD TYPES ===
    #[derive(Debug)]
    struct Employee {
        id: u32,
        name: String,
        salary: f64,
        active: bool,
        department: String,
    }
    
    let employee = Employee {
        id: 1001,
        name: String::from("Alice Johnson"),
        salary: 75000.0,
        active: true,
        department: String::from("Engineering"),
    };
    
    println!("Employee {}: ${}", employee.name, employee.salary);
    println!("Employee details: {:#?}", employee);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username,    // Field init shorthand
        email,       // Same as email: email
        sign_in_count: 1,
    }
}