//! Chapter 8.2: Storing UTF-8 Encoded Text with Strings
//! 
//! This example demonstrates working with Rust's string types, including String
//! and &str. Strings in Rust are UTF-8 encoded, which means they can contain
//! any valid Unicode data, but this also makes certain operations more complex
//! than in languages with simpler string representations.
//!
//! Key concepts:
//! - String vs &str differences
//! - UTF-8 encoding and its implications
//! - String creation, manipulation, and concatenation
//! - Safe string access and iteration
//! - Working with international text

use rust_book_examples::print_chapter_header;

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn analyze_text(text: &str) {
    println!("Text: {}", text);
    println!("Byte length: {}", text.len());
    println!("Character count: {}", text.chars().count());
    
    println!("Characters:");
    for (i, c) in text.chars().enumerate() {
        println!("  {}: {} (Unicode: U+{:04X})", i, c, c as u32);
    }
    
    println!("First 3 bytes: {:?}", &text.as_bytes()[..text.len().min(3)]);
}

fn demonstrate_basic_strings() {
    println!("\n=== Basic String Creation and Usage ===");
    
    // Creating strings
    let mut s = String::new();
    println!("Empty string: '{}'", s);
    
    let data = "initial contents";
    let s = data.to_string();
    println!("From string literal: '{}'", s);
    
    // Alternative creation method
    let s = String::from("initial contents");
    println!("Using String::from: '{}'", s);
    
    // This also works:
    let s = "initial contents".to_string();
    println!("Direct to_string(): '{}'", s);
    
    // Growing a string
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("After push_str: '{}'", s);
    
    // push_str doesn't take ownership
    let s1 = String::from("Hello, ");
    let s2 = "world!";
    let mut s = s1.clone();
    s.push_str(s2);
    println!("Final string: '{}', s2 is still valid: '{}'", s, s2);
}

fn demonstrate_concatenation() {
    println!("\n=== String Concatenation Methods ===");
    
    // Method 1: + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note: s1 has been moved and can no longer be used
    println!("Using + operator: '{}'", s3);
    // println!("s1 is no longer valid: {}", s1); // This would error
    
    // Method 2: format! macro (doesn't take ownership)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("Using format! macro: '{}'", s);
    println!("Original strings still valid: {}, {}, {}", s1, s2, s3);
    
    // Method 3: Multiple concatenations
    let mut result = String::from("Hello");
    result.push_str(", ");
    result.push_str("beautiful");
    result.push_str(" ");
    result.push_str("world");
    result.push('!');
    println!("Built with push operations: '{}'", result);
}

fn demonstrate_indexing_limitations() {
    println!("\n=== Understanding String Indexing Limitations ===");
    
    let s1 = String::from("hello");
    // let h = s1[0]; // This won't compile!
    println!("String '{}' has {} bytes", s1, s1.len());
    
    // Why indexing doesn't work:
    let hello_english = String::from("Hola");           // 4 bytes
    let hello_russian = String::from("Здравствуйте");  // 24 bytes, not 12!
    
    println!("'{}' - {} characters, {} bytes", 
             hello_english, hello_english.chars().count(), hello_english.len());
    println!("'{}' - {} characters, {} bytes", 
             hello_russian, hello_russian.chars().count(), hello_russian.len());
    
    println!("Each Cyrillic character takes 2 bytes in UTF-8");
    println!("So 'indexing' by byte position would be meaningless and unsafe");
}

fn demonstrate_safe_access() {
    println!("\n=== Safe String Access Methods ===");
    
    let hello = "Здравствуйте";
    
    // Slicing by byte position (be careful!)
    let s = &hello[0..4]; // Gets first 2 characters (4 bytes)
    println!("First 4 bytes of '{}': '{}'", hello, s);
    
    // This would panic: &hello[0..1] (cuts character in middle)
    println!("Attempting to slice in middle of character would panic!");
    
    // Safe iteration over characters
    println!("Characters in 'नमस्ते':");
    for (i, c) in "नमस्ते".chars().enumerate() {
        println!("  {}: {} (Unicode: U+{:04X})", i, c, c as u32);
    }
    
    // Iteration over bytes
    println!("Bytes in 'नमस्ते':");
    let byte_vec: Vec<u8> = "नमस्ते".bytes().collect();
    println!("  {:?} ({} bytes total)", byte_vec, byte_vec.len());
}

