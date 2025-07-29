# Chapter 14.1: Customizing Builds with Release Profiles

## Key Takeaways

### Release Profiles
- **Build Configurations**: Different settings for different build scenarios
- **dev Profile**: Default for `cargo build`, optimized for development
- **release Profile**: Default for `cargo build --release`, optimized for production
- **Customization**: Override default settings in Cargo.toml

### Default Profile Settings
```toml
# Dev profile (implicit defaults)
[profile.dev]
opt-level = 0      # No optimizations
debug = true       # Include debug info
split-debuginfo = '...'  # Platform-specific
strip = false      # Don't strip symbols
debug-assertions = true   # Enable debug assertions
overflow-checks = true    # Check for integer overflow
lto = false        # No link-time optimization
panic = 'unwind'   # Stack unwinding on panic
incremental = true # Incremental compilation
codegen-units = 256 # Parallel code generation

# Release profile (implicit defaults)
[profile.release]
opt-level = 3      # Maximum optimizations
debug = false      # No debug info
strip = false      # Don't strip symbols
debug-assertions = false  # Disable debug assertions
overflow-checks = false   # No overflow checks
lto = false        # No link-time optimization
panic = 'unwind'   # Stack unwinding on panic
incremental = false # No incremental compilation
codegen-units = 16  # Fewer units for better optimization
```

### Customizing Profiles
```toml
[profile.dev]
opt-level = 1      # Some optimization for dev builds

[profile.release]
lto = true         # Enable link-time optimization
panic = 'abort'    # Abort on panic (smaller binary)
strip = true       # Strip debug symbols
codegen-units = 1  # Single unit for maximum optimization
```

### Optimization Levels
- **opt-level = 0**: No optimization (fastest compile)
- **opt-level = 1**: Basic optimization
- **opt-level = 2**: Some optimization
- **opt-level = 3**: Maximum optimization (default release)
- **opt-level = "s"**: Optimize for size
- **opt-level = "z"**: Optimize aggressively for size

### Debug Information
```toml
[profile.release]
debug = true       # Include debug info in release
debug = 1          # Line tables only
debug = 2          # Full debug info (default for dev)
strip = "none"     # Keep all symbols
strip = "debuginfo" # Strip debug info only
strip = "symbols"  # Strip all symbols
```

### Link-Time Optimization (LTO)
```toml
[profile.release]
lto = false        # No LTO (fastest compile)
lto = "thin"       # Thin LTO (good balance)
lto = true         # Full LTO (best optimization, slow compile)
lto = "fat"        # Same as true
```

### Panic Behavior
```toml
[profile.release]
panic = "unwind"   # Stack unwinding (default)
panic = "abort"    # Abort immediately (smaller binary)
```

### Code Generation Units
```toml
[profile.release]
codegen-units = 1  # Single unit (best optimization)
codegen-units = 16 # Multiple units (faster compile)
```

### Example Custom Profile
```toml
# Fast debug builds
[profile.dev]
opt-level = 1
debug = true
incremental = true

# Size-optimized release
[profile.release]
opt-level = "z"
lto = true
panic = "abort"
strip = true
codegen-units = 1

# Custom profile for benchmarking
[profile.bench]
inherits = "release"
debug = true
```

### Profile Inheritance
```toml
[profile.custom]
inherits = "release"  # Start with release settings
opt-level = 2         # Override specific settings
debug = true
```

### Build Commands
```bash
cargo build                    # Uses dev profile
cargo build --release         # Uses release profile
cargo build --profile custom  # Uses custom profile
```

Official Chapter: https://doc.rust-lang.org/book/ch14-01-release-profiles.html

---
*Completed: ✓*