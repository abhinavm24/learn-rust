# Chapter 1: Getting Started

## Key Takeaways

### Installation & Setup
- Rust is installed via `rustup` (toolchain installer)
- Installation command: `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`
- **Platform Requirements:**
  - macOS: Install C compiler with `xcode-select --install`
  - Linux: Install GCC or Clang via package manager
  - Windows: Install Visual Studio for linker and native libraries
- Verify installation: `rustc --version` (format: `rustc x.y.z (abcabcabc yyyy-mm-dd)`)
- Update Rust: `rustup update`
- Uninstall: `rustup self uninstall`
- View offline docs: `rustup doc`
- `rustc` is the Rust compiler
- `cargo` is Rust's build system and package manager

### Hello World Program
- **File naming**: `main.rs` (multi-word files use underscores)
- **Directory structure**: `~/projects/hello_world/` recommended
- `fn main()` is the entry point of every Rust program
- `println!` is a macro (note the `!`) for printing to console
- Rust uses curly braces `{}` for code blocks
- Statements end with semicolons
- **Execution**: `./main` (Linux/macOS), `.\main` (Windows)
- `rustfmt` available for consistent code formatting

### Compilation Process
- Rust is **ahead-of-time compiled** (not interpreted)
- Source files have `.rs` extension
- Compile with `rustc filename.rs` or use `cargo run`
- Creates standalone executable binary
- Executable naming: same as source file without `.rs`
- Compilation separate from execution (unlike scripting languages)

### Cargo Basics
- `cargo new project_name` creates new project (with Git repo)
- `cargo init` converts existing project to Cargo project
- **Configuration**: `Cargo.toml` uses TOML format (Tom's Obvious, Minimal Language)
  - `[package]` section: name, version, edition
  - `[dependencies]` section: external crates
- **Project structure**: 
  - `src/` directory for source code
  - `target/debug/` for debug builds
  - `target/release/` for release builds
  - Top-level for configuration, README, etc.
- `cargo build` compiles the project (debug mode)
- `cargo build --release` compiles optimized release build
- `cargo run` compiles and runs in one step
- `cargo check` checks code without producing executable (faster)
- `Cargo.lock` file manages exact dependency versions (auto-generated)
- Cross-platform consistency of commands

### Key Concepts
- Rust emphasizes safety, speed, and concurrency
- Zero-cost abstractions principle
- Memory safety without garbage collection
- **Development workflow**: typical Rust project lifecycle
- **Build modes**: Debug (development) vs Release (production)
- **Community**: Rustaceans (Rust community members)
- **Macro system**: Code generation with `!` syntax

### Offline Development
- `cargo new get-dependencies` for offline setup
- `cargo add rand@0.8.5 trpl@0.2.0` to pre-download dependencies
- `--offline` flag for offline builds

---
*Completed: ✓*