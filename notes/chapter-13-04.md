# Chapter 13.4: Comparing Performance - Loops vs. Iterators

## Key Takeaways

### Performance Comparison
- **Zero-Cost Abstractions**: Iterators compile to same performance as loops
- **Compiler Optimization**: Rust compiler optimizes iterator chains effectively
- **Benchmarking**: Measure performance to verify assumptions
- **Assembly Output**: Iterator code produces similar assembly to manual loops

### Benchmark Setup
```rust
use std::time::Instant;

fn benchmark_loops_vs_iterators() {
    let data: Vec<i32> = (0..1_000_000).collect();
    
    // Benchmark manual loop
    let start = Instant::now();
    let mut sum = 0;
    for &item in &data {
        sum += item;
    }
    let loop_duration = start.elapsed();
    println!("Loop sum: {}, time: {:?}", sum, loop_duration);
    
    // Benchmark iterator
    let start = Instant::now();
    let sum: i32 = data.iter().sum();
    let iter_duration = start.elapsed();
    println!("Iterator sum: {}, time: {:?}", sum, iter_duration);
}
```

### Complex Processing Comparison
```rust
// Manual loop approach
fn process_data_loop(data: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    
    for &item in data {
        if item % 2 == 0 {
            let squared = item * item;
            if squared > 100 {
                result.push(squared);
            }
        }
    }
    
    result
}

// Iterator approach
fn process_data_iterator(data: &[i32]) -> Vec<i32> {
    data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .filter(|&x| x > 100)
        .collect()
}

// Benchmark both approaches
fn benchmark_processing() {
    let data: Vec<i32> = (1..1000).collect();
    
    let start = Instant::now();
    let result1 = process_data_loop(&data);
    let loop_time = start.elapsed();
    
    let start = Instant::now();
    let result2 = process_data_iterator(&data);
    let iter_time = start.elapsed();
    
    assert_eq!(result1, result2);
    println!("Loop time: {:?}, Iterator time: {:?}", loop_time, iter_time);
}
```

### Assembly Output Analysis
```rust
// This iterator chain...
let sum: u32 = (0..1_000_000)
    .map(|x| x * x)
    .filter(|&x| x % 2 == 0)
    .sum();

// Compiles to assembly similar to this loop:
let mut sum = 0u32;
for i in 0..1_000_000 {
    let square = i * i;
    if square % 2 == 0 {
        sum += square;
    }
}
```

### Micro-benchmarking with Criterion
```toml
# Cargo.toml
[dev-dependencies]
criterion = "0.4"

[[bench]]
name = "iterator_benchmark"
harness = false
```

```rust
// benches/iterator_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sum_loop(data: &[i32]) -> i32 {
    let mut sum = 0;
    for &item in data {
        sum += item;
    }
    sum
}

fn sum_iterator(data: &[i32]) -> i32 {
    data.iter().sum()
}

fn benchmark_sum(c: &mut Criterion) {
    let data: Vec<i32> = (0..1000).collect();
    
    c.bench_function("sum_loop", |b| {
        b.iter(|| sum_loop(black_box(&data)))
    });
    
    c.bench_function("sum_iterator", |b| {
        b.iter(|| sum_iterator(black_box(&data)))
    });
}

criterion_group!(benches, benchmark_sum);
criterion_main!(benches);
```

### Real-World Performance Factors
```rust
// Iterator advantages
fn find_first_match(data: &[String], pattern: &str) -> Option<&String> {
    // Short-circuits on first match
    data.iter().find(|s| s.contains(pattern))
}

// Lazy evaluation benefit
fn expensive_processing(data: &[i32]) -> Option<i32> {
    data.iter()
        .map(|&x| expensive_computation(x))  // Only called for needed items
        .find(|&x| x > 1000)                 // Stops at first match
}

fn expensive_computation(x: i32) -> i32 {
    // Simulate expensive operation
    std::thread::sleep(std::time::Duration::from_millis(1));
    x * x
}
```

### Memory Efficiency
```rust
// Iterator: No intermediate collections
fn process_large_dataset(data: &[i32]) -> Vec<i32> {
    data.iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)
        .filter(|&x| x < 1000)
        .collect()  // Only one allocation
}

// Manual approach might create intermediate vectors
fn process_manual(data: &[i32]) -> Vec<i32> {
    let mut positive: Vec<i32> = data.iter()
        .filter(|&&x| x > 0)
        .cloned()
        .collect();  // First allocation
    
    let mut doubled: Vec<i32> = positive.iter()
        .map(|&x| x * 2)
        .collect();  // Second allocation
    
    doubled.retain(|&x| x < 1000);  // Potential reallocation
    doubled
}
```

### Performance Guidelines
- **Trust the Compiler**: Iterators are optimized heavily
- **Measure, Don't Guess**: Use benchmarks to verify performance
- **Readability First**: Prefer readable code, optimize when needed
- **Consider Context**: Different workloads may favor different approaches

### When Loops Might Be Better
```rust
// Complex state management
fn stateful_processing(data: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut accumulator = 0;
    let mut skip_next = false;
    
    for (i, &item) in data.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }
        
        accumulator += item;
        
        if item > 100 && i < data.len() - 1 {
            // Look ahead
            accumulator += data[i + 1];
            skip_next = true;
        }
        
        result.push(accumulator);
    }
    
    result
}
```

### Best Practices
- **Start with Iterators**: More readable and maintainable
- **Profile When Needed**: Identify actual bottlenecks
- **Combine Approaches**: Use both iterators and loops where appropriate
- **Consider Readability**: Team understanding is important
- **Trust Zero-Cost**: Rust's abstractions are truly zero-cost

Official Chapter: https://doc.rust-lang.org/book/ch13-04-performance.html

---
*Completed: ✓*