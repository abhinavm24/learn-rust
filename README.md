# Rust Learning Examples
*Complete implementation of "The Rust Programming Language" Book*

## Project Structure

```
learn/
├── Cargo.toml              # Project configuration with 70 examples
├── src/lib.rs             # Shared utilities and common code
├── examples/              # All 70 chapter examples (Chapters 1-20)
│   ├── ch01_hello_world.rs
│   ├── ch02_guessing_game.rs
│   ├── ch03_01_variables.rs
│   ├── ch04_01_ownership.rs
│   ├── ch05_01_defining_structs.rs
│   ├── ch06_01_defining_enums.rs
│   ├── ch07_01_packages_crates.rs
│   ├── ch08_01_vectors.rs
│   ├── ch09_01_unrecoverable_errors_panic.rs
│   ├── ch10_01_generic_data_types.rs
│   ├── ch11_01_writing_tests.rs
│   ├── ch12_01_accepting_cli_args.rs
│   ├── ch13_01_closures.rs
│   ├── ch14_01_release_profiles.rs
│   ├── ch15_01_smart_pointers.rs
│   ├── ch16_01_threads.rs
│   ├── ch17_01_oop_characteristics.rs
│   ├── ch18_01_pattern_locations.rs
│   ├── ch19_01_unsafe_rust.rs
│   ├── ch20_01_single_threaded.rs
│   └── ... (and 50 more!)
├── web_assets/            # Web server assets and static files
│   ├── README.md         # Documentation for web assets
│   └── ch20_web_server/  # HTML files for Chapter 20 web server
│       ├── graceful.html # Main page for graceful shutdown server
│       ├── about.html    # Complete web server journey documentation
│       ├── shutdown.html # Technical deep-dive into graceful shutdown
│       └── 404.html      # Custom error page
└── notes/                # Comprehensive chapter notes
    ├── INDEX.md          # Master index with cross-references
    ├── NEXT_STEPS.md     # Learning roadmap and tasks
    └── chapter-*.md      # Detailed notes for each chapter
```

## Running Examples

### Basic Usage
```bash
# Run any specific chapter example
cargo run --example ch06_01_defining_enums
cargo run --example ch08_01_vectors
cargo run --example ch10_01_generic_data_types

# Fast syntax checking while coding
cargo check --examples

# Run tests in examples
cargo test --example ch11_01_writing_tests
```

### Chapter 20 Web Server Examples
The final project includes interactive web servers with HTML interfaces:

```bash
# Single-threaded web server (Chapter 20.1)
cargo run --example ch20_01_single_threaded
# Visit: http://localhost:7878

# Multithreaded web server (Chapter 20.2)  
cargo run --example ch20_02_multithreaded
# Visit: http://localhost:7879

# Graceful shutdown web server (Chapter 20.3)
cargo run --example ch20_03_graceful_shutdown
# Visit: http://localhost:7880
```

The web servers serve interactive HTML pages from `web_assets/ch20_web_server/` that provide:
- Educational content about web server concepts
- Technical documentation of implementation details
- Interactive demonstrations of graceful shutdown
- Complete timeline of the web server development journey

### All Available Examples
This project includes **70 complete examples** covering every chapter:

**Chapters 1-4: Fundamentals**
- `ch01_hello_world` - Getting started with Rust
- `ch02_guessing_game` - Interactive programming with user input
- `ch03_01_variables` - Variables, mutability, and shadowing
- `ch03_02_data_types` - Scalar and compound data types
- `ch03_03_functions` - Function definitions and parameters
- `ch03_04_comments` - Documentation and commenting styles
- `ch03_05_control_flow` - if expressions, loops, and control structures
- `ch04_01_ownership` - Ownership rules and move semantics
- `ch04_02_references_borrowing` - References and borrowing
- `ch04_03_slices` - String slices and array slices

**Chapters 5-8: Core Concepts**
- `ch05_01_defining_structs` - Struct definitions and instantiation
- `ch05_02_example_structs` - Rectangle area calculation example
- `ch05_03_method_syntax` - Methods and associated functions
- `ch06_01_defining_enums` - Enum definitions and variants
- `ch06_02_match_control_flow` - Pattern matching with match
- `ch06_03_if_let` - Concise control flow with if let
- `ch07_01_packages_crates` - Packages, crates, and modules
- `ch07_02_defining_modules` - Module system and privacy
- `ch07_03_module_paths` - Referring to items in modules
- `ch07_04_bringing_paths_into_scope` - use keyword and scope
- `ch07_05_separating_modules` - Separating modules into files
- `ch08_01_vectors` - Vector collections and operations
- `ch08_02_strings` - String handling and UTF-8
- `ch08_03_hash_maps` - HashMap collections and methods

**Chapters 9-12: Error Handling & Project Organization**
- `ch09_01_unrecoverable_errors_panic` - panic! macro and unrecoverable errors
- `ch09_02_recoverable_errors` - Result type and error handling
- `ch09_03_to_panic_or_not` - When to panic vs return errors
- `ch10_01_generic_data_types` - Generic functions, structs, and enums
- `ch10_02_traits` - Trait definitions and implementations
- `ch10_03_lifetime_syntax` - Lifetime annotations and validation
- `ch11_01_writing_tests` - Unit tests and test organization
- `ch11_02_controlling_tests` - Running and controlling test execution
- `ch11_03_test_organization` - Test organization and structure
- `ch12_01_accepting_cli_args` - Command line argument processing
- `ch12_02_reading_files` - File I/O and error handling
- `ch12_03_refactoring_modularity` - Code organization and modularity
- `ch12_04_tdd_development` - Test-driven development approach
- `ch12_05_environment_variables` - Environment variable handling

