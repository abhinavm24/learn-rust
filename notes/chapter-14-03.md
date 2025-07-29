# Chapter 14.3: Cargo Workspaces

## Key Takeaways

### Workspace Fundamentals
- **Multi-Package Projects**: Manage multiple related crates together
- **Shared Dependencies**: Common Cargo.lock and target directory
- **Coordinated Development**: Develop related crates simultaneously
- **Workspace Root**: Contains workspace configuration

### Workspace Structure
```
my_workspace/
├── Cargo.toml          # Workspace root
├── Cargo.lock          # Shared lock file
├── target/             # Shared build artifacts
├── add_one/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── adder/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

### Workspace Cargo.toml
```toml
# Root Cargo.toml
[workspace]
members = [
    "adder",
    "add_one",
]

# Optional: specify resolver version
resolver = "2"

# Workspace-wide dependencies
[workspace.dependencies]
serde = "1.0"
tokio = "1.0"

# Workspace metadata
[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT OR Apache-2.0"
authors = ["Your Name <email@example.com>"]
```

### Member Crate Configuration
```toml
# add_one/Cargo.toml
[package]
name = "add_one"
version.workspace = true    # Inherit from workspace
edition.workspace = true
license.workspace = true
authors.workspace = true

[dependencies]
# Can use workspace dependencies
serde.workspace = true
```

### Creating a Workspace
```bash
# 1. Create workspace root
mkdir my_workspace
cd my_workspace

# 2. Create workspace Cargo.toml
cat > Cargo.toml << EOF
[workspace]
members = [
    "adder",
    "add_one",
]
EOF

# 3. Create member crates
cargo new adder
cargo new add_one --lib
```

### Inter-Workspace Dependencies
```toml
# adder/Cargo.toml
[package]
name = "adder"
version = "0.1.0"
edition = "2021"

[dependencies]
add_one = { path = "../add_one" }  # Local workspace member
rand = "0.8"                       # External dependency
```

### Example Implementation
```rust
// add_one/src/lib.rs
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// adder/src/main.rs
use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
}
```

### Workspace Commands
```bash
# Build entire workspace
cargo build

# Build specific package
cargo build -p add_one
cargo build --package adder

# Run specific binary
cargo run -p adder

# Test entire workspace
cargo test

# Test specific package
cargo test -p add_one

# Check entire workspace
cargo check

# Publish workspace member
cd add_one
cargo publish
```

### External Dependencies in Workspaces
```toml
# Root Cargo.toml - shared dependencies
[workspace]
members = ["crate1", "crate2"]

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }

# crate1/Cargo.toml
[dependencies]
serde.workspace = true         # Use workspace version
tokio = { workspace = true }   # Alternative syntax
rand = "0.8"                   # Crate-specific dependency
```

### Workspace with Different Crate Types
```toml
# Workspace with library and multiple binaries
[workspace]
members = [
    "core",           # Library crate
    "cli",            # CLI application
    "web-server",     # Web server
    "worker",         # Background worker
]

# All can depend on core
```

### Testing Across Workspace
```rust
// Integration tests can test workspace interactions
// tests/integration_test.rs in workspace root
use adder;
use add_one;

#[test]
fn test_workspace_integration() {
    // Test how crates work together
    assert_eq!(add_one::add_one(5), 6);
}
```

### Workspace Patterns
```toml
# Pattern 1: Application with multiple binaries
[workspace]
members = [
    "core",
    "cli", 
    "gui",
    "web"
]

# Pattern 2: Library with examples and tools
[workspace]
members = [
    "mylib",
    "examples/basic",
    "examples/advanced", 
    "tools/codegen"
]

# Pattern 3: Microservices
[workspace]
members = [
    "shared",
    "user-service",
    "order-service",
    "payment-service"
]
```

### Workspace Best Practices
- **Shared Dependencies**: Use workspace dependencies for common deps
- **Version Consistency**: Keep related crates at same version
- **Feature Flags**: Coordinate features across workspace
- **Documentation**: Document workspace structure and relationships
- **CI/CD**: Test entire workspace together

### Workspace vs Single Crate
**Use Workspace When:**
- Multiple related binaries
- Shared code across applications
- Large project with logical divisions
- Need coordinated releases

**Use Single Crate When:**
- Simple application
- Self-contained library
- No shared code needs
- Independent release cycles

### Advanced Workspace Features
```toml
# Exclude directories from workspace
[workspace]
members = ["crate1", "crate2"]
exclude = ["old-crate", "experimental/*"]

# Default members (for cargo commands without -p)
default-members = ["main-crate"]

# Workspace inheritance
[workspace.package]
version = "1.0.0"
authors = ["Team Name"]
license = "MIT"
edition = "2021"

[workspace.dependencies]
# Define versions once, use everywhere
common-dep = "1.0"
```

Official Chapter: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

---
*Completed: ✓*