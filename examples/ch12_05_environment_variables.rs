//! # Chapter 12.5: Working with Environment Variables
//! 
//! This example demonstrates:
//! - Reading environment variables with std::env::var()
//! - Using environment variables for configuration
//! - Implementing case-insensitive search based on env vars
//! - Error handling for missing environment variables
//! 
//! Run this example with: `cargo run --example ch12_05_environment_variables -- search filename.txt`
//! For case-insensitive search: `CASE_INSENSITIVE=1 cargo run --example ch12_05_environment_variables -- search filename.txt`

use rust_book_examples::print_chapter_header;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

/// Configuration structure that uses environment variables
#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
    verbose: bool,
}

impl Config {
    /// Build configuration from command line args and environment variables
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Usage: program <query> <file_path>");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // Read environment variables for configuration
        // CASE_INSENSITIVE=1 enables case-insensitive search
        let ignore_case = env::var("CASE_INSENSITIVE").is_ok();
        
        // VERBOSE=1 enables verbose output
        let verbose = env::var("VERBOSE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
            verbose,
        })
    }
}

fn main() {
    print_chapter_header("Chapter 12.5", "Working with Environment Variables");

    println!("Grep program with environment variable configuration");
    println!();

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        println!();
        println!("Environment Variables:");
        println!("  CASE_INSENSITIVE=1  - Enable case-insensitive search");
        println!("  VERBOSE=1          - Enable verbose output");
        println!();
        println!("Examples:");
        println!("  cargo run --example ch12_05_environment_variables -- rust file.txt");
        println!("  CASE_INSENSITIVE=1 cargo run --example ch12_05_environment_variables -- rust file.txt");
        println!();
        println!("Running demo mode with sample data:");
        run_demo();
        process::exit(0);
    });

    display_config(&config);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn display_config(config: &Config) {
    println!("📋 Configuration");
    println!("  Query: '{}'", config.query);
    println!("  File: {}", config.file_path);
    println!("  Case insensitive: {}", config.ignore_case);
    println!("  Verbose mode: {}", config.verbose);
    println!();
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.verbose {
        println!("🔍 Reading file: {}", config.file_path);
    }

    let contents = fs::read_to_string(&config.file_path)?;

    if config.verbose {
        println!("✅ File read successfully ({} bytes)", contents.len());
        println!("📝 File contents preview:");
        println!("{}", "─".repeat(40));
        // Show first few lines
        for (i, line) in contents.lines().take(5).enumerate() {
            println!("{}: {}", i + 1, line);
        }
        if contents.lines().count() > 5 {
            println!("... ({} more lines)", contents.lines().count() - 5);
        }
        println!("{}", "─".repeat(40));
        println!();
    }

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    let search_type = if config.ignore_case { "case-insensitive" } else { "case-sensitive" };
    println!("🔍 {} search results for '{}':", search_type, config.query);
    println!("{}", "=".repeat(50));

    if results.is_empty() {
        println!("❌ No matches found");
    } else {
        for (line_num, line) in results.iter().enumerate() {
            if config.verbose {
                println!("Match {}: Line {}: {}", line_num + 1, get_line_number(line, &contents), line);
            } else {
                println!("{}", line);
            }
        }
        println!();
        println!("✅ Found {} matches", results.len());
    }

    Ok(())
}

/// Case-sensitive search function
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Case-insensitive search function
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

/// Helper function to get line number for verbose output
fn get_line_number(target_line: &str, contents: &str) -> usize {
    contents
        .lines()
        .enumerate()
        .find(|(_, line)| *line == target_line)
        .map(|(i, _)| i + 1)
        .unwrap_or(0)
}

fn run_demo() {
    println!();
    println!("=== Environment Variables Demo ===");
    
    demonstrate_env_var_reading();
    demonstrate_search_with_env_vars();
}

/// Demonstrates reading various environment variables
fn demonstrate_env_var_reading() {
    println!("📋 Environment Variable Examples:");
    println!();

    // Common environment variables
    let env_vars = [
        "PATH",
        "HOME", 
        "USER",
        "CASE_INSENSITIVE",
        "VERBOSE",
        "RUST_LOG",
    ];

    for var_name in &env_vars {
        match env::var(var_name) {
            Ok(value) => {
                // Truncate long values like PATH
                let display_value = if value.len() > 50 {
                    format!("{}...", &value[..47])
                } else {
                    value
                };
                println!("  {} = {}", var_name, display_value);
            }
            Err(_) => {
                println!("  {} = (not set)", var_name);
            }
        }
    }
    println!();
}

