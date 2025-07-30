// Shared code and utilities for Rust book examples
// Add common functions, types, or utilities here that can be used across examples

#[allow(dead_code)]
pub fn print_separator(title: &str) {
    println!("\n=== {} ===", title);
}

#[allow(dead_code)]
pub fn print_chapter_header(chapter: &str, title: &str) {
    println!("\n{:=^50}", format!(" {} ", chapter));
    println!("{:^50}", title);
    println!("{:=^50}", "");
}

// Example: Common ownership demonstration functions
#[allow(dead_code)]
pub mod ownership_examples {
    pub fn takes_ownership(some_string: String) {
        println!("takes_ownership: {}", some_string);
    } // some_string goes out of scope and `drop` is called

    pub fn makes_copy(some_integer: i32) {
        println!("makes_copy: {}", some_integer);
    } // some_integer goes out of scope, nothing special happens

    pub fn gives_ownership() -> String {
        let some_string = String::from("yours");
        some_string // returned and moves out to the calling function
    }

    pub fn takes_and_gives_back(a_string: String) -> String {
        a_string // returned and moves out to the calling function
    }
}