# Chapter 8.2: Storing UTF-8 Encoded Text with Strings

## Key Takeaways

### String Type Overview
- **Two Main Types**: `String` (owned, growable) and `&str` (string slice, borrowed)
- **UTF-8 Encoding**: All strings are valid UTF-8, supporting international characters
- **Memory Management**: String owns its data, &str references existing data
- **Complexity**: Strings are more complex than simple character arrays due to Unicode

### String vs &str
- **String**: Owned, mutable, growable, heap-allocated
- **&str**: Reference to UTF-8 data, usually immutable, can point to different locations
- **String Literals**: `"hello"` are `&str` type, stored in program binary
- **Conversion**: Easy conversion between types with methods like `to_string()`

### UTF-8 Implications
- Characters can be 1-4 bytes long
- Direct indexing `s[0]` is not allowed
- Must iterate by characters, bytes, or grapheme clusters
- Prevents common Unicode-related bugs

### Important Syntax and Operators

#### String Creation
```rust
let s = String::new();                    // Empty string
let s = "hello".to_string();             // From string literal
let s = String::from("hello");           // Alternative creation
```

#### String Manipulation
```rust
s.push_str("world");                     // Append string slice
s.push('!');                             // Append single character
let s3 = s1 + &s2;                       // Concatenation (moves s1)
let s = format!("{}-{}", s1, s2);        // Format macro (doesn't move)
```

#### String Iteration
```rust
for c in s.chars() { }                   // Iterate over characters
for b in s.bytes() { }                   // Iterate over bytes
```

### Programming Concepts Introduced
- **Unicode Handling**: Proper support for international text
- **UTF-8 Encoding**: Variable-length character encoding
- **String Ownership**: Understanding when strings are owned vs borrowed
- **Text Processing**: Safe methods for working with textual data

### Code Examples and Patterns

#### Basic String Creation and Manipulation
```rust
fn main() {
    // Creating strings
    let mut s = String::new();
    
    let data = "initial contents";
    let s = data.to_string();
    
    // This also works:
    let s = "initial contents".to_string();
    
    // Alternative creation method
    let s = String::from("initial contents");
    
    // Growing a string
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s); // "foobar"
    
    // push_str doesn't take ownership
    let s1 = String::from("Hello, ");
    let s2 = "world!";
    let mut s = s1.clone();
    s.push_str(s2);
    println!("s2 is {}", s2); // s2 is still valid
}
```

#### String Concatenation Methods
```rust
fn main() {
    // Method 1: + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note: s1 has been moved and can no longer be used
    
    // Method 2: format! macro (doesn't take ownership)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s); // "tic-tac-toe"
    // s1, s2, s3 are still valid
}
```

#### Understanding String Indexing Limitations
```rust
fn main() {
    let s1 = String::from("hello");
    // let h = s1[0]; // This won't compile!
    
    // Why indexing doesn't work:
    let hello = String::from("Hola");     // 4 bytes
    let hello = String::from("Здравствуйте"); // 24 bytes, not 12!
    
    // Each Cyrillic character takes 2 bytes in UTF-8
    // So "indexing" by byte position would be meaningless
}
```

#### Safe String Access Methods
```rust
fn main() {
    let hello = "Здравствуйте";
    
    // Slicing by byte position (be careful!)
    let s = &hello[0..4]; // Gets first 2 characters (4 bytes)
    println!("{}", s); // "Зд"
    
    // This would panic: &hello[0..1] (cuts character in middle)
    
    // Safe iteration over characters
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // Output: न म स ् त े
    
    // Iteration over bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    // Output: 224 164 168 224 164 174 ... (18 bytes total)
}
```

#### Real-World String Processing Example
```rust
fn main() {
    // Building a user greeting system
    let mut user_data = Vec::new();
    
    // Collect user information
    let names = vec!["Alice", "Bob", "世界", "مرحبا"];
    
    for name in names {
        let greeting = format!("Hello, {}!", name);
        user_data.push(greeting);
    }
    
    // Process and display
    for (index, greeting) in user_data.iter().enumerate() {
        println!("{}: {} (length: {} chars, {} bytes)", 
                 index + 1, 
                 greeting, 
                 greeting.chars().count(),
                 greeting.len());
    }
}
```

#### String Methods and Operations
```rust
fn main() {
    let mut s = String::from("Hello");
    
    // Adding to string
    s.push_str(", world");
    s.push('!');
    
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
    println!("Replaced: {}", replaced);
    
    // Case conversion
    println!("Uppercase: {}", s.to_uppercase());
    println!("Lowercase: {}", s.to_lowercase());
    
    // Trimming whitespace
    let padded = "  hello world  ";
    println!("Trimmed: '{}'", padded.trim());
}
```

#### Working with String Slices
```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn main() {
    let my_string = String::from("hello world");
    
    // first_word works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    
    // first_word also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    
    let my_string_literal = "hello world";
    
    // first_word works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

#### Unicode and Internationalization
```rust
fn analyze_text(text: &str) {
    println!("Text: {}", text);
    println!("Byte length: {}", text.len());
    println!("Character count: {}", text.chars().count());
    
    println!("Characters:");
    for (i, c) in text.chars().enumerate() {
        println!("  {}: {}", i, c);
    }
    
    println!("Bytes:");
    for (i, b) in text.bytes().enumerate() {
        println!("  {}: {}", i, b);
    }
}

