//! # Chapter 13.2: Processing a Series of Items with Iterators
//! 
//! This example demonstrates:
//! - Iterator trait and lazy evaluation
//! - Different types of iterators (iter, into_iter, iter_mut)
//! - Iterator adaptors vs consuming adaptors
//! - Chaining iterator methods
//! - Zero-cost abstractions in practice
//! 
//! Run this example with: `cargo run --example ch13_02_iterators`

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 13.2", "Processing a Series of Items with Iterators");

    println!("Rust iterators are lazy and provide zero-cost abstractions!");
    println!();

    demonstrate_iterator_basics();
    demonstrate_iterator_types();
    demonstrate_iterator_adaptors();
    demonstrate_consuming_adaptors();
    demonstrate_iterator_chaining();
    demonstrate_custom_iterator();
}

/// Demonstrates basic iterator concepts and lazy evaluation
fn demonstrate_iterator_basics() {
    println!("=== Iterator Basics and Lazy Evaluation ===");
    
    let v1 = vec![1, 2, 3];
    
    // Creating an iterator doesn't do any work yet (lazy evaluation)
    println!("Creating iterator from vec: {:?}", v1);
    let v1_iter = v1.iter();
    println!("Iterator created (no work done yet due to lazy evaluation)");
    
    // Only when we consume the iterator does work happen
    println!("Consuming iterator with for loop:");
    for val in v1_iter {
        println!("  Got: {}", val);
    }
    
    // Manual iteration using next()
    println!("\nManual iteration with next():");
    let mut v2_iter = v1.iter();
    println!("  First call to next(): {:?}", v2_iter.next());
    println!("  Second call to next(): {:?}", v2_iter.next());
    println!("  Third call to next(): {:?}", v2_iter.next());
    println!("  Fourth call to next(): {:?}", v2_iter.next()); // None
    
    println!();
}

/// Demonstrates different types of iterators
fn demonstrate_iterator_types() {
    println!("=== Different Iterator Types ===");
    
    let mut v = vec![1, 2, 3];
    
    // iter() - creates iterator over references
    println!("1. iter() - iterator over references:");
    for item in v.iter() {
        println!("  &{} (type: &i32)", item);
    }
    
    // iter_mut() - creates iterator over mutable references
    println!("\n2. iter_mut() - iterator over mutable references:");
    for item in v.iter_mut() {
        *item *= 2; // Modify through mutable reference
        println!("  &mut {} (doubled)", item);
    }
    println!("  Vector after mutation: {:?}", v);
    
    // into_iter() - creates iterator that takes ownership
    println!("\n3. into_iter() - iterator that takes ownership:");
    for item in v.into_iter() {
        println!("  {} (owned value)", item);
    }
    // v is no longer available here due to move
    // println!("{:?}", v); // This would cause a compile error
    
    println!();
}

/// Demonstrates iterator adaptors (lazy - return other iterators)
fn demonstrate_iterator_adaptors() {
    println!("=== Iterator Adaptors (Lazy) ===");
    
    let v1: Vec<i32> = vec![1, 2, 3];
    
    // map() - transforms each element
    println!("Original vector: {:?}", v1);
    let doubled: Vec<i32> = v1.iter().map(|x| x * 2).collect();
    println!("After map(|x| x * 2): {:?}", doubled);
    
    // filter() - keeps elements that match predicate
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).cloned().collect();
    println!("Even numbers from {:?}: {:?}", numbers, evens);
    
    // enumerate() - adds index to each element
    let words = vec!["hello", "world", "rust"];
    let indexed: Vec<(usize, &str)> = words.iter().enumerate().map(|(i, &s)| (i, s)).collect();
    println!("Enumerated words: {:?}", indexed);
    
    // zip() - combines two iterators
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![30, 25, 35];
    let people: Vec<(&str, i32)> = names.iter().zip(ages.iter()).map(|(&name, &age)| (name, age)).collect();
    println!("Zipped names and ages: {:?}", people);
    
    // take() - takes first n elements
    let first_five: Vec<i32> = (1..100).take(5).collect();
    println!("First 5 numbers from 1..100: {:?}", first_five);
    
    // skip() - skips first n elements
    let skip_first_three: Vec<i32> = (1..8).skip(3).collect();
    println!("Skip first 3 from 1..8: {:?}", skip_first_three);
    
    println!();
}

/// Demonstrates consuming adaptors (eager - consume the iterator)
fn demonstrate_consuming_adaptors() {
    println!("=== Consuming Adaptors (Eager) ===");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // collect() - consumes iterator into collection
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("collect(): {:?}", doubled);
    
    // fold() - accumulates values with closure
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("fold(0, |acc, x| acc + x): {}", sum);
    
    // reduce() - similar to fold but uses first element as initial value
    let product = numbers.iter().cloned().reduce(|acc, x| acc * x);
    println!("reduce(|acc, x| acc * x): {:?}", product);
    
    // sum() - sums numeric values
    let sum: i32 = numbers.iter().sum();
    println!("sum(): {}", sum);
    
    // count() - counts elements
    let count = numbers.iter().count();
    println!("count(): {}", count);
    
    // any() - checks if any element matches predicate
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    println!("any(|&x| x % 2 == 0): {}", has_even);
    
    // all() - checks if all elements match predicate
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("all(|&x| x > 0): {}", all_positive);
    
    // find() - finds first element matching predicate
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    println!("find(|&&x| x % 2 == 0): {:?}", first_even);
    
    // max() and min()
    let max = numbers.iter().max();
    let min = numbers.iter().min();
    println!("max(): {:?}, min(): {:?}", max, min);
    
    println!();
}

