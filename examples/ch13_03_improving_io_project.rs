//! # Chapter 13.3: Improving Our I/O Project
//! 
//! This example demonstrates:
//! - Refactoring the grep project to use iterators
//! - Replacing manual loops with iterator methods
//! - Functional programming style in Rust
//! - Performance benefits of iterator chains
//! 
//! Run this example with: `cargo run --example ch13_03_improving_io_project -- search_term filename.txt`

use rust_book_examples::print_chapter_header;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

/// Configuration using iterator-based argument parsing
#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    /// Build configuration using iterator instead of indexing
    /// This is more idiomatic and handles edge cases better
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // Skip program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("CASE_INSENSITIVE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    /// Alternative: Old indexing-based approach (for comparison)
    #[allow(dead_code)]
    fn build_old_way(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("CASE_INSENSITIVE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn main() {
    print_chapter_header("Chapter 13.3", "Improving Our I/O Project");

    println!("Refactored grep using iterators and functional programming style");
    println!();

    // Use iterator-based config building
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        println!();
        println!("Usage: program <query> <file_path>");
        println!("Set CASE_INSENSITIVE=1 for case-insensitive search");
        println!();
        println!("Running demo with sample data:");
        run_demo();
        process::exit(0);
    });

    println!("📋 Configuration:");
    println!("  Query: '{}'", config.query);
    println!("  File: {}", config.file_path);
    println!("  Case insensitive: {}", config.ignore_case);
    println!();

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    println!("🔍 Search Results:");
    println!("{}", "=".repeat(50));

    if results.is_empty() {
        println!("❌ No matches found for '{}'", config.query);
    } else {
        for (line_num, line) in results.iter().enumerate() {
            println!("{}. {}", line_num + 1, line);
        }
        println!("\n✅ Found {} matches", results.len());
    }

    Ok(())
}

/// Case-sensitive search using iterators (improved version)
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Case-insensitive search using iterators (improved version)  
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

/// Old implementations for comparison (using manual loops)
mod old_implementations {
    #[allow(dead_code)]
    pub fn search_old<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }

        results
    }

    #[allow(dead_code)]
    pub fn search_case_insensitive_old<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let query = query.to_lowercase();
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }

        results
    }
}

fn run_demo() {
    println!();
    println!("=== Iterator-Based Grep Demo ===");
    
    demonstrate_config_improvements();
    demonstrate_search_improvements();
    demonstrate_functional_style();
}

/// Demonstrates improvements in config parsing
fn demonstrate_config_improvements() {
    println!("📋 Config Parsing Improvements:");
    println!();
    
    // Show difference between old and new approaches
    println!("Old approach (indexing):");
    println!("  - Requires length checking");
    println!("  - Uses clone() for each argument");
    println!("  - Less idiomatic Rust");
    println!();
    
    println!("New approach (iterators):");
    println!("  - More explicit error handling");
    println!("  - Consumes arguments directly");
    println!("  - More functional style");
    println!();
    
    // Simulate argument parsing
    let sample_args = vec![
        "program".to_string(),
        "rust".to_string(),
        "sample.txt".to_string(),
    ];
    
    match Config::build(sample_args.into_iter()) {
        Ok(config) => {
            println!("✅ Successfully parsed config: {:?}", config);
        }
        Err(e) => {
            println!("❌ Error parsing config: {}", e);
        }
    }
    
    println!();
}

/// Demonstrates improvements in search functionality
fn demonstrate_search_improvements() {
    println!("🔍 Search Function Improvements:");
    
    let sample_content = "The Rust Programming Language
Rust is fast and memory-safe
Without compromising performance
RUST has zero-cost abstractions
Programming in rust is enjoyable";
    
    println!("\nSample content:");
    println!("{}", "─".repeat(40));
    println!("{}", sample_content);
    println!("{}", "─".repeat(40));
    
    // Compare old vs new implementations
    println!("\n📊 Comparing Implementations:");
    
    let query = "rust";
    
    // New iterator-based approach
    let new_results = search_case_insensitive(query, sample_content);
    println!("Iterator approach results for '{}':", query);
    for (i, result) in new_results.iter().enumerate() {
        println!("  {}. {}", i + 1, result);
    }
    
    // Old loop-based approach (same results)
    let old_results = old_implementations::search_case_insensitive_old(query, sample_content);
    println!("\nLoop approach results (same logic):");
    for (i, result) in old_results.iter().enumerate() {
        println!("  {}. {}", i + 1, result);
    }
    
    println!("\n✅ Results are identical: {}", new_results == old_results);
    println!();
}