fn demonstrate_string_methods() {
    println!("\n=== String Methods and Operations ===");
    
    let mut s = String::from("Hello");
    
    // Adding to string
    s.push_str(", world");
    s.push('!');
    println!("Built string: '{}'", s);
    
    // String properties
    println!("Length in bytes: {}", s.len());
    println!("Length in chars: {}", s.chars().count());
    println!("Is empty: {}", s.is_empty());
    
    // String searching
    println!("Contains 'world': {}", s.contains("world"));
    println!("Starts with 'Hello': {}", s.starts_with("Hello"));
    println!("Ends with '!': {}", s.ends_with("!"));
    
    // String modification
    let replaced = s.replace("world", "Rust");
    println!("Replaced: '{}'", replaced);
    
    // Case conversion
    println!("Uppercase: '{}'", s.to_uppercase());
    println!("Lowercase: '{}'", s.to_lowercase());
    
    // Trimming whitespace
    let padded = "  hello world  ";
    println!("Original: '{}'", padded);
    println!("Trimmed: '{}'", padded.trim());
    println!("Trim start: '{}'", padded.trim_start());
    println!("Trim end: '{}'", padded.trim_end());
}

fn demonstrate_string_slices() {
    println!("\n=== Working with String Slices ===");
    
    let my_string = String::from("hello world");
    
    // first_word works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("First word of '{}': '{}'", &my_string[0..6], word);
    
    let word = first_word(&my_string[..]);
    println!("First word of '{}': '{}'", my_string, word);
    
    // first_word also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    println!("First word (reference): '{}'", word);
    
    let my_string_literal = "hello world";
    
    // first_word works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("First word of literal slice: '{}'", word);
    
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("First word of literal: '{}'", word);
}

fn demonstrate_unicode_handling() {
    println!("\n=== Unicode and Internationalization ===");
    
    // English text
    println!("==> English text:");
    analyze_text("Hello");
    
    println!("\n==> Text with accented characters:");
    analyze_text("Café");
    
    println!("\n==> Japanese text:");
    analyze_text("こんにちは");
    
    println!("\n==> Mixed scripts with emoji:");
    analyze_text("Hello 世界 🌍");
    
    // Demonstrate grapheme clusters
    println!("\n=== Complex Unicode Examples ===");
    let complex = "é"; // This might be one character or two (e + combining accent)
    println!("String '{}' has {} chars and {} bytes", 
             complex, complex.chars().count(), complex.len());
    
    // Different ways to represent the same character
    let e_acute1 = "é";  // Single precomposed character
    let e_acute2 = "e\u{0301}"; // e + combining acute accent
    println!("'{}' == '{}': {}", e_acute1, e_acute2, e_acute1 == e_acute2);
    println!("'{}' chars: {}, bytes: {}", e_acute1, e_acute1.chars().count(), e_acute1.len());
    println!("'{}' chars: {}, bytes: {}", e_acute2, e_acute2.chars().count(), e_acute2.len());
}

fn demonstrate_string_building() {
    println!("\n=== String Building and Performance ===");
    
    let words = vec!["Hello", " ", "beautiful", " ", "world"];
    
    // Less efficient: creates new string each time
    let mut result = String::new();
    for word in &words {
        result = result + word; // Creates new string each time
    }
    println!("Built with + operator: '{}'", result);
    
    // More efficient: push_str reuses existing capacity
    let mut result = String::new();
    for word in &words {
        result.push_str(word);
    }
    println!("Built with push_str: '{}'", result);
    
    // Even better: pre-allocate capacity
    let mut result = String::with_capacity(100);
    println!("Initial capacity: {}", result.capacity());
    for word in &words {
        result.push_str(word);
    }
    println!("Built with pre-allocated capacity: '{}' (capacity: {})", 
             result, result.capacity());
    
    // Best for simple cases: use format! or join
    let result = words.join("");
    println!("Built with join: '{}'", result);
    
    // Collecting from iterator
    let result: String = words.iter().copied().collect();
    println!("Built with collect: '{}'", result);
}

fn demonstrate_parsing_conversion() {
    println!("\n=== String Parsing and Conversion ===");
    
    // String to number conversion
    let num_str = "42";
    let num: i32 = num_str.parse().expect("Not a number!");
    println!("Parsed '{}' to number: {}", num_str, num);
    
    // Handle parsing errors
    let maybe_num = "not_a_number".parse::<i32>();
    match maybe_num {
        Ok(num) => println!("Parsed: {}", num),
        Err(e) => println!("Parse error for 'not_a_number': {}", e),
    }
    
    // Number to string conversion
    let num = 42;
    let num_str = num.to_string();
    let num_str2 = format!("{}", num);
    println!("Number {} as strings: '{}', '{}'", num, num_str, num_str2);
    
    // Boolean to string
    let flag = true;
    let flag_str = flag.to_string();
    println!("Boolean {} as string: '{}'", flag, flag_str);
    
    // Working with string collections
    let words: Vec<&str> = "hello world rust programming".split_whitespace().collect();
    println!("Split into words: {:?}", words);
    
    let rejoined = words.join("-");
    println!("Rejoined with '-': '{}'", rejoined);
}

