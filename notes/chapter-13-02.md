# Chapter 13.2: Processing a Series of Items with Iterators

## Key Takeaways

### Iterator Fundamentals
- **Lazy Evaluation**: Iterators do no work until consumed
- **Iterator Trait**: Standard interface for iteration
- **Chaining**: Iterator methods can be chained together
- **Zero-Cost Abstractions**: Compile to equivalent loop performance

### Creating Iterators
```rust
let v1 = vec![1, 2, 3];

// iter() creates iterator over references
let v1_iter = v1.iter();

// into_iter() creates iterator that takes ownership
let v1_iter = v1.into_iter();

// iter_mut() creates iterator over mutable references
let mut v1 = vec![1, 2, 3];
let v1_iter = v1.iter_mut();
```

### Iterator Trait
```rust
trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
    
    // Many default implementations based on next()
}
```

### Consuming Adaptors
```rust
let v1 = vec![1, 2, 3];

// collect() consumes iterator into collection
let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

// sum() consumes iterator, returns sum
let total: i32 = v1.iter().sum();

// for_each() consumes iterator, runs closure on each
v1.iter().for_each(|x| println!("{}", x));

// find() returns first matching element
let found = v1.iter().find(|&&x| x > 2);
```

### Iterator Adaptors
```rust
let v1: Vec<i32> = vec![1, 2, 3];

// map() transforms each element
let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

// filter() keeps elements matching predicate
let v3: Vec<&i32> = v1.iter().filter(|&&x| x > 1).collect();

// enumerate() adds indices
let v4: Vec<(usize, &i32)> = v1.iter().enumerate().collect();

// zip() combines with another iterator
let v5 = vec!["a", "b", "c"];
let combined: Vec<(i32, &str)> = v1.into_iter().zip(v5.iter()).collect();
```

### Chaining Operations
```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

let result: Vec<i32> = numbers
    .iter()
    .filter(|&&x| x % 2 == 0)  // Keep even numbers
    .map(|x| x * x)            // Square them
    .collect();                // Collect into Vec

println!("{:?}", result); // [4, 16, 36]
```

### Custom Iterator Implementation
```rust
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

fn main() {
    let mut counter = Counter::new(3);
    
    for n in counter {
        println!("{}", n); // 0, 1, 2
    }
}
```

### Iterator vs Loop Performance
```rust
// Iterator version (zero-cost abstraction)
let sum: u32 = (0..1_000_000)
    .map(|x| x * x)
    .filter(|&x| x % 2 == 0)
    .sum();

// Equivalent for loop
let mut sum = 0;
for i in 0..1_000_000 {
    let square = i * i;
    if square % 2 == 0 {
        sum += square;
    }
}
// Both compile to similar assembly!
```

### Working with Results
```rust
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = vec!["file1.txt", "file2.txt", "file3.txt"];
    
    // collect() can handle Result<Vec<T>, E>
    let contents: Result<Vec<String>, _> = files
        .iter()
        .map(|&filename| fs::read_to_string(filename))
        .collect();
    
    match contents {
        Ok(all_contents) => {
            for content in all_contents {
                println!("File content: {}", content);
            }
        }
        Err(e) => println!("Error reading files: {}", e),
    }
    
    Ok(())
}
```

### Practical Examples
```rust
// Processing CSV-like data
let data = "1,2,3\n4,5,6\n7,8,9";
let parsed: Vec<Vec<i32>> = data
    .lines()
    .map(|line| {
        line.split(',')
            .map(|s| s.parse().unwrap())
            .collect()
    })
    .collect();

// Word frequency counting
use std::collections::HashMap;

let text = "hello world hello rust world";
let word_counts: HashMap<&str, usize> = text
    .split_whitespace()
    .fold(HashMap::new(), |mut acc, word| {
        *acc.entry(word).or_insert(0) += 1;
        acc
    });
```

Official Chapter: https://doc.rust-lang.org/book/ch13-02-iterators.html

---
*Completed: ✓*