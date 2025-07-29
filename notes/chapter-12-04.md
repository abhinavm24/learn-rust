# Chapter 12.4: Developing the Library's Functionality with Test-Driven Development

## Key Takeaways

### Test-Driven Development (TDD)
- **Red-Green-Refactor**: Write failing test, make it pass, refactor
- **Test First**: Write tests before implementation
- **Incremental Development**: Build functionality step by step
- **Living Documentation**: Tests serve as usage examples

### Search Function Development
```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

### TDD Cycle for Search Function
1. **Write Test**: Define expected behavior with test
2. **Run Test**: Confirm test fails (red)
3. **Write Code**: Minimal code to make test pass (green)
4. **Refactor**: Improve code while keeping tests passing

### Lifetime Annotations
- **Input Lifetimes**: Contents parameter lifetime
- **Output Lifetimes**: Returned string slices reference contents
- **Relationship**: Output lifetime tied to input lifetime
- **Compiler Verification**: Ensures returned references remain valid

### Integration with Main Function
```rust
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    
    Ok(())
}
```

### Test Design Principles
- **Single Responsibility**: Each test verifies one behavior
- **Clear Names**: Test names describe what they verify
- **Minimal Setup**: Use simple, focused test data
- **Expected Behavior**: Test what the function should do

### Iterative Development Benefits
- **Confidence**: Tests verify correctness at each step
- **Regression Prevention**: Tests catch breaking changes
- **Design Guidance**: Tests help design clean APIs
- **Documentation**: Tests show how to use functions

### Library Structure
- **Public Functions**: Exposed API for external use
- **Private Functions**: Internal implementation details
- **Test Module**: Unit tests alongside implementation
- **Clear Interfaces**: Well-defined function signatures

Official Chapter: https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html

---
*Completed: ✓*