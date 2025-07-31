//! # Chapter 13.4: Comparing Performance - Loops vs. Iterators
//! 
//! This example demonstrates:
//! - Performance benchmarking of loops vs iterators
//! - Zero-cost abstractions in practice
//! - Compiler optimizations for iterator chains
//! - When to use iterators vs manual loops
//! 
//! Run this example with: `cargo run --example ch13_04_performance_comparison`
//! For release mode: `cargo run --release --example ch13_04_performance_comparison`

use rust_book_examples::print_chapter_header;
use std::time::Instant;

fn main() {
    print_chapter_header("Chapter 13.4", "Comparing Performance - Loops vs. Iterators");

    println!("Benchmarking loops vs iterators to demonstrate zero-cost abstractions");
    println!();

    run_basic_benchmarks();
    run_complex_benchmarks();
    demonstrate_compiler_optimizations();
    discuss_performance_considerations();
}

/// Basic performance comparison between loops and iterators
fn run_basic_benchmarks() {
    println!("=== Basic Performance Benchmarks ===");
    println!();

    let sizes = vec![10_000, 100_000, 1_000_000];
    
    for size in sizes {
        println!("📊 Benchmarking with {} elements:", size);
        let data: Vec<i32> = (0..size).collect();
        
        // Benchmark 1: Simple sum
        benchmark_sum(&data);
        
        // Benchmark 2: Filter and sum even numbers
        benchmark_filter_sum(&data);
        
        // Benchmark 3: Map and sum squares
        benchmark_map_sum(&data);
        
        println!();
    }
}

/// Benchmark simple summation
fn benchmark_sum(data: &[i32]) {
    // Manual loop approach
    let start = Instant::now();
    let mut sum = 0;
    for &item in data {
        sum += item;
    }
    let loop_duration = start.elapsed();
    
    // Iterator approach
    let start = Instant::now();
    let iter_sum: i32 = data.iter().sum();
    let iter_duration = start.elapsed();
    
    println!("  Sum:");
    println!("    Manual loop: {} ({:?})", sum, loop_duration);
    println!("    Iterator:    {} ({:?})", iter_sum, iter_duration);
    println!("    Results equal: {}", sum == iter_sum);
    
    let ratio = loop_duration.as_nanos() as f64 / iter_duration.as_nanos() as f64;
    println!("    Performance ratio (loop/iter): {:.2}", ratio);
}

/// Benchmark filtering and summing even numbers
fn benchmark_filter_sum(data: &[i32]) {
    // Manual loop approach
    let start = Instant::now();
    let mut sum = 0;
    for &item in data {
        if item % 2 == 0 {
            sum += item;
        }
    }
    let loop_duration = start.elapsed();
    
    // Iterator approach
    let start = Instant::now();
    let iter_sum: i32 = data.iter().filter(|&&x| x % 2 == 0).sum();
    let iter_duration = start.elapsed();
    
    println!("  Filter + Sum (even numbers):");
    println!("    Manual loop: {} ({:?})", sum, loop_duration);
    println!("    Iterator:    {} ({:?})", iter_sum, iter_duration);
    println!("    Results equal: {}", sum == iter_sum);
    
    let ratio = loop_duration.as_nanos() as f64 / iter_duration.as_nanos() as f64;
    println!("    Performance ratio (loop/iter): {:.2}", ratio);
}

/// Benchmark mapping to squares and summing
fn benchmark_map_sum(data: &[i32]) {
    // Manual loop approach
    let start = Instant::now();
    let mut sum = 0;
    for &item in data {
        sum += item * item;
    }
    let loop_duration = start.elapsed();
    
    // Iterator approach
    let start = Instant::now();
    let iter_sum: i32 = data.iter().map(|x| x * x).sum();
    let iter_duration = start.elapsed();
    
    println!("  Map + Sum (squares):");
    println!("    Manual loop: {} ({:?})", sum, loop_duration);
    println!("    Iterator:    {} ({:?})", iter_sum, iter_duration);
    println!("    Results equal: {}", sum == iter_sum);
    
    let ratio = loop_duration.as_nanos() as f64 / iter_duration.as_nanos() as f64;
    println!("    Performance ratio (loop/iter): {:.2}", ratio);
}

/// More complex performance benchmarks
fn run_complex_benchmarks() {
    println!("=== Complex Operation Benchmarks ===");
    println!();

    let data: Vec<i32> = (0..500_000).collect();
    
    // Complex operation: filter even, square, keep if > 1000, sum
    benchmark_complex_chain(&data);
    
    // String processing benchmark
    let words: Vec<String> = (0..100_000)
        .map(|i| format!("word_{}", i))
        .collect();
    benchmark_string_processing(&words);
}

