# Chapter 13: Functional Language Features - Iterators and Closures

## Key Takeaways

### Functional Programming in Rust
- **Closures**: Anonymous functions that capture their environment
- **Iterators**: Lazy functional programming constructs for processing data
- **Zero-Cost Abstractions**: High-level features with no runtime overhead
- **Functional Style**: Enables expressive, concise code

### Closures
- **Anonymous Functions**: Functions without names that can capture variables
- **Environment Capture**: Can capture variables from enclosing scope
- **Type Inference**: Compiler infers parameter and return types
- **Flexible Syntax**: Can be stored in variables and passed as parameters

### Closure Syntax
```rust
let closure = |param| param + 1;
let closure = |x: i32| -> i32 { x + 1 };
let closure = || println!("Hello");
```

### Iterators
- **Lazy Evaluation**: Only do work when consumed
- **Chainable**: Can be chained together for complex operations
- **Performance**: Often faster than equivalent loops
- **Functional Methods**: map, filter, fold, collect, etc.

### Iterator Methods
```rust
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
let sum: i32 = v1.iter().sum();
let filtered: Vec<&i32> = v1.iter().filter(|&x| *x > 2).collect();
```

### Key Concepts
- **Iterator Adaptors**: Transform iterators (map, filter, enumerate)
- **Consuming Adaptors**: Consume iterators (collect, fold, reduce)
- **Capturing Environment**: Closures can move, borrow, or mutably borrow
- **Performance**: Iterator chains compile to efficient loops

### Integration with Previous Chapters
- Enhances collections usage from Chapter 8
- Provides elegant error handling patterns
- Works seamlessly with generic types and traits
- Enables more expressive testing patterns

### Practical Applications
- Data processing pipelines
- Functional-style programming
- Performance-critical code with readability
- API design with flexible callback parameters

Official Chapter: https://doc.rust-lang.org/book/ch13-00-functional-features.html

---
*Completed: ✓*