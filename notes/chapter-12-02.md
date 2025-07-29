# Chapter 12.2: Reading a File

## Key Takeaways

### File Reading Basics
- **std::fs::read_to_string()**: Convenient function for reading entire file
- **Error Handling**: File operations can fail and return Result
- **Path Handling**: Use file paths from command line arguments
- **Memory Considerations**: Entire file loaded into memory

### Basic File Reading
```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let query = &args[1];
    let file_path = &args[2];
    
    println!("Searching for {}", query);
    println!("In file {}", file_path);
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    println!("With text:\n{contents}");
}
```

### Error Handling Approaches
- **expect()**: Panic with custom message on error
- **unwrap()**: Panic with default message on error
- **match**: Handle Ok and Err cases explicitly
- **?**: Propagate errors to calling function

### File Reading with Error Handling
```rust
use std::fs;
use std::io;

fn read_file(file_path: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_path)
}

fn main() {
    match read_file("example.txt") {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(error) => println!("Error reading file: {}", error),
    }
}
```

### Alternative File Reading Methods
- **File::open()**: Open file handle for more control
- **BufReader**: Buffered reading for large files
- **read()**: Read into byte buffer
- **read_to_end()**: Read all bytes into Vec<u8>

### File Path Considerations
- **Relative Paths**: Relative to current working directory
- **Absolute Paths**: Full path from filesystem root
- **Path Validation**: Check if file exists before reading
- **Security**: Validate paths to prevent directory traversal

### Memory and Performance
- **Small Files**: read_to_string() is convenient
- **Large Files**: Consider streaming approaches
- **Memory Usage**: Entire file contents in memory
- **Error Recovery**: Handle file not found, permission errors

Official Chapter: https://doc.rust-lang.org/book/ch12-02-reading-a-file.html

---
*Completed: ✓*