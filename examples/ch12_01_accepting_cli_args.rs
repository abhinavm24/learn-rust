//! # Chapter 12.1: Accepting Command Line Arguments
//! 
//! This example demonstrates:
//! - Reading command line arguments with std::env::args
//! - Building a simple command line application
//! - File I/O operations
//! - Error handling in CLI applications
//! - Separating concerns with modules and functions
//! - Creating a minigrep-like application

use rust_book_examples::print_chapter_header;
use std::env;
use std::fs;
use std::error::Error;
use std::process;

fn main() {
    print_chapter_header("Chapter 12.1", "Accepting Command Line Arguments");

    println!("=== Basic Command Line Argument Reading ===");
    basic_args_example();
    
    println!("\n=== Building a Simple CLI Application ===");
    
    // Try to run our minigrep application
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for '{}' in file '{}'", config.query, config.file_path);
    
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn basic_args_example() {
    let args: Vec<String> = env::args().collect();
    
    println!("Number of arguments: {}", args.len());
    println!("Program name: {}", args[0]);
    
    if args.len() > 1 {
        println!("Arguments:");
        for (index, arg) in args.iter().enumerate().skip(1) {
            println!("  [{}]: {}", index, arg);
        }
    } else {
        println!("No additional arguments provided");
        println!("Try running: cargo run --example ch12_01_accepting_cli_args hello world");
    }
}

// Configuration struct to hold our command line arguments
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // Skip the program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Create a sample file if it doesn't exist
    let sample_content = r#"I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!"#;

    // For this example, we'll use a default content if the file doesn't exist
    let contents = match fs::read_to_string(&config.file_path) {
        Ok(content) => content,
        Err(_) => {
            println!("File '{}' not found, using sample text:", config.file_path);
            sample_content.to_string()
        }
    };

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    if results.is_empty() {
        println!("No matches found for '{}'", config.query);
    } else {
        println!("Found {} match(es):", results.len());
        for line in results {
            println!("  {}", line);
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

// Demonstrate different CLI patterns
fn demonstrate_cli_patterns() {
    println!("=== CLI Application Patterns ===");
    
    // Pattern 1: Simple argument parsing
    fn parse_simple_args() -> Result<(String, String), &'static str> {
        let args: Vec<String> = env::args().collect();
        
        if args.len() < 3 {
            return Err("Usage: program <query> <filename>");
        }
        
        Ok((args[1].clone(), args[2].clone()))
    }
    
    // Pattern 2: Using clap-like argument parsing (simplified)
    #[derive(Debug)]
    struct CliConfig {
        query: String,
        file: String,
        verbose: bool,
        ignore_case: bool,
    }
    
    fn parse_advanced_args() -> Result<CliConfig, String> {
        let args: Vec<String> = env::args().collect();
        let mut config = CliConfig {
            query: String::new(),
            file: String::new(),
            verbose: false,
            ignore_case: false,
        };
        
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-v" | "--verbose" => config.verbose = true,
                "-i" | "--ignore-case" => config.ignore_case = true,
                arg if arg.starts_with('-') => {
                    return Err(format!("Unknown flag: {}", arg));
                }
                _ => {
                    if config.query.is_empty() {
                        config.query = args[i].clone();
                    } else if config.file.is_empty() {
                        config.file = args[i].clone();
                    } else {
                        return Err("Too many arguments".to_string());
                    }
                }
            }
            i += 1;
        }
        
        if config.query.is_empty() || config.file.is_empty() {
            return Err("Missing required arguments: query and file".to_string());
        }
        
        Ok(config)
    }
    
    match parse_simple_args() {
        Ok((query, file)) => println!("Simple parsing: query='{}', file='{}'", query, file),
        Err(e) => println!("Simple parsing error: {}", e),
    }
    
    match parse_advanced_args() {
        Ok(config) => println!("Advanced parsing: {:?}", config),
        Err(e) => println!("Advanced parsing error: {}", e),
    }
}

// File operations for CLI applications
mod file_operations {
    use std::fs;
    use std::io::{self, Write};
    use std::path::Path;
    
    pub fn read_file_safe(path: &str) -> Result<String, String> {
        fs::read_to_string(path)
            .map_err(|e| format!("Error reading file '{}': {}", path, e))
    }
    
    pub fn write_file_safe(path: &str, content: &str) -> Result<(), String> {
        fs::write(path, content)
            .map_err(|e| format!("Error writing file '{}': {}", path, e))
    }
    
    pub fn file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }
    
    pub fn interactive_input(prompt: &str) -> Result<String, io::Error> {
        print!("{}", prompt);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }
    
    pub fn create_sample_file(filename: &str) -> Result<(), String> {
        let content = r#"The quick brown fox jumps over the lazy dog.
Rust is a systems programming language that runs blazingly fast.
Memory safety without garbage collection.
Zero-cost abstractions in Rust.
Hello, World! This is a sample text file.
Grep is a command-line utility for searching text patterns.
"#;
        write_file_safe(filename, content)
    }
}

// Error handling patterns for CLI applications
mod error_handling {
    use std::fmt;
    
    #[derive(Debug)]
    pub enum CliError {
        InvalidArguments(String),
        FileError(String),
        ProcessingError(String),
    }
    
    impl fmt::Display for CliError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                CliError::InvalidArguments(msg) => write!(f, "Invalid arguments: {}", msg),
                CliError::FileError(msg) => write!(f, "File error: {}", msg),
                CliError::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
            }
        }
    }
    
    impl std::error::Error for CliError {}
    
    pub fn handle_cli_error(error: CliError) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

// Testing module for our CLI application
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn config_build_success() {
        let args = vec![
            "program".to_string(),
            "query".to_string(),
            "filename.txt".to_string(),
        ];
        
        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "filename.txt");
    }

    #[test]
    fn config_build_missing_args() {
        let args = vec!["program".to_string()];
        let result = Config::build(args.into_iter());
        assert!(result.is_err());
    }

    #[test]
    fn no_matches() {
        let query = "xyz";
        let contents = "Hello, world!";
        let results = search(query, contents);
        assert!(results.is_empty());
    }

    #[test]
    fn multiple_matches() {
        let query = "the";
        let contents = "the quick brown fox\njumps over the lazy dog\nthe end";
        let results = search(query, contents);
        assert_eq!(results.len(), 3);
    }
}