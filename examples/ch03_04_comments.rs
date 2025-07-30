use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 3.4", "Comments");
    
    // === WHAT ARE COMMENTS? ===
    // Comments are notes in your code that are ignored by the compiler
    // They're for humans to read and understand the code
    // In Rust, comments start with // and continue to the end of the line
    
    println!("Comments demo starting...");
    
    // === SINGLE-LINE COMMENTS ===
    // This is a single-line comment
    // Everything after // on this line is ignored by the compiler
    
    let x = 5; // This is an end-of-line comment
    println!("x = {}", x);
    
    // You can have multiple consecutive comment lines
    // to create what looks like a multi-line comment
    // Each line needs its own // though
    
    // === COMMENTING STYLES ===
    
    // Style 1: Comments above the code they describe
    // Calculate the area of a rectangle
    let length = 10;
    let width = 5;
    let area = length * width;
    println!("Rectangle area: {} square units", area);
    
    let radius = 3.0; // Style 2: End-of-line comments for simple explanations
    let pi = 3.14159;
    
    // Style 3: Block of comments explaining complex logic
    // Calculate circle area using the formula: A = π × r²
    // We're using a simplified value of π for this example
    // In real code, you'd use std::f64::consts::PI
    let circle_area = pi * radius * radius;
    println!("Circle area: {:.2} square units", circle_area);
    
    // === WHAT TO COMMENT ===
    
    // ✅ Good: Explain WHY, not WHAT
    // We use a small epsilon for floating point comparison
    // because direct equality checks on floats are unreliable
    let epsilon = 0.0001f64;
    let a = 0.1f64 + 0.2f64;
    let b = 0.3f64;
    if (a - b).abs() < epsilon {
        println!("Values are effectively equal");
    }
    
    // ❌ Bad: Stating the obvious
    // let name = "Alice"; // Set name to Alice
    let name = "Alice"; // Default user name for demo purposes
    
    // ✅ Good: Explaining business logic or complex algorithms
    // Apply 15% discount for orders over $100
    // This matches our current promotion policy
    let order_total = 120.0;
    let discount_rate = if order_total > 100.0 { 0.15 } else { 0.0 };
    let final_total = order_total * (1.0 - discount_rate);
    println!("{}'s order total: ${:.2}", name, final_total);
    
    // === SPECIAL COMMENT CONVENTIONS ===
    
    // TODO: Add input validation for negative numbers
    let temperature = -5;
    println!("Temperature: {}°C", temperature);
    
    // FIXME: This algorithm is inefficient for large datasets
    // NOTE: Consider using a hash map for O(1) lookup instead
    let numbers = vec![1, 2, 3, 4, 5];
    let target = 3;
    let mut found_index = None;
    for (i, &num) in numbers.iter().enumerate() {
        if num == target {
            found_index = Some(i);
            break;
        }
    }
    
    match found_index {
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found", target),
    }
    
    // === COMMENTS FOR DOCUMENTATION ===
    // These comments help explain the overall program structure
    demonstrate_comment_best_practices();
    
    println!("Comments demo completed!");
    
    // === IMPORTANT NOTES ABOUT COMMENTS ===
    // 1. Comments are completely ignored by the compiler
    // 2. They don't affect program performance at all
    // 3. Keep them updated when you change the code
    // 4. Use them to explain complex business logic
    // 5. Avoid obvious comments that just repeat the code
}

/// This function demonstrates documentation comments (covered in later chapters)
/// Documentation comments use /// and can generate HTML documentation
/// For now, we're using regular // comments
fn demonstrate_comment_best_practices() {
    println!("\n--- Comment Best Practices ---");
    
    // Good: Explain the algorithm or approach
    // Using the Euclidean algorithm to find GCD
    // This is more efficient than checking all factors
    let mut a = 48;
    let mut b = 18;
    
    // The algorithm repeatedly replaces the larger number
    // with the remainder of dividing the larger by the smaller
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    
    println!("Greatest Common Divisor: {}", a);
    
    // Good: Explain why you chose a particular approach
    // We're using a vector instead of an array here because
    // we don't know the final size at compile time
    let mut fibonacci = Vec::new();
    fibonacci.push(0);
    fibonacci.push(1);
    
    // Generate first 10 Fibonacci numbers
    for i in 2..10 {
        let next = fibonacci[i-1] + fibonacci[i-2];
        fibonacci.push(next);
    }
    
    println!("Fibonacci sequence: {:?}", fibonacci);
    
    // Good: Explain tricky or non-obvious code
    // Reverse the vector in-place using two pointers
    // This is more memory-efficient than creating a new vector
    let mut left = 0;
    let mut right = fibonacci.len() - 1;
    
    while left < right {
        fibonacci.swap(left, right);
        left += 1;
        right -= 1;
    }
    
    println!("Reversed: {:?}", fibonacci);
}