/// Benchmark complex iterator chain
fn benchmark_complex_chain(data: &[i32]) {
    println!("📊 Complex Chain: filter even -> square -> filter >1000 -> sum");
    
    // Manual loop approach
    let start = Instant::now();
    let mut sum = 0;
    for &item in data {
        if item % 2 == 0 {
            let squared = item * item;
            if squared > 1000 {
                sum += squared;
            }
        }
    }
    let loop_duration = start.elapsed();
    
    // Iterator approach
    let start = Instant::now();
    let iter_sum: i32 = data
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .filter(|&x| x > 1000)
        .sum();
    let iter_duration = start.elapsed();
    
    println!("  Manual loop: {} ({:?})", sum, loop_duration);
    println!("  Iterator:    {} ({:?})", iter_sum, iter_duration);
    println!("  Results equal: {}", sum == iter_sum);
    
    let ratio = loop_duration.as_nanos() as f64 / iter_duration.as_nanos() as f64;
    println!("  Performance ratio (loop/iter): {:.2}", ratio);
    println!();
}

/// Benchmark string processing operations
fn benchmark_string_processing(words: &[String]) {
    println!("📊 String Processing: filter long words -> to uppercase -> collect");
    
    // Manual loop approach
    let start = Instant::now();
    let mut result = Vec::new();
    for word in words {
        if word.len() > 7 {
            result.push(word.to_uppercase());
        }
    }
    let loop_duration = start.elapsed();
    let loop_count = result.len();
    
    // Iterator approach
    let start = Instant::now();
    let iter_result: Vec<String> = words
        .iter()
        .filter(|word| word.len() > 7)
        .map(|word| word.to_uppercase())
        .collect();
    let iter_duration = start.elapsed();
    let iter_count = iter_result.len();
    
    println!("  Manual loop: {} items ({:?})", loop_count, loop_duration);
    println!("  Iterator:    {} items ({:?})", iter_count, iter_duration);
    println!("  Results equal: {}", loop_count == iter_count);
    
    let ratio = loop_duration.as_nanos() as f64 / iter_duration.as_nanos() as f64;
    println!("  Performance ratio (loop/iter): {:.2}", ratio);
    println!();
}

/// Demonstrates how compiler optimizations work
fn demonstrate_compiler_optimizations() {
    println!("=== Compiler Optimization Examples ===");
    println!();

    println!("🔧 The Rust compiler optimizes both approaches similarly:");
    println!();
    
    // Show equivalent assembly-level optimizations (conceptually)
    println!("Iterator code:");
    println!("  let sum: i32 = data.iter().map(|x| x * 2).sum();");
    println!();
    println!("Compiles to similar assembly as:");
    println!("  let mut sum = 0;");
    println!("  for item in data {{");
    println!("      sum += item * 2;");
    println!("  }}");
    println!();
    
    // Demonstrate with a simple example
    let data = vec![1, 2, 3, 4, 5];
    
    let start = Instant::now();
    let _: i32 = data.iter().map(|x| x * 2).sum();
    let iter_time = start.elapsed();
    
    let start = Instant::now();
    let mut sum = 0;
    for &item in &data {
        sum += item * 2;
    }
    let loop_time = start.elapsed();
    
    println!("Small dataset (5 elements) - times may be noise:");
    println!("  Iterator: {:?}", iter_time);
    println!("  Loop:     {:?}", loop_time);
    println!();
    
    println!("💡 Key Points:");
    println!("• Both compile to similar optimized machine code");
    println!("• No runtime overhead from iterator abstractions");
    println!("• Compiler can often vectorize both approaches");
    println!("• LLVM optimizations apply equally to both styles");
    println!();
}

/// Discusses when to use iterators vs loops
fn discuss_performance_considerations() {
    println!("=== Performance Considerations ===");
    println!();
    
    println!("📈 When Iterators Excel:");
    println!("• Complex data transformations with multiple steps");
    println!("• Functional programming patterns");
    println!("• Code readability and maintainability are priorities");
    println!("• Working with large datasets where lazy evaluation helps");
    println!("• Parallel processing (with rayon crate)");
    println!();
    
    println!("🔄 When Manual Loops Might Be Better:");
    println!("• Very simple operations where clarity is paramount");
    println!("• Complex control flow with early breaks/continues");
    println!("• Memory-constrained environments (rare)");
    println!("• When you need precise control over memory allocation");
    println!();
    
    println!("🚀 Best Practices:");
    println!("• Default to iterators for their expressiveness");
    println!("• Profile your specific use case if performance is critical");
    println!("• Use --release builds for accurate performance testing");
    println!("• Consider using criterion crate for proper benchmarking");
    println!("• Remember: premature optimization is the root of all evil");
    println!();
    
    // Demonstrate parallel processing potential
    demonstrate_parallel_potential();
}

