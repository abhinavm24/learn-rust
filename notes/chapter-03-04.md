# Chapter 3.4: Comments

## Key Takeaways

### Comment Syntax
- `//` starts single-line comments (everything after ignored by compiler)
- No multi-line comment syntax - use `//` on each line
- Can be placed above code or at end of lines

### Comment Placement
- **Above code**: `// Comment explaining next lines`
- **End-of-line**: `let x = 5; // Comment about this line`
- **Multi-line**: Multiple consecutive `//` lines

### Important Syntax and Operators
- `//` - Comment marker (not an operator, ignored by compiler)

### Programming Concepts Introduced
- **Code Documentation**: Explaining logic for human readers
- **Compiler Behavior**: Understanding what gets processed vs ignored

### Code Examples and Patterns

#### Basic Comments
```rust
fn main() {
    // This is a single-line comment
    println!("Hello, world!");
    
    let lucky_number = 7; // End-of-line comment
}
```

#### Multi-line Comments
```rust
fn main() {
    // Calculate compound interest using formula: A = P(1 + r)^t
    // Where: P = principal, r = rate, t = time
    let principal = 1000.0;
    let rate = 0.05;
    let time = 5.0;
    let amount = principal * (1.0 + rate).powf(time);
}
```

### Practical Applications
- Explaining complex algorithms and formulas
- Documenting assumptions and business logic
- Adding TODO notes for future improvements
- Clarifying non-obvious code decisions

### Community Conventions and Idioms
- Explain "why," not "what" - avoid obvious comments
- Keep comments updated when code changes
- Use TODO/FIXME/NOTE prefixes for special cases
- Prefer self-documenting code when possible

### Personal Notes
- Essential for team collaboration and code maintenance
- Balance between over-commenting and under-commenting
- Comments should add value, not repeat what code already shows

Official Chapter: https://doc.rust-lang.org/book/ch03-04-comments.html

---
*Completed: ✓*