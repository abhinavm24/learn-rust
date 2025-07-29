# Chapter 11.2: Controlling How Tests Are Run

## Key Takeaways

### Test Execution Control
- **Parallel Execution**: Tests run in parallel by default
- **Thread Control**: Use `--test-threads` to control parallelism
- **Sequential Testing**: `--test-threads=1` for sequential execution
- **Output Capture**: Test output is captured unless test fails

### Command Line Options
```bash
cargo test                           # Run all tests
cargo test test_name                 # Run specific test
cargo test -- --test-threads=1      # Run tests sequentially
cargo test -- --nocapture           # Show println! output
cargo test -- --ignored             # Run only ignored tests
cargo test -- --include-ignored     # Run all tests including ignored
```

### Test Filtering
- **Name Filtering**: Run tests matching name pattern
- **Module Filtering**: Run tests in specific modules
- **Substring Matching**: Tests containing substring in name
- **Exact Matching**: Run single test by exact name

### Ignoring Tests
```rust
#[test]
#[ignore]
fn expensive_test() {
    // Code that takes a long time to run
}
```

### Test Categories
- **Unit Tests**: Fast, isolated tests
- **Integration Tests**: Slower, test component interaction
- **Ignored Tests**: Expensive or conditional tests
- **Documentation Tests**: Tests in doc comments

### Parallel vs Sequential Considerations
- **Shared State**: Use sequential for shared resources
- **File System**: Sequential for file operations
- **Performance**: Parallel for independent tests
- **Debugging**: Sequential for easier debugging

Official Chapter: https://doc.rust-lang.org/book/ch11-02-running-tests.html

---
*Completed: ✓*