fn main() {
    // English text
    analyze_text("Hello");
    
    println!("---");
    
    // Text with accented characters
    analyze_text("Café");
    
    println!("---");
    
    // Non-Latin script
    analyze_text("こんにちは");
    
    println!("---");
    
    // Mixed scripts
    analyze_text("Hello 世界 🌍");
}
```

#### String Building and Performance
```rust
fn main() {
    // Inefficient: creates new string each time
    let mut result = String::new();
    let words = vec!["Hello", " ", "beautiful", " ", "world"];
    
    for word in &words {
        result = result + word; // Creates new string each time
    }
    
    // More efficient: push_str reuses existing capacity
    let mut result = String::new();
    for word in &words {
        result.push_str(word);
    }
    
    // Even better: pre-allocate capacity
    let mut result = String::with_capacity(100);
    for word in &words {
        result.push_str(word);
    }
    
    // Best for simple cases: use format! or join
    let result = words.join("");
    let result = format!("{}{}{}{}{}", words[0], words[1], words[2], words[3], words[4]);
}
```

#### String Parsing and Conversion
```rust
fn main() {
    // String to number conversion
    let num_str = "42";
    let num: i32 = num_str.parse().expect("Not a number!");
    println!("Parsed number: {}", num);
    
    // Handle parsing errors
    let maybe_num = "not_a_number".parse::<i32>();
    match maybe_num {
        Ok(num) => println!("Parsed: {}", num),
        Err(e) => println!("Parse error: {}", e),
    }
    
    // Number to string conversion
    let num = 42;
    let num_str = num.to_string();
    let num_str2 = format!("{}", num);
    
    // Boolean to string
    let flag = true;
    let flag_str = flag.to_string(); // "true"
    
    // Working with string collections
    let words: Vec<&str> = "hello world rust".split_whitespace().collect();
    println!("Words: {:?}", words);
    
    let rejoined = words.join("-");
    println!("Rejoined: {}", rejoined);
}
```

#### Advanced String Operations
```rust
fn main() {
    let text = "The quick brown fox jumps over the lazy dog";
    
    // Splitting strings
    let words: Vec<&str> = text.split(' ').collect();
    println!("Words: {:?}", words);
    
    let parts: Vec<&str> = text.split("brown").collect();
    println!("Split by 'brown': {:?}", parts);
    
    // Finding substrings
    if let Some(pos) = text.find("fox") {
        println!("Found 'fox' at position: {}", pos);
    }
    
    // String replacement
    let replaced = text.replace("fox", "cat");
    println!("Replaced: {}", replaced);
    
    // Lines iteration
    let multiline = "Line 1\nLine 2\nLine 3";
    for (i, line) in multiline.lines().enumerate() {
        println!("Line {}: {}", i + 1, line);
    }
    
    // Filtering characters
    let filtered: String = text.chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .collect();
    println!("Filtered: {}", filtered);
}
```

### Practical Applications
- Text processing and parsing
- User input handling
- File content manipulation
- Web development (HTML, JSON processing)
- Internationalization and localization
- Configuration file parsing

### Common String Methods
- `len()` - Byte length (not character count!)
- `chars().count()` - Character count
- `is_empty()` - Check if string is empty
- `contains(pattern)` - Search for substring
- `starts_with(pattern)` / `ends_with(pattern)` - Prefix/suffix check
- `trim()` - Remove leading/trailing whitespace
- `replace(from, to)` - Replace substrings
- `split(pattern)` - Split string into parts
- `to_uppercase()` / `to_lowercase()` - Case conversion

### Best Practices

#### Prefer string slices for parameters:
```rust
// Good: accepts both String and &str
fn process_text(text: &str) {
    println!("{}", text);
}

// Less flexible: only accepts String
fn process_text_owned(text: String) {
    println!("{}", text);
}
```

#### Use appropriate creation method:
```rust
let s1 = String::new();                    // Empty string
let s2 = String::from("hello");            // When you need String
let s3 = "hello".to_string();              // Converting from &str
let s4 = format!("Hello, {}!", name);      // Formatted string
```

### Integration with Previous Chapters
- Uses ownership rules for string data management  
- String slices demonstrate borrowing concepts
- Vectors and strings are both UTF-8 collections
- Pattern matching useful for string processing

### Community Conventions and Idioms
- Use `&str` for function parameters when you don't need ownership
- Use `String` when you need to own or modify the string
- Prefer `format!` over `+` for complex string concatenation
- Use `collect()` to build strings from iterators
- Be explicit about Unicode handling in international applications

### Performance Considerations
- String concatenation with `+` creates new strings each time
- `push_str()` is more efficient for building strings incrementally
- Pre-allocate capacity with `String::with_capacity()` when size is known
- String slicing by bytes is O(1), but must respect UTF-8 boundaries
- Character counting requires O(n) iteration

### Personal Notes
- String handling in Rust is more complex but prevents many common bugs
- The UTF-8 requirement means all strings are valid Unicode
- Understanding the difference between bytes and characters is crucial
- The borrowing rules make string sharing very efficient
- Rust's approach prevents buffer overflows and encoding errors common in other languages

Official Chapter: https://doc.rust-lang.org/book/ch08-02-strings.html

---
*Completed: ✓*