//! # Chapter 12.3: Refactoring to Improve Modularity and Error Handling
//! 
//! This example demonstrates:
//! - Separating concerns with configuration structs
//! - Proper error handling with Result types
//! - Extracting functions for better modularity
//! - Making code more testable and maintainable
//! 
//! Run this example with: `cargo run --example ch12_03_refactoring_modularity -- search_term filename.txt`

use rust_book_examples::print_chapter_header;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

/// Configuration structure to hold command line arguments
#[derive(Debug, Clone)]
struct Config {
    query: String,
    file_path: String,
    case_sensitive: bool,
}

impl Config {
    /// Build a Config from command line arguments
    /// 
    /// # Arguments
    /// * `args` - Slice of command line arguments
    /// 
    /// # Returns
    /// * `Result<Config, &'static str>` - Config on success, error message on failure
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        
        // Check for case sensitivity flag
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            file_path,
            case_sensitive,
        })
    }
    
    /// Alternative constructor using iterators (more idiomatic)
    fn build_from_iter(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // Skip program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            file_path,
            case_sensitive,
        })
    }
}

fn main() {
    print_chapter_header("Chapter 12.3", "Refactoring to Improve Modularity and Error Handling");

    println!("Improved grep program with better error handling and modularity");
    println!();

    let args: Vec<String> = env::args().collect();

    // Use the new Config::build method
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        println!();
        println!("Usage: {} <search_term> <filename>", args[0]);
        println!("Set CASE_INSENSITIVE=1 for case-insensitive search");
        println!();
        println!("Since no valid arguments provided, running demo mode:");
        run_demo();
        process::exit(0);
    });

    println!("Configuration:");
    println!("  Query: '{}'", config.query);
    println!("  File: {}", config.file_path);
    println!("  Case sensitive: {}", config.case_sensitive);
    println!();

    // Run the main program logic
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

/// Main program logic separated from argument parsing
/// 
/// # Arguments
/// * `config` - Configuration containing query and file path
/// 
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Success or error
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("📖 Reading file: {}", config.file_path);
    
    let contents = fs::read_to_string(config.file_path)?;
    
    println!("✅ File read successfully! ({} bytes)", contents.len());
    println!();

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    println!("🔍 Search results for '{}':", config.query);
    println!("{}", "=".repeat(50));
    
    if results.is_empty() {
        println!("❌ No matches found");
    } else {
        let count = results.len();
        for (line_num, line) in results {
            println!("Line {}: {}", line_num, line);
        }
        println!("\n✅ Found {} matches", count);
    }

    Ok(())
}

/// Search for query in contents (case sensitive)
/// 
/// # Arguments
/// * `query` - String to search for
/// * `contents` - Text to search in
/// 
/// # Returns
/// * `Vec<(usize, String)>` - Vector of (line_number, line_content) tuples
fn search(query: &str, contents: &str) -> Vec<(usize, String)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(i, line)| (i + 1, line.to_string()))
        .collect()
}

/// Search for query in contents (case insensitive)
/// 
/// # Arguments
/// * `query` - String to search for
/// * `contents` - Text to search in
/// 
/// # Returns
/// * `Vec<(usize, String)>` - Vector of (line_number, line_content) tuples
fn search_case_insensitive(query: &str, contents: &str) -> Vec<(usize, String)> {
    let query = query.to_lowercase();
    
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&query))
        .map(|(i, line)| (i + 1, line.to_string()))
        .collect()
}

/// Demonstrates the improved modularity with sample data
fn run_demo() {
    println!();
    println!("=== Demo Mode: Refactored grep Program ===");
    
    let sample_content = "The quick brown FOX jumps over the lazy dog.
Rust is a systems programming language that focuses on safety.
File reading in Rust is safe and efficient.
Error handling prevents common programming bugs.
The fox is clever and quick with its movements.
RUST provides zero-cost abstractions.
Functional programming features work great with imperative code.";

    println!("Sample file content:");
    println!("{}", "-".repeat(50));
    println!("{}", sample_content);
    println!("{}", "-".repeat(50));
    println!();

    // Demonstrate case-sensitive search
    println!("Case-sensitive search for 'fox':");
    let results = search("fox", sample_content);
    display_results(&results);
    
    println!("Case-sensitive search for 'FOX':");
    let results = search("FOX", sample_content);
    display_results(&results);
    
    println!("Case-insensitive search for 'fox':");
    let results = search_case_insensitive("fox", sample_content);
    display_results(&results);
    
    println!("Case-insensitive search for 'rust':");
    let results = search_case_insensitive("rust", sample_content);
    display_results(&results);
    
    println!();
    println!("🏗️  Refactoring Benefits:");
    println!("• Separation of concerns - parsing vs logic");
    println!("• Better error handling with Result types");  
    println!("• Testable functions with clear inputs/outputs");
    println!("• Configuration struct makes extending easier");
    println!("• Main function focuses on coordination, not details");
}

/// Helper function to display search results
fn display_results(results: &[(usize, String)]) {
    if results.is_empty() {
        println!("  ❌ No matches found");
    } else {
        for (line_num, line) in results {
            println!("  Line {}: {}", line_num, line);
        }
        println!("  ✅ Total matches: {}", results.len());
    }
    println!();
}

// === ADDITIONAL EXAMPLES ===

/// Example of using the iterator-based Config constructor
#[allow(dead_code)]
fn demonstrate_iterator_config() {
    let args = vec![
        "program".to_string(),
        "query".to_string(),
        "file.txt".to_string(),
    ];
    
    match Config::build_from_iter(args.into_iter()) {
        Ok(config) => println!("Iterator config: {:?}", config),
        Err(e) => println!("Error: {}", e),
    }
}

/// Example of different error handling strategies
#[allow(dead_code)]
fn error_handling_examples() {
    // Strategy 1: expect() - panic on error (use for unrecoverable errors)
    // let config = Config::build(&args).expect("Failed to parse arguments");
    
    // Strategy 2: unwrap_or_else() - provide fallback behavior
    // let config = Config::build(&args).unwrap_or_else(|_| Config::default());
    
    // Strategy 3: match - handle each case explicitly
    // match Config::build(&args) {
    //     Ok(config) => { /* use config */ },
    //     Err(e) => { /* handle error */ },
    // }
    
    // Strategy 4: ? operator - propagate error up the call stack
    // fn parse_config(args: &[String]) -> Result<Config, Box<dyn Error>> {
    //     let config = Config::build(args)?;
    //     Ok(config)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_build_success() {
        let args = vec![
            "program".to_string(),
            "query".to_string(),
            "file.txt".to_string(),
        ];
        
        let config = Config::build(&args).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file.txt");
    }

    #[test]
    fn test_config_build_failure() {
        let args = vec!["program".to_string()];
        
        let result = Config::build(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_search_case_sensitive() {
        let contents = "Rust is great\nrust is fun\nRUST rocks";
        let results = search("Rust", contents);
        
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, 1); // Line number
        assert!(results[0].1.contains("Rust is great"));
    }

    #[test]
    fn test_search_case_insensitive() {
        let contents = "Rust is great\nrust is fun\nRUST rocks";
        let results = search_case_insensitive("rust", contents);
        
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_search_no_results() {
        let contents = "Hello world";
        let results = search("xyz", contents);
        
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_config_from_iterator() {
        let args = vec![
            "program".to_string(),
            "search".to_string(),
            "file.txt".to_string(),
        ];
        
        let config = Config::build_from_iter(args.into_iter()).unwrap();
        assert_eq!(config.query, "search");
        assert_eq!(config.file_path, "file.txt");
    }
}