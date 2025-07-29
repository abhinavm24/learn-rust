# Chapter 11.3: Test Organization

## Key Takeaways

### Unit Tests
- **Location**: Same file as code being tested
- **Module**: Inside `#[cfg(test)]` module
- **Access**: Can test private functions
- **Purpose**: Test individual units in isolation

### Unit Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn internal() {
        // Can test private functions
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

### Integration Tests
- **Location**: `tests/` directory at project root
- **Scope**: Test library from external perspective
- **Access**: Only public API available
- **Purpose**: Test multiple components together

### Integration Test File
```rust
// tests/integration_test.rs
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

### Common Module Pattern
```rust
// tests/common/mod.rs
pub fn setup() {
    // Common test setup code
}

// tests/integration_test.rs
mod common;

#[test]
fn it_works() {
    common::setup();
    // Test code
}
```

### Binary Crate Testing
- **Library Required**: Integration tests need `src/lib.rs`
- **Binary Logic**: Move logic to library, call from binary
- **Main Function**: Keep `main.rs` minimal
- **Testing Strategy**: Test library functions, not main

### Test Organization Best Practices
- **Unit Tests**: Test implementation details
- **Integration Tests**: Test public API
- **Documentation Tests**: Test examples in docs
- **Common Utilities**: Share test setup code

### Conditional Compilation
- **#[cfg(test)]**: Only compile during testing
- **Development Dependencies**: Test-only dependencies
- **Feature Flags**: Conditional test compilation
- **Platform-Specific**: Tests for specific platforms

Official Chapter: https://doc.rust-lang.org/book/ch11-03-test-organization.html

---
*Completed: ✓*