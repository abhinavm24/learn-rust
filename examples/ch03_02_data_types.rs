use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 3.2", "Data Types");
    
    // === RUST'S TYPE SYSTEM ===
    // Rust is statically typed - all types must be known at compile time
    // The compiler can usually infer types, but sometimes we need explicit annotations
    
    // === SCALAR TYPES ===
    // Scalar types represent a single value: integers, floats, booleans, characters
    
    println!("\n--- Integer Types ---");
    // Rust has signed (i) and unsigned (u) integers of various sizes
    let small_positive: u8 = 255;        // 0 to 255
    let small_signed: i8 = -128;         // -128 to 127
    let medium: u16 = 65535;             // 0 to 65,535
    let normal: i32 = 2_147_483_647;     // Default integer type, most commonly used
    let big: u64 = 18_446_744_073_709_551_615u64; // Suffix to specify type
    let architecture_size: usize = 1000; // Size depends on architecture (32 or 64 bit)
    
    println!("u8: {}, i8: {}, u16: {}, i32: {}, u64: {}, usize: {}", 
             small_positive, small_signed, medium, normal, big, architecture_size);

    // Type annotation required when multiple types are possible
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Parsed string to u32: {}", guess);

    println!("\n--- Floating Point Types ---");
    // Rust has two floating point types: f32 and f64
    let x = 2.0;      // f64 by default (double precision)
    let y: f32 = 3.0; // f32 single precision (explicit annotation)
    
    println!("f64 (default): {}, f32: {}", x, y);

    println!("\n--- Numeric Operations ---");
    // All the basic math operations you'd expect
    let sum = 5 + 10;                    // Addition
    let difference = 95.5 - 4.3;        // Subtraction  
    let product = 4 * 30;                // Multiplication
    let quotient = 56.7 / 32.2;         // Division
    let truncated = -5 / 3;              // Integer division truncates toward zero
    let remainder = 43 % 5;              // Remainder (modulo)
    
    println!("5 + 10 = {}", sum);
    println!("95.5 - 4.3 = {}", difference);
    println!("4 * 30 = {}", product);
    println!("56.7 / 32.2 = {}", quotient);
    println!("-5 / 3 = {} (truncated)", truncated);
    println!("43 % 5 = {}", remainder);

    println!("\n--- Boolean Type ---");
    // Boolean type with two possible values: true and false
    let t = true;                        // Type inferred
    let f: bool = false;                 // Explicit type annotation
    
    println!("t: {}, f: {}", t, f);
    println!("!t (not t): {}", !t);     // Boolean negation
    println!("t && f (t and f): {}", t && f); // Logical AND
    println!("t || f (t or f): {}", t || f);  // Logical OR

    println!("\n--- Character Type ---");
    // char represents a Unicode scalar value (4 bytes)
    let c = 'z';                         // Single quotes for char literals
    let z: char = 'ℤ';                   // Unicode characters
    let heart_eyed_cat = '😻';           // Even emojis!
    
    println!("ASCII: '{}', Unicode: '{}', Emoji: '{}'", c, z, heart_eyed_cat);

    // === COMPOUND TYPES ===
    // Compound types can group multiple values: tuples and arrays
    
    println!("\n--- Tuple Type ---");
    // Tuples group together values of different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple with type annotation: {:?}", tup);

    // Destructuring - breaking apart a tuple into individual variables
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // Pattern matching to extract values
    println!("Destructured values - x: {}, y: {}, z: {}", x, y, z);

    // Accessing tuple elements by index
    let coordinates: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = coordinates.0;    // First element (index 0)
    let six_point_four = coordinates.1;  // Second element (index 1) 
    let one = coordinates.2;             // Third element (index 2)
    
    println!("Tuple access by index - [0]: {}, [1]: {}, [2]: {}", 
             five_hundred, six_point_four, one);

    println!("\n--- Array Type ---");
    // Arrays contain multiple values of the SAME type with FIXED size
    let a: [i32; 5] = [1, 2, 3, 4, 5];  // Type: [element_type; size]
    let first = a[0];                     // Arrays are zero-indexed
    let second = a[1];
    
    println!("Array: {:?}", a);
    println!("First element: {}, Second element: {}", first, second);
    
    // Array with repeated values
    let repeated = [3; 5];               // Creates [3, 3, 3, 3, 3]
    println!("Repeated array: {:?}", repeated);
    
    // Array bounds are checked at runtime - this would panic:
    // let invalid = a[10]; // index out of bounds: the len is 5 but the index is 10
    
    println!("\n--- Key Differences ---");
    println!("• Tuples: Fixed size, mixed types, access by index");
    println!("• Arrays: Fixed size, same type, access by index");
    println!("• Vectors: Dynamic size, same type (covered in later chapters)");
}