/// Shows how iterators enable easy parallel processing
fn demonstrate_parallel_potential() {
    println!("🔀 Parallel Processing Potential:");
    println!();
    
    let data: Vec<i32> = (0..1000).collect();
    
    // Sequential iterator (what we've been doing)
    let start = Instant::now();
    let sequential_sum: i32 = data
        .iter()
        .map(|x| expensive_operation(*x))
        .sum();
    let sequential_time = start.elapsed();
    
    println!("Sequential processing:");
    println!("  Result: {} (time: {:?})", sequential_sum, sequential_time);
    
    // Note: This would require rayon crate for actual parallel execution
    println!();
    println!("With rayon crate, you could easily make it parallel:");
    println!("  use rayon::prelude::*;");
    println!("  let parallel_sum: i32 = data");
    println!("      .par_iter()  // <-- Just add 'par_'!");
    println!("      .map(|x| expensive_operation(*x))");
    println!("      .sum();");
    println!();
    println!("This is much harder to do with manual loops!");
}

/// Simulates an expensive operation for demonstration
fn expensive_operation(x: i32) -> i32 {
    // Simulate some computational work
    let mut result = x;
    for _ in 0..10 {
        result = (result * 13) % 1000;
    }
    result
}

/// Advanced benchmarking utilities
#[allow(dead_code)]
mod benchmarking_utils {
    use std::time::{Duration, Instant};

    pub fn time_it<F, R>(f: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }

    pub fn benchmark_multiple<F>(name: &str, f: F, iterations: usize)
    where
        F: Fn() -> i32,
    {
        println!("Benchmarking '{}' over {} iterations:", name, iterations);
        
        let mut total_time = Duration::from_nanos(0);
        let mut results = Vec::with_capacity(iterations);
        
        for _ in 0..iterations {
            let (result, duration) = time_it(&f);
            results.push(result);
            total_time += duration;
        }
        
        let avg_time = total_time / iterations as u32;
        println!("  Average time: {:?}", avg_time);
        println!("  Total time: {:?}", total_time);
        
        // Verify all results are the same
        if results.windows(2).all(|w| w[0] == w[1]) {
            println!("  ✅ All results consistent: {}", results[0]);
        } else {
            println!("  ❌ Results inconsistent!");
        }
    }
}

/// Memory usage comparison
#[allow(dead_code)]
fn memory_usage_comparison() {
    println!("=== Memory Usage Comparison ===");
    
    // Iterator chains are often more memory efficient due to lazy evaluation
    let large_range = 0..10_000_000;
    
    // This doesn't allocate intermediate collections
    let _iter_result: i32 = large_range
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .take(1000)
        .sum();
    
    // Manual approach might require intermediate allocations
    let data: Vec<i32> = (0..10_000_000).collect(); // Large allocation!
    let mut filtered = Vec::new();
    for &item in &data {
        if item % 2 == 0 {
            filtered.push(item);
        }
    }
    // ... more allocations for each step
    
    println!("Iterators can be more memory efficient due to lazy evaluation");
    println!("They process items one at a time without intermediate collections");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_equivalence() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        
        // Manual sum
        let mut manual_sum = 0;
        for &item in &data {
            manual_sum += item;
        }
        
        // Iterator sum
        let iter_sum: i32 = data.iter().sum();
        
        assert_eq!(manual_sum, iter_sum);
    }

    #[test]
    fn test_complex_operation_equivalence() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        
        // Manual approach
        let mut manual_result = 0;
        for &item in &data {
            if item % 2 == 0 {
                let squared = item * item;
                if squared > 10 {
                    manual_result += squared;
                }
            }
        }
        
        // Iterator approach
        let iter_result: i32 = data
            .iter()
            .filter(|&&x| x % 2 == 0)
            .map(|x| x * x)
            .filter(|&&x| x > 10)
            .sum();
        
        assert_eq!(manual_result, iter_result);
    }

    #[test]
    fn test_string_processing_equivalence() {
        let words = vec!["hello".to_string(), "world".to_string(), "programming".to_string()];
        
        // Manual approach
        let mut manual_result = Vec::new();
        for word in &words {
            if word.len() > 5 {
                manual_result.push(word.to_uppercase());
            }
        }
        
        // Iterator approach
        let iter_result: Vec<String> = words
            .iter()
            .filter(|word| word.len() > 5)
            .map(|word| word.to_uppercase())
            .collect();
        
        assert_eq!(manual_result, iter_result);
    }

    #[test]
    fn test_expensive_operation() {
        // Test that our expensive operation is deterministic
        assert_eq!(expensive_operation(5), expensive_operation(5));
        assert_eq!(expensive_operation(10), expensive_operation(10));
    }
}