use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 4.3", "The Slice Type");
    
    // === WHAT ARE SLICES? ===
    // Slices are references to contiguous sequences of elements in a collection
    // They don't have ownership - they're a "view" into existing data
    // Slices are written as &[T] for arrays/vectors and &str for strings
    
    println!("\n--- String Slices Basics ---");
    
    let s = String::from("hello world");
    
    // Create slices using range syntax
    let hello = &s[0..5];   // Characters 0 through 4 (5 is exclusive)
    let world = &s[6..11];  // Characters 6 through 10
    
    println!("Original string: '{}'", s);
    println!("First slice: '{}'", hello);
    println!("Second slice: '{}'", world);
    
    // === SLICE RANGE SYNTAX ===
    println!("\n--- Slice Range Syntax ---");
    
    let text = String::from("programming");
    
    // Different ways to create slices
    let beginning = &text[0..4];    // "prog"
    let middle = &text[4..7];       // "ram"
    let end = &text[7..11];         // "ming"
    
    // Shorthand syntax
    let from_start = &text[..4];    // Same as &text[0..4]
    let to_end = &text[7..];        // From index 7 to end
    let whole_string = &text[..];   // Entire string
    
    println!("text: '{}'", text);
    println!("beginning [0..4]: '{}'", beginning);
    println!("middle [4..7]: '{}'", middle);
    println!("end [7..11]: '{}'", end);
    println!("from_start [..4]: '{}'", from_start);
    println!("to_end [7..]: '{}'", to_end);
    println!("whole_string [..]: '{}'", whole_string);
    
    // === PRACTICAL STRING SLICE EXAMPLE ===
    println!("\n--- First Word Function ---");
    
    let sentence = String::from("hello rust world");
    let first_word = get_first_word(&sentence);
    println!("First word of '{}' is '{}'", sentence, first_word);
    
    // The slice remains valid as long as the original string exists
    println!("Original sentence still exists: '{}'", sentence);
    
    // === STRING LITERALS ARE SLICES ===
    println!("\n--- String Literals as Slices ---");
    
    // String literals are actually slices (&str)
    let literal = "Hello, world!"; // Type is &str (string slice)
    let owned_string = String::from("Hello, world!"); // Type is String
    
    // Both work with functions that take &str
    println!("Literal length: {}", get_string_length(literal));
    println!("String length: {}", get_string_length(&owned_string));
    
    // === ARRAY SLICES ===
    println!("\n--- Array Slices ---");
    
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Create slices of the array
    let first_half = &numbers[0..5];    // [1, 2, 3, 4, 5]
    let second_half = &numbers[5..];    // [6, 7, 8, 9, 10]
    let middle_three = &numbers[3..6];  // [4, 5, 6]
    
    println!("Full array: {:?}", numbers);
    println!("First half: {:?}", first_half);
    println!("Second half: {:?}", second_half);
    println!("Middle three: {:?}", middle_three);
    
    // === VECTOR SLICES ===
    println!("\n--- Vector Slices ---");
    
    let vec = vec![10, 20, 30, 40, 50];
    let vec_slice = &vec[1..4]; // [20, 30, 40]
    
    println!("Vector: {:?}", vec);
    println!("Vector slice: {:?}", vec_slice);
    
    // Process the slice without taking ownership of the vector
    let sum = sum_slice(vec_slice);
    println!("Sum of slice: {}", sum);
    println!("Original vector still available: {:?}", vec);
    
    // === MUTABLE SLICES ===
    println!("\n--- Mutable Slices ---");
    
    let mut arr = [1, 2, 3, 4, 5];
    println!("Before modification: {:?}", arr);
    
    {
        // Create a mutable slice
        let slice = &mut arr[1..4]; // [2, 3, 4]
        
        // Modify elements through the slice
        slice[0] = 20; // Changes arr[1] to 20
        slice[2] = 40; // Changes arr[3] to 40
        
        println!("Slice after modification: {:?}", slice);
    }
    
    println!("Array after modification: {:?}", arr);
    
    // === SLICE BOUNDS CHECKING ===
    println!("\n--- Safe Slice Access ---");
    
    let safe_string = String::from("safe");
    
    // This would panic: let bad_slice = &safe_string[0..10]; // Index out of bounds!
    
    // Safe way using get() method
    match safe_string.get(0..2) {
        Some(slice) => println!("Safe slice: '{}'", slice),
        None => println!("Slice range is invalid"),
    }
    
    match safe_string.get(0..10) {
        Some(slice) => println!("Safe slice: '{}'", slice),
        None => println!("Slice range is invalid"), // This will print
    }
    
    // === FUNCTION PARAMETERS WITH SLICES ===
    println!("\n--- Flexible Function Parameters ---");
    
    // Functions taking slices are more flexible
    let string = String::from("Hello");
    let literal = "World";
    let array = [1, 2, 3, 4, 5];
    let vector = vec![6, 7, 8, 9, 10];
    
    // Same function works with String, &str, string literals
    analyze_text(&string);  // &String automatically converted to &str
    analyze_text(literal);  // &str directly
    analyze_text("inline"); // String literal
    
    // Same function works with arrays, vectors, and their slices
    process_numbers(&array);          // Array slice
    process_numbers(&vector);         // Vector slice
    process_numbers(&array[1..3]);    // Partial array slice
    process_numbers(&vector[2..]);    // Partial vector slice
    
    // === PRACTICAL APPLICATIONS ===
    println!("\n--- Practical Applications ---");
    
    // Text processing
    let document = String::from("The quick brown fox jumps over the lazy dog");
    let words = split_into_words(&document);
    println!("Document has {} words", words.len());
    for (i, word) in words.iter().enumerate() {
        if i < 5 { // Show first 5 words
            println!("Word {}: '{}'", i + 1, word);
        }
    }
    
    // Data processing
    let data = vec![1, 4, 2, 8, 5, 7, 3, 6];
    let max_in_range = find_max_in_range(&data, 2, 6);
    match max_in_range {
        Some(max) => println!("Max in range [2..6]: {}", max),
        None => println!("Range is empty or invalid"),
    }
    
    // === WHY SLICES MATTER ===
    println!("\n--- Why Slices Are Important ---");
    println!("✓ Memory efficient - no copying, just references");
    println!("✓ Flexible - work with String, &str, arrays, vectors");
    println!("✓ Safe - bounds checking prevents buffer overflows");
    println!("✓ Ergonomic - natural syntax for working with parts of data");
    println!("✓ Zero-cost - compile to simple pointer + length");
}

// Function that finds the first word in a string
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // Found a space
            return &s[0..i]; // Return slice from start to space
        }
    }
    
    &s[..] // No space found, return entire string
}

// Function that works with both String and &str
fn get_string_length(s: &str) -> usize {
    s.len()
}

// Function that processes a slice of integers
fn sum_slice(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

// Function that analyzes text (works with any string-like type)
fn analyze_text(text: &str) {
    println!("Text '{}' has {} characters", text, text.len());
}

// Function that processes number slices (works with arrays, vectors, slices)
fn process_numbers(numbers: &[i32]) {
    if !numbers.is_empty() {
        let sum: i32 = numbers.iter().sum();
        let avg = sum as f64 / numbers.len() as f64;
        println!("Numbers: {:?}, Sum: {}, Average: {:.2}", numbers, sum, avg);
    }
}

// Split text into words and return references to the original string
fn split_into_words(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}

// Find maximum value in a specific range of a slice
fn find_max_in_range(data: &[i32], start: usize, end: usize) -> Option<&i32> {
    if start >= data.len() || end > data.len() || start >= end {
        return None;
    }
    
    data[start..end].iter().max()
}