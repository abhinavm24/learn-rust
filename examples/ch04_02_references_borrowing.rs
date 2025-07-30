use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 4.2", "References and Borrowing");
    
    // === WHAT IS BORROWING? ===
    // Borrowing allows you to use values without taking ownership
    // It's like borrowing a book - you can read it, but you have to give it back
    // References (&) let you refer to some value without owning it
    
    println!("\n--- Basic References ---");
    
    // Without borrowing, this would move ownership
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1 creates a reference to s1
    println!("The length of '{}' is {}.", s1, len); // s1 is still valid!
    
    // The ampersand (&) creates a reference
    // This is called "borrowing" because we're borrowing the value
    
    println!("\n--- How References Work ---");
    let s = String::from("hello world");
    let r = &s; // r is a reference to s
    
    // We can use r just like s for reading
    println!("Original string: {}", s);
    println!("Reference to string: {}", r);
    println!("Both refer to the same data: {}", s == *r); // * dereferences
    
    // === BORROWING RULES ===
    // 1. At any given time, you can have EITHER one mutable reference 
    //    OR any number of immutable references
    // 2. References must always be valid (no dangling references)
    
    println!("\n--- Multiple Immutable References ---");
    let text = String::from("Rust is awesome");
    
    // We can have multiple immutable references
    let r1 = &text;
    let r2 = &text;
    let r3 = &text;
    
    println!("r1: {}", r1);
    println!("r2: {}", r2);
    println!("r3: {}", r3);
    // All three references can coexist because they're all immutable
    
    // === MUTABLE REFERENCES ===
    println!("\n--- Mutable References ---");
    
    let mut s = String::from("hello");
    println!("Before change: {}", s);
    
    // Create a mutable reference and pass it to a function
    change(&mut s);
    println!("After change: {}", s);
    
    // === MUTABLE REFERENCE RESTRICTIONS ===
    println!("\n--- Mutable Reference Rules ---");
    
    let mut value = String::from("initial");
    
    {
        // Only one mutable reference allowed at a time
        let r1 = &mut value;
        r1.push_str(" modified");
        println!("Modified through r1: {}", r1);
        // r1 goes out of scope here
    }
    
    // Now we can create another mutable reference
    let r2 = &mut value;
    r2.push_str(" again");
    println!("Modified through r2: {}", r2);
    
    // This would cause a compile error (uncomment to see):
    // let r3 = &mut value; // ERROR: cannot borrow as mutable more than once
    // println!("{} {}", r2, r3);
    
    // === MIXING IMMUTABLE AND MUTABLE REFERENCES ===
    println!("\n--- Reference Scope Rules ---");
    
    let mut data = String::from("hello");
    
    let r1 = &data;     // OK - immutable reference
    let r2 = &data;     // OK - another immutable reference
    println!("{} and {}", r1, r2); // Last use of r1 and r2
    
    // After r1 and r2 are no longer used, we can create a mutable reference
    let r3 = &mut data; // OK - no immutable references in use anymore
    r3.push_str(" world");
    println!("{}", r3);
    
    // === REFERENCES AS FUNCTION PARAMETERS ===
    println!("\n--- Function Parameters ---");
    
    let message = String::from("Hello, Rust!");
    
    // Pass immutable reference - function can read but not modify
    print_message(&message);
    
    // Original value is still available
    println!("Original message still exists: {}", message);
    
    let mut mutable_message = String::from("Hello");
    println!("Before modification: {}", mutable_message);
    
    // Pass mutable reference - function can modify
    append_exclamation(&mut mutable_message);
    println!("After modification: {}", mutable_message);
    
    // === RETURN REFERENCES ===
    println!("\n--- Returning References ---");
    
    let numbers = vec![1, 2, 3, 4, 5];
    let first = get_first(&numbers);
    match first {
        Some(value) => println!("First number: {}", value),
        None => println!("Empty vector"),
    }
    
    // === PREVENTING DANGLING REFERENCES ===
    println!("\n--- Dangling Reference Prevention ---");
    
    // This would create a dangling reference (won't compile):
    // let reference_to_nothing = dangle(); // ERROR!
    
    // Instead, return owned data:
    let valid_string = no_dangle();
    println!("Valid owned string: {}", valid_string);
    
    // === PRACTICAL EXAMPLES ===
    println!("\n--- Practical Examples ---");
    
    // String processing without taking ownership
    let text = String::from("The quick brown fox jumps over the lazy dog");
    let word_count = count_words(&text);
    println!("'{}' has {} words", text, word_count);
    
    // Finding elements without moving data
    let scores = vec![85, 92, 78, 96, 88];
    if let Some(highest) = find_max(&scores) {
        println!("Highest score: {}", highest);
    }
    println!("Original scores still available: {:?}", scores);
    
    // Modifying data through references
    let mut inventory = vec![10, 5, 8, 12];
    println!("Before update: {:?}", inventory);
    update_inventory(&mut inventory, 0, 15); // Update index 0 to 15
    println!("After update: {:?}", inventory);
    
    println!("\n--- Summary ---");
    println!("✓ References let you use values without taking ownership");
    println!("✓ Use & for immutable references, &mut for mutable references");
    println!("✓ Can have many immutable references OR one mutable reference");
    println!("✓ References must always be valid (no dangling references)");
    println!("✓ Borrowing makes Rust functions more ergonomic and efficient");
}

// Function that takes an immutable reference
// It can read the value but cannot modify it
fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // s goes out of scope, but it doesn't have ownership, so nothing is dropped

// Function that takes a mutable reference
// It can both read and modify the value
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn print_message(msg: &String) {
    println!("Message: {}", msg);
    // We can read the value but not modify it
}

fn append_exclamation(msg: &mut String) {
    msg.push('!');
    // We can modify the value because we have a mutable reference
}

// Function returning a reference to existing data
fn get_first(numbers: &Vec<i32>) -> Option<&i32> {
    numbers.first() // Returns Option<&i32>
}

// This would create a dangling reference (commented out because it won't compile):
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // ERROR: s is dropped but we're returning a reference to it
// }

// Correct way: return owned data instead of a reference
fn no_dangle() -> String {
    let s = String::from("hello");
    s // Move ownership to the caller
}

// Practical function: count words in a string without taking ownership
fn count_words(text: &String) -> usize {
    text.split_whitespace().count()
}

// Find maximum value in a slice without taking ownership
fn find_max(numbers: &[i32]) -> Option<&i32> {
    numbers.iter().max()
}

// Modify data through a mutable reference
fn update_inventory(inventory: &mut Vec<i32>, index: usize, new_value: i32) {
    if index < inventory.len() {
        inventory[index] = new_value;
    }
}