/// Demonstrates complex iterator chaining
fn demonstrate_iterator_chaining() {
    println!("=== Iterator Chaining ===");
    
    let words = vec!["hello", "world", "rust", "is", "awesome", "programming", "language"];
    
    // Complex chain: filter -> map -> enumerate -> collect
    let long_words_with_index: Vec<(usize, String)> = words
        .iter()
        .filter(|word| word.len() > 4)          // Keep words longer than 4 chars
        .map(|word| word.to_uppercase())        // Convert to uppercase
        .enumerate()                            // Add index
        .collect();
    
    println!("Long words (>4 chars) in uppercase with index:");
    for (i, word) in &long_words_with_index {
        println!("  {}: {}", i, word);
    }
    
    // Another complex example: numbers processing
    let numbers: Vec<i32> = (1..20).collect();
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)              // Even numbers only
        .map(|&x| x * x)                       // Square them
        .filter(|&x| x > 10)                   // Keep squares > 10
        .take(3)                               // Take first 3
        .collect();
    
    println!("\nProcessing 1..20: even -> square -> >10 -> take(3):");
    println!("Result: {:?}", result);
    
    // Functional programming style vs imperative
    println!("\n📊 Functional vs Imperative Style:");
    
    // Imperative style
    let mut imperative_result = Vec::new();
    for &num in &numbers {
        if num % 2 == 0 {
            let squared = num * num;
            if squared > 10 {
                imperative_result.push(squared);
                if imperative_result.len() >= 3 {
                    break;
                }
            }
        }
    }
    println!("Imperative result: {:?}", imperative_result);
    
    // Functional style (same logic as above)
    let functional_result: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .filter(|&x| x > 10)
        .take(3)
        .collect();
    println!("Functional result:  {:?}", functional_result);
    
    println!();
}

/// Custom iterator implementation
#[derive(Debug)]
struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

/// Demonstrates creating custom iterators
fn demonstrate_custom_iterator() {
    println!("=== Custom Iterator Implementation ===");
    
    let counter = Counter::new(5);
    println!("Custom Counter iterator (0..5):");
    
    for num in counter {
        println!("  Count: {}", num);
    }
    
    // Using iterator methods on custom iterator
    let counter2 = Counter::new(10);
    let sum: usize = counter2.sum();
    println!("Sum of Counter(10): {}", sum);
    
    // Chaining with custom iterator
    let counter3 = Counter::new(8);
    let even_squares: Vec<usize> = counter3
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("Even squares from Counter(8): {:?}", even_squares);
    
    println!();
    println!("🚀 Iterator Benefits:");
    println!("• Zero-cost abstractions - compile to efficient machine code");
    println!("• Lazy evaluation - work only done when needed");
    println!("• Composable - chain operations naturally");
    println!("• Functional programming - expressive and concise");
    println!("• Memory efficient - process items one at a time");
}

// === PERFORMANCE COMPARISON ===

/// Demonstrates that iterators have zero-cost abstractions
#[allow(dead_code)]
fn performance_comparison() {
    use std::time::Instant;
    
    let data: Vec<i32> = (0..1_000_000).collect();
    
    // Manual loop approach
    let start = Instant::now();
    let mut sum = 0;
    for &item in &data {
        if item % 2 == 0 {
            sum += item * item;
        }
    }
    let loop_duration = start.elapsed();
    
    // Iterator approach
    let start = Instant::now();
    let iter_sum: i32 = data
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    let iter_duration = start.elapsed();
    
    println!("Performance comparison:");
    println!("Manual loop: {} (time: {:?})", sum, loop_duration);
    println!("Iterator:    {} (time: {:?})", iter_sum, iter_duration);
    println!("Results are equal: {}", sum == iter_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_demonstration() {
        let v = vec![1, 2, 3];
        let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);
    }

    #[test]
    fn test_custom_counter() {
        let counter = Counter::new(3);
        let values: Vec<usize> = counter.collect();
        assert_eq!(values, vec![0, 1, 2]);
    }

    #[test]
    fn test_iterator_chaining() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let result: Vec<i32> = numbers
            .iter()
            .filter(|&&x| x % 2 == 0)
            .map(|x| x * x)
            .collect();
        assert_eq!(result, vec![4, 16, 36]);
    }

    #[test]
    fn test_consuming_adaptors() {
        let numbers = vec![1, 2, 3, 4, 5];
        
        assert_eq!(numbers.iter().sum::<i32>(), 15);
        assert_eq!(numbers.iter().count(), 5);
        assert_eq!(numbers.iter().max(), Some(&5));
        assert_eq!(numbers.iter().min(), Some(&1));
        assert!(numbers.iter().any(|&x| x > 3));
        assert!(numbers.iter().all(|&x| x > 0));
    }

    #[test]
    fn test_iterator_types() {
        let mut v = vec![1, 2, 3];
        
        // Test iter() - references
        let sum: i32 = v.iter().sum();
        assert_eq!(sum, 6);
        assert_eq!(v, vec![1, 2, 3]); // v still available
        
        // Test iter_mut() - mutable references
        for item in v.iter_mut() {
            *item *= 2;
        }
        assert_eq!(v, vec![2, 4, 6]);
        
        // Test into_iter() - owned values
        let sum: i32 = v.into_iter().sum();
        assert_eq!(sum, 12);
        // v is no longer available here
    }
}