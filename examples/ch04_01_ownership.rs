use rust_book_examples::{print_chapter_header, ownership_examples::*};

fn main() {
    print_chapter_header("Chapter 4.1", "What is Ownership?");
    
    // Basic ownership and scope
    println!("\n--- Basic Ownership ---");
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("s = {}", s);

    let s1 = s; // s is moved to s1
    println!("s1 = {}", s1);

    let s2 = s1.clone(); // Explicit clone
    println!("s1 = {}, s2 = {}", s1, s2);

    // Function ownership examples
    println!("\n--- Function Ownership ---");
    let y = String::from("hello");
    takes_ownership(y); // y is moved into function

    let x = 5;
    makes_copy(x); // x is copied (i32 has Copy trait)
    println!("x is still valid: {}", x);

    // Return value ownership
    println!("\n--- Return Value Ownership ---");
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    
    println!("s1 = {}, s3 = {}", s1, s3);
    // s2 is no longer available here - it was moved
}