**Chapters 13-16: Advanced Features**
- `ch13_01_closures` - Closures and capturing environment
- `ch13_02_iterators` - Iterator trait and lazy evaluation
- `ch13_03_improving_io_project` - Refactoring with iterators
- `ch13_04_performance_comparison` - Performance of functional features
- `ch14_01_release_profiles` - Cargo release profiles
- `ch14_02_publishing_crates` - Publishing to crates.io
- `ch14_03_cargo_workspaces` - Workspace organization
- `ch14_04_installing_binaries` - Installing binary crates
- `ch14_05_extending_cargo` - Custom cargo commands
- `ch15_01_smart_pointers` - Box, Rc, and RefCell smart pointers
- `ch15_02_deref_trait` - Deref trait and deref coercion
- `ch15_03_drop_trait` - Drop trait and resource cleanup
- `ch15_04_rc_smart_pointer` - Reference counting with Rc
- `ch15_05_refcell_interior_mutability` - Interior mutability patterns
- `ch15_06_reference_cycles` - Memory leaks and weak references
- `ch16_01_threads` - Thread creation and management
- `ch16_02_message_passing` - Channel communication between threads
- `ch16_03_shared_state` - Shared state concurrency with Mutex and Arc

**Chapters 17-20: Object-Oriented Programming & Final Project**
- `ch17_01_oop_characteristics` - Object-oriented programming features
- `ch17_02_trait_objects` - Trait objects for dynamic dispatch
- `ch17_03_oop_implementations` - State pattern implementation
- `ch18_01_pattern_locations` - All places patterns can be used
- `ch18_02_pattern_refutability` - Refutable and irrefutable patterns
- `ch18_03_pattern_syntax` - Pattern syntax and matching
- `ch19_01_unsafe_rust` - Unsafe Rust and raw pointers
- `ch19_02_advanced_traits` - Advanced trait features
- `ch19_03_advanced_types` - Advanced type features
- `ch19_04_advanced_functions` - Function pointers and closures
- `ch19_05_macros` - Declarative and procedural macros
- `ch20_01_single_threaded` - Single-threaded web server
- `ch20_02_multithreaded` - Multithreaded web server with thread pool
- `ch20_03_graceful_shutdown` - Graceful shutdown with Drop trait

## Learning Progress

✅ **COMPLETE!** All 70 examples are implemented and functional.

Detailed notes for each chapter are in `notes/` - see [INDEX.md](notes/INDEX.md) for navigation.

## Web Assets

The `web_assets/` directory contains static files used by the examples:

### Chapter 20 Web Server Assets
- **Interactive HTML pages** with educational content
- **Styled interfaces** demonstrating web server concepts  
- **Technical documentation** embedded in web pages
- **Complete learning journey** from single-threaded to graceful shutdown

See [web_assets/README.md](web_assets/README.md) for detailed information about the web server assets.

## Development Setup

### RustRover/IntelliJ IDEA
- Create run configurations for frequently used examples
- Enable "Run cargo check on save" in Rust settings
- Use split editor to view examples and notes simultaneously

### VS Code
- Install the Rust Analyzer extension
- Use `cargo check --examples` for fast compilation checking
- Configure tasks for running specific examples

### Command Line Workflow
```bash
# Quick syntax check all examples
cargo check --examples

# Run and test specific examples
cargo run --example ch15_01_smart_pointers
cargo test --example ch11_01_writing_tests

# Build all examples in release mode
cargo build --examples --release
```

## Key Features

### 🎯 **Complete Coverage**
- All 20 chapters from The Rust Programming Language book
- 70 comprehensive examples with detailed explanations
- Both fundamental concepts and advanced features

### 🔬 **Interactive Learning**  
- Runnable examples with educational output
- Web server with interactive HTML interfaces
- Test examples demonstrating TDD approach

### 📚 **Educational Design**
- Consistent `print_chapter_header` formatting
- Detailed comments explaining concepts
- Real-world examples and use cases
- Progressive complexity building

### 🛠️ **Production-Ready Patterns**
- Error handling best practices
- Concurrency and thread safety
- Resource management with RAII
- Testing strategies and organization

## Resources

- 📖 [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - Primary reference
- 🧪 [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Additional examples
- 🏃 [Rustlings Exercises](https://github.com/rust-lang/rustlings) - Interactive exercises
- 🎮 [Rust Playground](https://play.rust-lang.org/) - Online Rust environment
- 📝 [Rust Reference](https://doc.rust-lang.org/reference/) - Language reference
- 🔧 [Cargo Book](https://doc.rust-lang.org/cargo/) - Cargo documentation

---

## Project Status: ✅ **COMPLETE**

This project provides a comprehensive, hands-on approach to learning Rust through practical examples. Each example is self-contained, well-documented, and demonstrates key concepts from The Rust Programming Language book.

**Ready for learning, teaching, and reference!** 🦀