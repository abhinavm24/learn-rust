use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 3.5", "Control Flow");
    
    // === IF EXPRESSIONS ===
    // if statements allow you to branch your code depending on conditions
    // Note: conditions MUST be boolean - Rust won't automatically convert types
    
    println!("\n--- Basic If Expressions ---");
    let number = 6;

    // Simple if-else
    if number < 10 {
        println!("Smaller than 10");
    } else {
        println!("Bigger than or equal to 10");
    }

    // if-else if-else chain
    // Each condition is checked in order until one is true
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // === IF AS AN EXPRESSION ===
    // if is an expression, so it returns a value
    // Both arms must return the same type
    let number = if true { 5 } else { 6 };  // Both arms return i32
    println!("The value of number is: {number}");
    
    // This would be a compile error - type mismatch:
    // let number = if condition { 5 } else { "six" }; // i32 vs &str
    
    // More complex if expressions
    let x = 10;
    let description = if x > 0 {
        "positive"
    } else if x < 0 {
        "negative" 
    } else {
        "zero"
    };
    println!("{x} is {description}");

    // === LOOPS ===
    // Rust has three kinds of loops: loop, while, and for
    
    println!("\n--- Loop (Infinite Loop) ---");
    // loop creates an infinite loop - you must break out explicitly
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("Loop iteration: {counter}");

        if counter < 10 {
            continue;  // Skip to next iteration
        } else if counter == 10 {
            break counter * 2;  // break with a value - loop can return values!
        } else {
            break counter + 1;
        }
    };
    println!("The result is: {result}");

    // === LOOP LABELS ===
    // You can label loops and break/continue specific loops
    println!("\n--- Loop Labels ---");
    // Loop with labels - useful for nested loops
    let mut count = 0;
    'counting_up: loop {  // Loop label starts with single quote
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;  // Breaks inner loop only
            }
            if count == 2 {
                break 'counting_up;  // Breaks outer loop using label
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // === WHILE LOOPS ===
    // while loops continue as long as a condition is true
    println!("\n--- While Loops ---");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
    
    // while with more complex conditions
    let mut stack = vec![1, 2, 3];
    println!("Popping from stack:");
    while let Some(value) = stack.pop() {  // while let pattern
        println!("Popped: {value}");
    }

    // === FOR LOOPS ===
    // for loops iterate over collections - most common loop type
    println!("\n--- For Loops ---");
    
    // Iterating over arrays
    let a = [10, 20, 30, 40, 50];
    println!("Iterating over array:");
    for element in a {
        println!("the value is: {element}");
    }
    
    // Iterating with index (using enumerate)
    println!("Iterating with index:");
    for (index, element) in a.iter().enumerate() {
        println!("a[{index}] = {element}");
    }

    // === RANGES ===
    // for loops commonly use ranges
    println!("\n--- Ranges ---");
    
    // Range 1..4 means 1, 2, 3 (exclusive end)
    println!("Range 1..4:");
    for number in 1..4 {
        println!("{number}");
    }
    
    // Reversed range for countdown
    println!("Countdown:");
    for number in (1..4).rev() {  // .rev() reverses the iterator
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    
    // === BREAK AND CONTINUE ===
    println!("\n--- Break and Continue ---");
    
    for i in 1..=10 {
        if i % 2 == 0 {
            continue;  // Skip even numbers
        }
        if i > 7 {
            break;     // Stop when we reach numbers > 7
        }
        println!("Odd number: {i}");
    }
    
    // === PRACTICAL EXAMPLES ===
    println!("\n--- Practical Examples ---");
    
    // Finding elements
    let numbers = [1, 3, 5, 7, 9, 2, 4, 6, 8];
    let target = 7;
    let mut found = false;
    
    for (index, &number) in numbers.iter().enumerate() {
        if number == target {
            println!("Found {target} at index {index}");
            found = true;
            break;
        }
    }
    
    if !found {
        println!("{target} not found in array");
    }
}
