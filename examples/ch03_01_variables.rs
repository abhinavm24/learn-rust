use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 3.1", "Variables and Mutability");
    
    // === IMMUTABILITY BY DEFAULT ===
    // Variables in Rust are immutable by default
    // Once a value is bound to a variable, you can't change that value
    let x = 5;
    println!("Initial x: {x}");
    
    // This would cause a compile error:
    // x = 6; // ERROR: cannot assign twice to immutable variable

    // === VARIABLE SHADOWING ===
    // We can declare a new variable with the same name as a previous variable
    // The first variable is "shadowed" by the second
    // This is different from making a variable mutable - we're creating a new variable
    let x = x + 1;  // This creates a NEW variable x, shadowing the previous one
    println!("After shadowing (x + 1): {x}");

    // === SCOPE AND SHADOWING ===
    // Variables have scope - they're only valid within their block
    {
        // This creates another new variable x within this inner scope
        // It shadows the x from the outer scope, but only within this block
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
        
        // We could even change the type when shadowing:
        // let x = "Hello"; // This would be valid - shadowing allows type changes
    } // The inner scope x goes out of scope here

    // Back to the outer scope - the x here is the one from line 15 (value 6)
    println!("The value of x is: {x}");
    
    // === MUTABILITY ===
    // If we want to modify a variable's value, we must explicitly declare it mutable
    let mut y = 5;
    println!("Initial y: {y}");
    
    y = 6; // This is allowed because y is declared as mutable
    println!("Modified y: {y}");
    
    // Note: With mut, we can change the value but not the type
    // y = "Hello"; // This would be a compile error - type mismatch
}