/// Demonstrates functional programming style benefits
fn demonstrate_functional_style() {
    println!("🚀 Functional Programming Style Benefits:");
    println!();
    
    let content = "line 1: hello world
line 2: goodbye world  
line 3: hello rust
line 4: rust programming
line 5: world peace";
    
    println!("Advanced search examples using iterator chains:");
    println!();
    
    // 1. Find lines containing word and get their lengths
    let word_lines_with_lengths: Vec<(usize, &str)> = content
        .lines()
        .filter(|line| line.contains("world"))
        .map(|line| (line.len(), line))
        .collect();
    
    println!("1. Lines containing 'world' with their lengths:");
    for (length, line) in &word_lines_with_lengths {
        println!("   {} chars: {}", length, line);
    }
    
    // 2. Count words in matching lines
    let total_words: usize = content
        .lines()
        .filter(|line| line.contains("hello"))
        .map(|line| line.split_whitespace().count())
        .sum();
    
    println!("\n2. Total words in lines containing 'hello': {}", total_words);
    
    // 3. Get unique words from matching lines
    let unique_words: std::collections::HashSet<&str> = content
        .lines()
        .filter(|line| line.contains("rust"))
        .flat_map(|line| line.split_whitespace())
        .collect();
    
    println!("\n3. Unique words from lines containing 'rust':");
    let mut sorted_words: Vec<&str> = unique_words.into_iter().collect();
    sorted_words.sort();
    println!("   {:?}", sorted_words);
    
    // 4. Find the longest line containing a query
    let longest_line = content
        .lines()
        .filter(|line| line.contains("line"))
        .max_by_key(|line| line.len());
    
    println!("\n4. Longest line containing 'line':");
    match longest_line {
        Some(line) => println!("   {} ({} chars)", line, line.len()),
        None => println!("   No matches found"),
    }
    
    // 5. Complex processing pipeline
    let processed_results: Vec<String> = content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.len() > 15)      // Lines longer than 15 chars
        .map(|(i, line)| format!("Line {}: {} characters", i + 1, line.len()))
        .collect();
    
    println!("\n5. Complex processing (lines > 15 chars with metadata):");
    for result in &processed_results {
        println!("   {}", result);
    }
    
    println!();
    println!("💡 Iterator Benefits:");
    println!("• Composable - chain operations naturally");
    println!("• Lazy - only process what's needed");
    println!("• Readable - expresses intent clearly");
    println!("• Efficient - zero-cost abstractions");
    println!("• Functional - encourages immutable data flow");
}

/// Advanced search functionality using iterators
#[allow(dead_code)]
fn advanced_search_examples() {
    let content = "This is a sample file.
It contains multiple lines.
Some lines have the word 'sample' in them.
Others talk about Rust programming.
The SAMPLE word appears in different cases.";

    // Multi-word search
    fn search_multiple_words<'a>(queries: &[&str], contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| {
                queries.iter().all(|query| 
                    line.to_lowercase().contains(&query.to_lowercase())
                )
            })
            .collect()
    }

    // Search with context (lines before/after)
    fn search_with_context<'a>(query: &str, contents: &'a str, context: usize) -> Vec<(usize, &'a str)> {
        let lines: Vec<&str> = contents.lines().collect();
        let mut results = Vec::new();
        
        for (i, line) in lines.iter().enumerate() {
            if line.contains(query) {
                let start = i.saturating_sub(context);
                let end = std::cmp::min(i + context + 1, lines.len());
                
                for j in start..end {
                    results.push((j + 1, lines[j]));
                }
            }
        }
        
        results.sort_by_key(|&(line_num, _)| line_num);
        results.dedup();
        results
    }

    // Count occurrences per line
    fn count_occurrences_per_line(query: &str, contents: &str) -> Vec<(usize, usize)> {
        contents
            .lines()
            .enumerate()
            .map(|(i, line)| {
                let count = line.matches(query).count();
                (i + 1, count)
            })
            .filter(|(_, count)| *count > 0)
            .collect()
    }

    println!("Advanced search examples:");
    
    let multi_results = search_multiple_words(&["sample", "file"], content);
    println!("Multi-word search: {:?}", multi_results);
    
    let context_results = search_with_context("sample", content, 1);
    println!("Search with context: {:?}", context_results);
    
    let count_results = count_occurrences_per_line("sample", content);
    println!("Occurrence counts: {:?}", count_results);
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
        
        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file.txt");
    }

    #[test]
    fn test_config_build_missing_query() {
        let args = vec!["program".to_string()];
        let result = Config::build(args.into_iter());
        assert!(result.is_err());
    }

    #[test]
    fn test_config_build_missing_file() {
        let args = vec!["program".to_string(), "query".to_string()];
        let result = Config::build(args.into_iter());
        assert!(result.is_err());
    }

    #[test]
    fn test_search_case_sensitive() {
        let contents = "Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search("duct", contents));
    }

    #[test]
    fn test_search_case_insensitive() {
        let contents = "Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape."],
            search_case_insensitive("duct", contents)
        );
    }

    #[test]
    fn test_old_vs_new_implementations() {
        let contents = "Hello World
hello world
HELLO WORLD";
        
        let new_results = search_case_insensitive("hello", contents);
        let old_results = old_implementations::search_case_insensitive_old("hello", contents);
        
        assert_eq!(new_results, old_results);
        assert_eq!(new_results.len(), 3);
    }

    #[test]
    fn test_iterator_chaining() {
        let contents = "line 1
line 2 with extra content
line 3
another line with content";
        
        // Test complex iterator chain
        let long_lines: Vec<&str> = contents
            .lines()
            .filter(|line| line.len() > 10)
            .collect();
        
        assert_eq!(long_lines.len(), 2);
        assert!(long_lines.contains(&"line 2 with extra content"));
        assert!(long_lines.contains(&"another line with content"));
    }
}