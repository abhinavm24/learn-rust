use rust_book_examples::print_chapter_header;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    print_chapter_header("Chapter 5.2", "An Example Program Using Structs");
    
    // === EVOLUTION FROM PRIMITIVES ===
    println!("\n=== Version 1: Separate Variables ===");
    let width1 = 30;
    let height1 = 50;
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area_primitives(width1, height1)
    );
    
    // === REFACTORING WITH TUPLES ===
    println!("\n=== Version 2: Using Tuples ===");
    let rect1 = (30, 50);
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );
    
    // === FINAL VERSION WITH STRUCTS ===
    println!("\n=== Version 3: Using Structs ===");
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect2)
    );
    
    // === DEBUG PRINTING ===
    println!("\n=== Debug Output Examples ===");
    
    // Compact debug output
    println!("rect2 is {:?}", rect2);
    
    // Pretty-printed debug output
    println!("rect2 is {:#?}", rect2);
    
    // === USING THE dbg! MACRO ===
    println!("\n=== Using dbg! Macro ===");
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),  // Prints the calculation and result
        height: 50,
    };
    
    dbg!(&rect3);  // Prints the entire struct
    
    // === MULTIPLE RECTANGLES ===
    println!("\n=== Working with Multiple Rectangles ===");
    let rectangles = vec![
        Rectangle { width: 30, height: 50 },
        Rectangle { width: 10, height: 40 },
        Rectangle { width: 60, height: 45 },
    ];
    
    for (i, rect) in rectangles.iter().enumerate() {
        println!("Rectangle {}: Area = {}", i + 1, area(rect));
        println!("  Dimensions: {:?}", rect);
    }
    
    // === COMPARING APPROACHES ===
    println!("\n=== Comparing All Approaches ===");
    let w = 25;
    let h = 35;
    let tuple_rect = (w, h);
    let struct_rect = Rectangle { width: w, height: h };
    
    println!("Primitive approach: {}", area_primitives(w, h));
    println!("Tuple approach: {}", area_tuple(tuple_rect));
    println!("Struct approach: {}", area(&struct_rect));
    
    // === DEMONSTRATING READABILITY ===
    println!("\n=== Demonstrating Code Readability ===");
    demonstrate_readability();
}

// Version 1: Using primitive parameters
fn area_primitives(width: u32, height: u32) -> u32 {
    width * height
}

// Version 2: Using tuple parameter
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1  // Less clear what .0 and .1 represent
}

// Version 3: Using struct parameter (final version)
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height  // Very clear and readable
}

fn demonstrate_readability() {
    let rect = Rectangle {
        width: 40,
        height: 30,
    };
    
    // With structs, the intent is much clearer
    println!("Width: {}, Height: {}", rect.width, rect.height);
    println!("Perimeter: {}", 2 * (rect.width + rect.height));
    println!("Is square: {}", rect.width == rect.height);
    
    // Compare with tuple version:
    let tuple_rect = (40, 30);
    println!("Tuple width: {}, height: {}", tuple_rect.0, tuple_rect.1);
    // Less clear which is width and which is height!
}