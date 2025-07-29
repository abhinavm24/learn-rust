# Chapter 14.2: Publishing a Crate to Crates.io

## Key Takeaways

### Documentation Comments
- **/// Syntax**: Document public API items
- **//! Syntax**: Document containing module or crate
- **Markdown Support**: Full markdown formatting in comments
- **Doc Tests**: Code examples in comments are tested

### Writing Documentation
```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
```

### Common Documentation Sections
```rust
/// Does important work.
///
/// # Arguments
///
/// * `input` - A string slice that holds the input data
/// * `verbose` - A boolean indicating whether to print debug info
///
/// # Returns
///
/// Returns a `Result<String, Error>` containing the processed data
/// or an error if processing fails.
///
/// # Errors
///
/// This function will return an error if the input contains invalid characters.
///
/// # Panics
///
/// This function panics if the input is empty.
///
/// # Safety
///
/// This function is safe to call from multiple threads.
///
/// # Examples
///
/// ```
/// use my_crate::process_data;
///
/// let result = process_data("hello", true);
/// assert!(result.is_ok());
/// ```
pub fn process_data(input: &str, verbose: bool) -> Result<String, Error> {
    // implementation
}
```

### Cargo.toml Metadata
```toml
[package]
name = "my_crate"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
license = "MIT OR Apache-2.0"
description = "A short description of the crate"
homepage = "https://example.com/my_crate"
documentation = "https://docs.rs/my_crate"
repository = "https://github.com/username/my_crate"
readme = "README.md"
keywords = ["cli", "tools", "utility"]
categories = ["command-line-utilities"]
exclude = [
    "tests/fixtures/*",
    "benches/data/*",
]
```

### Publishing Workflow
```bash
# 1. Create account on crates.io
# 2. Get API token from account settings
cargo login <your-api-token>

# 3. Package and inspect
cargo package
cargo package --list

# 4. Publish to crates.io
cargo publish

# 5. Publish dry run (test without uploading)
cargo publish --dry-run
```

### Version Management
```toml
[package]
version = "0.1.0"  # Major.Minor.Patch

# Semantic versioning guidelines:
# 0.1.0 -> 0.1.1 (patch: bug fixes)
# 0.1.1 -> 0.2.0 (minor: new features, backward compatible)
# 0.2.0 -> 1.0.0 (major: breaking changes)
```

### Re-exports for API Design
```rust
// src/lib.rs
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
    
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;
    
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // implementation
    }
}
```

### Documentation Generation
```bash
cargo doc                    # Generate documentation
cargo doc --open            # Generate and open in browser
cargo doc --no-deps         # Don't document dependencies
cargo doc --document-private-items  # Include private items
```

### Publishing Best Practices
- **Semantic Versioning**: Follow semver strictly
- **Good Documentation**: Include examples and clear descriptions
- **Comprehensive Testing**: Ensure code quality
- **README File**: Clear project description and usage
- **License**: Choose appropriate open source license
- **Keywords**: Help users find your crate

### Crate Categories
```toml
categories = [
    "algorithms",
    "api-bindings",
    "authentication",
    "caching",
    "command-line-interface",
    "command-line-utilities",
    "compression",
    "concurrency",
    "config",
    "cryptography",
    "data-structures",
    "database",
    "date-and-time",
    "development-tools",
    "email",
    "encoding",
    "filesystem",
    "game-development",
    "graphics",
    "gui",
    "hardware-support",
    "internationalization",
    "mathematics",
    "memory-management",
    "multimedia",
    "network-programming",
    "no-std",
    "os",
    "parser-implementations",
    "parsing",
    "rendering",
    "rust-patterns",
    "science",
    "simulation",
    "template-engine",
    "text-editors",
    "text-processing",
    "value-formatting",
    "visualization",
    "wasm",
    "web-programming"
]
```

### Yanking Versions
```bash
# Remove a version (doesn't delete, just prevents new usage)
cargo yank --vers 1.0.1

# Un-yank a version
cargo yank --vers 1.0.1 --undo
```

Official Chapter: https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html

---
*Completed: ✓*