fn demonstrate_advanced_operations() {
    println!("\n=== Advanced String Operations ===");
    
    let text = "The quick brown fox jumps over the lazy dog";
    
    // Splitting strings
    let words: Vec<&str> = text.split(' ').collect();
    println!("Words: {:?}", words);
    
    let parts: Vec<&str> = text.split("brown").collect();
    println!("Split by 'brown': {:?}", parts);
    
    // Finding substrings
    if let Some(pos) = text.find("fox") {
        println!("Found 'fox' at byte position: {}", pos);
    }
    
    // Multiple occurrences
    let text_with_repeats = "the cat and the dog and the bird";
    let positions: Vec<usize> = text_with_repeats.match_indices("the")
        .map(|(pos, _)| pos)
        .collect();
    println!("All 'the' positions in '{}': {:?}", text_with_repeats, positions);
    
    // String replacement
    let replaced = text.replace("fox", "cat");
    println!("Replaced 'fox' with 'cat': '{}'", replaced);
    
    // Replace multiple patterns
    let multi_replaced = text
        .replace("quick", "slow")
        .replace("brown", "white")
        .replace("fox", "turtle");
    println!("Multiple replacements: '{}'", multi_replaced);
    
    // Lines iteration
    let multiline = "Line 1\nLine 2\nLine 3\nLine 4";
    println!("Processing lines:");
    for (i, line) in multiline.lines().enumerate() {
        println!("  Line {}: '{}'", i + 1, line);
    }
    
    // Filtering characters
    let filtered: String = text.chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .collect();
    println!("Filtered (letters and spaces only): '{}'", filtered);
    
    // Transforming characters
    let transformed: String = text.chars()
        .map(|c| if c.is_lowercase() { c.to_uppercase().to_string() } else { c.to_string() })
        .collect();
    println!("Uppercase transformation: '{}'", transformed);
}

fn demonstrate_real_world_example() {
    println!("\n=== Real-World Example: User Greeting System ===");
    
    // Building a user greeting system
    let mut user_data = Vec::new();
    
    // Collect user information with international names
    let names = vec![
        "Alice",
        "Bob", 
        "世界",      // "World" in Chinese
        "مرحبا",     // "Hello" in Arabic
        "José",      // Spanish name with accent
        "François",  // French name with accent
        "🚀🦀",      // Emoji "name"
    ];
    
    for name in names {
        let greeting = format!("Hello, {}! Welcome to Rust! 🎉", name);
        user_data.push(greeting);
    }
    
    // Process and display
    println!("User Greetings:");
    for (index, greeting) in user_data.iter().enumerate() {
        println!("{}. {} (chars: {}, bytes: {})", 
                 index + 1, 
                 greeting, 
                 greeting.chars().count(),
                 greeting.len());
    }
    
    // Demonstrate text processing
    let all_greetings = user_data.join(" | ");
    println!("\nAll greetings combined: {}", all_greetings);
    
    // Count unique characters across all greetings
    let mut unique_chars = std::collections::HashSet::new();
    for greeting in &user_data {
        for c in greeting.chars() {
            unique_chars.insert(c);
        }
    }
    println!("Total unique characters used: {}", unique_chars.len());
}

fn main() {
    print_chapter_header("Chapter 8.2", "Storing UTF-8 Encoded Text with Strings");

    println!("Strings in Rust are UTF-8 encoded and can contain any valid Unicode data.");
    println!("This makes them powerful for international applications but requires");
    println!("understanding the difference between bytes, characters, and grapheme clusters.");

    demonstrate_basic_strings();
    demonstrate_concatenation();
    demonstrate_indexing_limitations();
    demonstrate_safe_access();
    demonstrate_string_methods();
    demonstrate_string_slices();
    demonstrate_unicode_handling();
    demonstrate_string_building();
    demonstrate_parsing_conversion();
    demonstrate_advanced_operations();
    demonstrate_real_world_example();

    println!("\n=== Key Takeaways ===");
    println!("• Use String for owned, mutable text data");
    println!("• Use &str for borrowed text data (string slices)");
    println!("• Strings are UTF-8 encoded, supporting international text");
    println!("• Cannot index strings directly - use chars() or bytes() iterators");
    println!("• Use format! macro for complex string building");
    println!("• Always consider Unicode when processing international text");
    println!("• String operations are designed to prevent common Unicode bugs");
}