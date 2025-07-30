use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 3.3", "Functions");
    
    // === FUNCTION BASICS ===
    // Functions are defined with 'fn' keyword
    // main() is the entry point of every Rust program
    println!("Starting from main function...");

    // === FUNCTION CALLS ===
    // Call functions by using their name followed by parentheses
    another_function(12);
    
    // Functions can take multiple parameters of different types
    print_labeled_measurement(5, 'h');
    
    // === RETURN VALUES ===
    // Functions can return values - note the return type annotation
    let x = five();
    println!("The value returned from five() is: {x}");

    // Functions can take parameters and return values
    let y = plus_two(x);
    println!("The value returned from plus_two({x}) is: {y}");
    
    // === STATEMENTS VS EXPRESSIONS ===
    demonstrate_statements_vs_expressions();
    
    // === EARLY RETURNS ===
    let result = early_return_example(10);
    println!("Early return result: {result}");
    
    let result2 = early_return_example(-5);
    println!("Early return result for negative: {result2}");
}

// === FUNCTION WITH PARAMETERS ===
// Parameters must have type annotations
// This function takes one parameter of type i32
fn another_function(x: i32) {
    println!("The value of parameter x is: {x}");
    // This function doesn't return anything (returns unit type '()')
}

// === FUNCTION WITH MULTIPLE PARAMETERS ===
// Each parameter needs its own type annotation
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// === FUNCTION WITH RETURN VALUE ===
// Return type is specified after the arrow ->
// The last expression is automatically returned (no semicolon!)
fn five() -> i32 {
    5  // This is an expression - no semicolon means it's returned
}

// === FUNCTION THAT TAKES INPUT AND RETURNS OUTPUT ===
fn plus_two(x: i32) -> i32 {
    x + 2  // Expression without semicolon - this is returned
    // Alternatively, we could write: return x + 2;
}

// === STATEMENTS VS EXPRESSIONS ===
fn demonstrate_statements_vs_expressions() {
    println!("\n--- Statements vs Expressions ---");
    
    // STATEMENTS: Instructions that perform actions but don't return values
    let x = 5;  // This is a statement - variable assignment
    
    // EXPRESSIONS: Evaluate to a resulting value
    let y = {
        let x = 3;      // Statement inside the block
        x + 1           // Expression - this value is returned from the block
    };  // The entire block {...} is an expression that evaluates to 4
    
    println!("x = {x}, y = {y}");
    
    // Function calls are expressions
    let z = plus_two(y);  // plus_two(y) is an expression that evaluates to a value
    println!("z = plus_two({y}) = {z}");
    
    // Math operations are expressions
    let calculation = 5 + 6;  // 5 + 6 is an expression
    println!("calculation = {calculation}");
}

// === EARLY RETURNS ===
// You can return early from a function using the 'return' keyword
fn early_return_example(x: i32) -> i32 {
    if x < 0 {
        return 0;  // Early return with explicit 'return' keyword
    }
    
    // If we get here, x is >= 0
    if x > 100 {
        return 100;  // Another early return
    }
    
    x * 2  // Normal return (expression without semicolon)
}