/// Demonstrates search functionality with environment variable configuration
fn demonstrate_search_with_env_vars() {
    let sample_content = "Rust is a systems programming language.
The Rust Programming Language book is excellent.
Python and RUST are both popular.
Java developers often learn rust.
Go and Rust have different philosophies.
JAVASCRIPT and rust serve different purposes.";

    println!("Sample text for searching:");
    println!("{}", "─".repeat(40));
    println!("{}", sample_content);
    println!("{}", "─".repeat(40));
    println!();

    // Simulate different environment variable settings
    println!("🔍 Search Demonstrations:");
    println!();

    // Case-sensitive search (default)
    println!("1. Case-sensitive search for 'rust':");
    let results = search("rust", sample_content);
    display_demo_results(&results);

    println!("2. Case-sensitive search for 'Rust':");
    let results = search("Rust", sample_content);
    display_demo_results(&results);

    println!("3. Case-sensitive search for 'RUST':");
    let results = search("RUST", sample_content);
    display_demo_results(&results);

    // Case-insensitive search (simulating CASE_INSENSITIVE=1)
    println!("4. Case-insensitive search for 'rust' (CASE_INSENSITIVE=1):");
    let results = search_case_insensitive("rust", sample_content);
    display_demo_results(&results);

    println!();
    println!("💡 Environment Variable Benefits:");
    println!("• Configure program behavior without recompilation");
    println!("• Different settings for different environments");
    println!("• User can override defaults easily");
    println!("• No need to modify code for configuration changes");
    println!();
    println!("Try setting environment variables:");
    println!("  CASE_INSENSITIVE=1 cargo run --example ch12_05_environment_variables");
    println!("  VERBOSE=1 CASE_INSENSITIVE=1 cargo run --example ch12_05_environment_variables");
}

fn display_demo_results(results: &[&str]) {
    if results.is_empty() {
        println!("   ❌ No matches");
    } else {
        for result in results {
            println!("   ✅ {}", result);
        }
    }
    println!();
}

// === ADVANCED ENVIRONMENT VARIABLE PATTERNS ===

/// Example of more advanced environment variable handling
#[allow(dead_code)]
fn advanced_env_examples() {
    // Pattern 1: Environment variable with default value
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    println!("Log level: {}", log_level);

    // Pattern 2: Parse environment variable to specific type
    let max_results: usize = env::var("MAX_RESULTS")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .unwrap_or(10);
    println!("Max results: {}", max_results);

    // Pattern 3: Boolean environment variable
    let debug_mode = env::var("DEBUG").map(|v| v == "1" || v.to_lowercase() == "true").unwrap_or(false);
    println!("Debug mode: {}", debug_mode);

    // Pattern 4: List from environment variable
    let search_paths: Vec<String> = env::var("SEARCH_PATHS")
        .unwrap_or_else(|_| ".".to_string())
        .split(':')
        .map(|s| s.to_string())
        .collect();
    println!("Search paths: {:?}", search_paths);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_case_sensitive() {
        let contents = "Rust\nrust\nRUST";
        
        assert_eq!(vec!["Rust"], search("Rust", contents));
        assert_eq!(vec!["rust"], search("rust", contents));
        assert_eq!(vec!["RUST"], search("RUST", contents));
    }

    #[test]
    fn test_search_case_insensitive() {
        let contents = "Rust\nrust\nRUST\nJava";
        let results = search_case_insensitive("rust", contents);
        
        assert_eq!(3, results.len());
        assert!(results.contains(&"Rust"));
        assert!(results.contains(&"rust"));
        assert!(results.contains(&"RUST"));
        assert!(!results.contains(&"Java"));
    }

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
        // ignore_case and verbose depend on environment variables
    }

    #[test]
    fn test_config_build_insufficient_args() {
        let args = vec!["program".to_string()];
        let result = Config::build(&args);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_get_line_number() {
        let contents = "line 1\nline 2\nline 3";
        
        assert_eq!(1, get_line_number("line 1", contents));
        assert_eq!(2, get_line_number("line 2", contents));
        assert_eq!(3, get_line_number("line 3", contents));
        assert_eq!(0, get_line_number("nonexistent", contents));
    }
}