# Chapter 1: Getting Started

## Key Takeaways

### Installation & Setup
- Rust is installed via `rustup` (toolchain installer)
- Installation command: `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`
- Verify installation: `rustc --version`
- Update Rust: `rustup update`
- Uninstall: `rustup self uninstall`
- View offline docs: `rustup doc`
- `rustc` is the Rust compiler
- `cargo` is Rust's build system and package manager

### Basic Program Structure
- `fn main()` is the entry point of every Rust program
- `println!` is a macro (note the `!`) for printing to console
- Rust uses curly braces `{}` for code blocks
- Statements end with semicolons

### Compilation Process
- Rust is a compiled language (unlike interpreted languages)
- Source files have `.rs` extension
- Compile with `rustc filename.rs` or use `cargo run`
- Creates executable binary

### Cargo Basics
- `cargo new project_name` creates new project (with Git repo)
- `Cargo.toml` is the manifest file with [package] and [dependencies] sections
- Project structure: src/ for source code, target/ for builds
- `cargo build` compiles the project (debug mode, creates target/debug/)
- `cargo build --release` compiles optimized release build (target/release/)
- `cargo run` compiles and runs in one step
- `cargo check` checks code without producing executable (faster)
- Cargo.lock file manages exact dependency versions

### Key Concepts
- Rust emphasizes safety, speed, and concurrency
- Zero-cost abstractions principle
- Memory safety without garbage collection

## Personal Notes
*Add your own insights and questions here*

---
*Completed: ✓*