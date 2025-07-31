//! # Chapter 12.2: Reading a File
//! 
//! This example demonstrates:
//! - Reading files using std::fs::read_to_string()
//! - Handling file I/O errors
//! - Processing command line arguments
//! - Basic file content manipulation
//! 
//! Run this example with: `cargo run --example ch12_02_reading_files -- search_term filename.txt`

use rust_book_examples::print_chapter_header;
use std::env;
use std::fs;
use std::process;

fn main() {
    print_chapter_header("Chapter 12.2", "Reading a File");

    println!("Building a simple grep-like program that reads files...");
    println!();

    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if we have enough arguments
    if args.len() < 3 {
        println!("Usage: {} <search_term> <filename>", args[0]);
        println!();
        println!("Since no arguments provided, using demo mode with sample data:");
        demonstrate_file_reading();
        return;
    }

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for: '{}'", query);
    println!("In file: {}", file_path);
    println!();

    // Read the file
    match read_and_search(query, file_path) {
        Ok(()) => println!("Search completed successfully!"),
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            process::exit(1);
        }
    }
}

/// Reads a file and searches for a query string
fn read_and_search(query: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📖 Reading file: {}", file_path);
    
    // Read the entire file into a string
    // This loads the whole file into memory - fine for small files
    let contents = fs::read_to_string(file_path)?;
    
    println!("✅ File read successfully! ({} bytes)", contents.len());
    println!();
    
    // Display file contents
    println!("File contents:");
    println!("{}", "=".repeat(50));
    println!("{}", contents);
    println!("{}", "=".repeat(50));
    println!();
    
    // Perform basic search
    search_in_content(query, &contents);
    
    Ok(())
}

/// Searches for query string in file contents and displays results
fn search_in_content(query: &str, contents: &str) {
    println!("🔍 Searching for: '{}'", query);
    println!();
    
    let mut found_count = 0;
    let mut line_number = 1;
    
    for line in contents.lines() {
        if line.contains(query) {
            found_count += 1;
            println!("Line {}: {}", line_number, line);
        }
        line_number += 1;
    }
    
    println!();
    if found_count > 0 {
        println!("✅ Found {} matches for '{}'", found_count, query);
    } else {
        println!("❌ No matches found for '{}'", query);
    }
}

/// Demonstrates file reading with sample data when no args provided
fn demonstrate_file_reading() {
    println!("=== Demo Mode: File Reading Examples ===");
    println!();
    
    // Create sample content in memory (simulating a file)
    let sample_content = "The quick brown fox jumps over the lazy dog.
Rust is a systems programming language.
File reading in Rust is safe and efficient.
Error handling prevents common bugs.
The fox is clever and quick.";
    
    println!("Sample file content:");
    println!("{}", "-".repeat(40));
    println!("{}", sample_content);
    println!("{}", "-".repeat(40));
    println!();
    
    // Demonstrate different search operations
    demonstrate_search("fox", sample_content);
    demonstrate_search("Rust", sample_content);
    demonstrate_search("python", sample_content); // No matches
    
    println!();
    println!("📝 File Reading Best Practices:");
    println!("• Use fs::read_to_string() for text files that fit in memory");
    println!("• Handle errors with Result type and ? operator");
    println!("• Consider using BufRead for large files");
    println!("• Always validate file paths and handle I/O errors gracefully");
    println!();
    println!("Try running with actual files:");
    println!("cargo run --example ch12_02_reading_files -- 'search_term' 'path/to/file.txt'");
}

/// Helper function to demonstrate search functionality
fn demonstrate_search(query: &str, content: &str) {
    println!("Searching for '{}':", query);
    
    let matches: Vec<_> = content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .collect();
    
    if matches.is_empty() {
        println!("  ❌ No matches found");
    } else {
        for (line_num, line) in matches {
            println!("  Line {}: {}", line_num + 1, line);
        }
    }
    println!();
}

// === ADDITIONAL FILE READING EXAMPLES ===

/// Example of different file reading approaches
#[allow(dead_code)]
fn file_reading_examples() {
    use std::fs;
    use std::io::{self, BufRead, BufReader};
    
    // Method 1: Read entire file to string (what we used above)
    fn read_whole_file(path: &str) -> io::Result<String> {
        fs::read_to_string(path)
    }
    
    // Method 2: Read file as bytes
    fn read_file_bytes(path: &str) -> io::Result<Vec<u8>> {
        fs::read(path)
    }
    
    // Method 3: Read file line by line (memory efficient for large files)
    fn read_file_lines(path: &str) -> io::Result<Vec<String>> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let lines: Result<Vec<_>, _> = reader.lines().collect();
        lines
    }
    
    println!("Different file reading methods available:");
    println!("1. fs::read_to_string() - entire file as String");
    println!("2. fs::read() - entire file as Vec<u8>");
    println!("3. BufReader with lines() - line-by-line reading");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_in_content() {
        let content = "line one\nline two\nline three";
        
        // This would normally print, but in tests we'd capture the output
        // For now, we'll test the underlying logic
        
        let matches: Vec<_> = content
            .lines()
            .filter(|line| line.contains("line"))
            .collect();
        
        assert_eq!(matches.len(), 3);
    }

    #[test]
    fn test_search_no_matches() {
        let content = "hello world";
        
        let matches: Vec<_> = content
            .lines()
            .filter(|line| line.contains("xyz"))
            .collect();
        
        assert_eq!(matches.len(), 0);
    }
}