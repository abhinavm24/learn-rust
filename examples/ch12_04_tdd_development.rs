//! # Chapter 12.4: Developing the Library's Functionality with Test-Driven Development
//! 
//! This example demonstrates:
//! - Test-driven development (TDD) methodology
//! - Writing tests before implementation
//! - Red-Green-Refactor cycle
//! - Building functionality incrementally
//! 
//! Run this example with: `cargo run --example ch12_04_tdd_development`
//! Run tests with: `cargo test --example ch12_04_tdd_development`

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 12.4", "Developing the Library's Functionality with Test-Driven Development");

    println!("Test-Driven Development (TDD) Example");
    println!("=====================================");
    println!();
    
    println!("TDD follows the Red-Green-Refactor cycle:");
    println!("🔴 RED: Write a failing test");
    println!("🟢 GREEN: Write minimal code to make test pass");
    println!("🔄 REFACTOR: Improve code while keeping tests passing");
    println!();

    demonstrate_search_functionality();
    demonstrate_tdd_process();
}

/// Demonstrates the search functionality we'll build with TDD
fn demonstrate_search_functionality() {
    println!("=== Search Functionality Demo ===");
    
    let contents = "Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    println!("Sample text:");
    println!("{}", contents);
    println!();

    // Test our search function
    let query = "duct";
    let results = search(query, contents);
    
    println!("Searching for '{}' (case-sensitive):", query);
    for result in &results {
        println!("  Found: {}", result);
    }
    
    if results.is_empty() {
        println!("  No matches found!");
    }
    println!();

    // Test case-insensitive search
    let results_insensitive = search_case_insensitive(query, contents);
    println!("Searching for '{}' (case-insensitive):", query);
    for result in &results_insensitive {
        println!("  Found: {}", result);
    }
    println!();
}

/// Demonstrates the TDD process step by step
fn demonstrate_tdd_process() {
    println!("=== TDD Process Demonstration ===");
    println!();
    
    println!("Step 1: 🔴 RED - Write failing tests first");
    println!("   We write tests that describe the behavior we want");
    println!("   Tests fail because implementation doesn't exist yet");
    println!();
    
    println!("Step 2: 🟢 GREEN - Write minimal implementation");
    println!("   Write just enough code to make tests pass");
    println!("   Don't worry about perfect design yet");
    println!();
    
    println!("Step 3: 🔄 REFACTOR - Improve the design");
    println!("   Clean up code while keeping tests green");
    println!("   Improve performance, readability, maintainability");
    println!();
    
    println!("Benefits of TDD:");
    println!("• Forces you to think about API design first");
    println!("• Provides safety net for refactoring");
    println!("• Creates living documentation through tests");
    println!("• Leads to more testable, modular code");
    println!("• Prevents over-engineering");
    
    println!();
    println!("Run 'cargo test --example ch12_04_tdd_development' to see TDD tests in action!");
}

// === SEARCH IMPLEMENTATION (Built with TDD) ===

/// Search for a query string in contents (case-sensitive)
/// 
/// This function was developed using TDD:
/// 1. First we wrote the test below
/// 2. Then we implemented this function to make the test pass
/// 3. Finally we refactored to improve the implementation
/// 
/// # Arguments
/// * `query` - The string to search for
/// * `contents` - The text to search in
/// 
/// # Returns
/// * `Vec<&str>` - Lines containing the query string
/// 
/// # Lifetimes
/// The returned string slices have the same lifetime as the input contents
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // TDD Step 2: Minimal implementation to make tests pass
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/// Search for a query string in contents (case-insensitive)
/// 
/// This function was also developed with TDD after the case-sensitive version
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// === TDD EVOLUTION EXAMPLES ===

/// Example showing how the search function might have evolved during TDD
mod tdd_evolution {
    // TDD Step 1: First failing implementation (just to make it compile)
    #[allow(dead_code)]
    fn search_v1<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {
        vec![] // Always returns empty - tests fail!
    }

    // TDD Step 2: Minimal implementation to pass first test
    #[allow(dead_code)]
    fn search_v2<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        if contents.contains(query) {
            contents.lines().collect() // Return all lines - too naive!
        } else {
            vec![]
        }
    }

    // TDD Step 3: Correct implementation after more tests
    #[allow(dead_code)]
    fn search_v3<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();
        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
        results
    }

    // TDD Step 4: Refactored using iterator methods (current implementation)
    #[allow(dead_code)]
    fn search_v4<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }
}

// === TESTS (TDD Step 1: RED) ===
// These tests were written BEFORE the implementation!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        // TDD Step 1: Write this test first (it will fail)
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
        // TDD: Another test written before implementation
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
    fn no_results() {
        // TDD: Test edge case - no matches
        let query = "xyz";
        let contents = "Hello world\nGoodbye world";
        
        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn multiple_results() {
        // TDD: Test multiple matches
        let query = "world";
        let contents = "Hello world\nGoodbye world\nAnother line\nworld of code";
        
        let expected = vec!["Hello world", "Goodbye world", "world of code"];
        assert_eq!(expected, search(query, contents));
    }

    #[test]
    fn empty_query() {
        // TDD: Test edge case - empty query
        let query = "";
        let contents = "Hello\nworld";
        
        // Empty string is contained in every line
        let expected = vec!["Hello", "world"];
        assert_eq!(expected, search(query, contents));
    }

    #[test]
    fn empty_contents() {
        // TDD: Test edge case - empty contents
        let query = "test";
        let contents = "";
        
        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    // === TESTS DEMONSTRATING TDD CYCLE ===

    /// This test demonstrates the TDD Red-Green-Refactor cycle
    #[test]
    fn tdd_cycle_example() {
        // Red: Write a failing test for new functionality
        let contents = "line 1\nline 2\nline 3";
        
        // Green: Implement just enough to pass
        let results = search("line", contents);
        
        // Refactor: Verify the implementation works correctly
        assert_eq!(3, results.len());
        assert!(results.contains(&"line 1"));
        assert!(results.contains(&"line 2"));
        assert!(results.contains(&"line 3"));
    }

    /// Test that demonstrates iterative development
    #[test]
    fn iterative_development() {
        let contents = "The quick brown fox\njumps over the lazy dog\nFox hunting is controversial";
        
        // First iteration: basic search
        let results = search("fox", contents);
        assert_eq!(vec!["The quick brown fox"], results);
        
        // Second iteration: case-insensitive search
        let results = search_case_insensitive("fox", contents);
        assert_eq!(2, results.len());
        assert!(results.contains(&"The quick brown fox"));
        assert!(results.contains(&"Fox hunting is controversial"));
    }

    /// Test showing how TDD helps catch edge cases
    #[test]
    fn edge_cases_caught_by_tdd() {
        // TDD forces us to think about edge cases upfront
        
        // Case 1: Query at start of line
        assert_eq!(vec!["start of line"], search("start", "start of line\nmiddle\nend"));
        
        // Case 2: Query at end of line
        assert_eq!(vec!["line end"], search("end", "start\nmiddle\nline end"));
        
        // Case 3: Query is entire line
        assert_eq!(vec!["exact"], search("exact", "start\nexact\nend"));
        
        // Case 4: Multiple occurrences in same line
        assert_eq!(vec!["test test test"], search("test", "hello\ntest test test\nworld"));
    }
}