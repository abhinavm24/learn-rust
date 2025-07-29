# Chapter 11.1: How to Write Tests

## Key Takeaways

### Test Function Anatomy
- **#[test] Attribute**: Marks functions as tests
- **Test Body**: Contains code that exercises functionality
- **Assertions**: Verify expected behavior with assert! macros
- **Test Failure**: Tests fail when code panics

### Basic Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

### Assertion Macros
- **assert!(expression)**: Verify boolean condition
- **assert_eq!(left, right)**: Test equality
- **assert_ne!(left, right)**: Test inequality
- **Custom Messages**: Add failure messages as additional arguments

### Testing Panics
```rust
#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200);
}

#[test]
#[should_panic(expected = "between 1 and 100")]
fn greater_than_100_with_message() {
    Guess::new(200);
}
```

### Using Result<T, E> in Tests
```rust
#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

### Test Organization Patterns
- Tests in same file as implementation
- Tests in separate `tests` module
- Use `use super::*` to import parent items
- Group related tests in sub-modules

Official Chapter: https://doc.rust-lang.org/book/ch11-01-writing-tests.html

---
*Completed: ✓*