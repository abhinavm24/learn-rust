use rust_book_examples::{print_chapter_header, ownership_examples::*};

fn main() {
    print_chapter_header("Chapter 4.1", "What is Ownership?");
    
    // === WHAT IS OWNERSHIP? ===
    // Ownership is Rust's unique approach to memory management
    // - Each value has a single owner (variable)
    // - When the owner goes out of scope, the value is dropped
    // - There can only be one owner at a time
    
    // === BASIC OWNERSHIP AND SCOPE ===
    println!("\n--- Basic Ownership ---");
    
    // String::from creates a String on the heap (unlike string literals which are on the stack)
    let mut s = String::from("hello");
    s.push_str(", world!"); // We can modify it because it's mutable
    println!("s = {}", s);

    // === MOVE SEMANTICS ===
    // When we assign s to s1, the ownership is MOVED (not copied)
    // After this line, s is no longer valid - it has been "moved"
    let s1 = s; // s is moved to s1, s becomes invalid
    println!("s1 = {}", s1);
    
    // This would cause a compile error:
    // println!("s = {}", s); // ERROR: borrow of moved value: `s`

    // === CLONING ===
    // If we want to create a deep copy, we must explicitly call clone()
    let s2 = s1.clone(); // Explicit deep copy - both s1 and s2 are now valid
    println!("s1 = {}, s2 = {}", s1, s2);
    
    // Both s1 and s2 are valid because clone() created a separate copy

    // === FUNCTION OWNERSHIP ===
    println!("\n--- Function Ownership ---");
    
    let y = String::from("hello");
    println!("Before function call, y = {}", y);
    
    // Passing y to the function moves ownership to the function
    // After this call, y is no longer valid in this scope
    takes_ownership(y); // y is moved into function and will be dropped when function ends
    
    // This would cause a compile error:
    // println!("y after function call: {}", y); // ERROR: borrow of moved value

    // === COPY TRAIT ===
    // Types like integers implement the Copy trait
    // When passed to functions, they are copied, not moved
    let x = 5;
    println!("Before function call, x = {}", x);
    
    makes_copy(x); // x is copied (i32 implements Copy trait)
    println!("x is still valid after function call: {}", x); // x is still valid!
    
    // Copy trait applies to:
    // - All integer types (i32, u32, etc.)
    // - Boolean type (bool)
    // - Floating point types (f64, etc.)
    // - Character type (char)
    // - Tuples containing only Copy types

    // === RETURN VALUE OWNERSHIP ===
    println!("\n--- Return Value Ownership ---");
    
    // Functions can transfer ownership through return values
    let s1 = gives_ownership(); // Function creates and returns ownership
    println!("s1 from gives_ownership: {}", s1);
    
    let s2 = String::from("hello");
    println!("s2 before passing to function: {}", s2);
    
    // s2 is moved into the function, but ownership is returned
    let s3 = takes_and_gives_back(s2); // s2 is moved in, ownership returned as s3
    
    println!("s3 from takes_and_gives_back: {}", s3);
    
    // s2 is no longer available here - it was moved into the function
    // This would cause a compile error:
    // println!("s2 after function call: {}", s2); // ERROR: borrow of moved value
    
    // === OWNERSHIP RULES SUMMARY ===
    // 1. Each value in Rust has a variable that's called its owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped
}