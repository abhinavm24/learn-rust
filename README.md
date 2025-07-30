# Rust Learning Plan
*Following "The Rust Programming Language" Book*

## Project Structure

```
learn/
├── Cargo.toml              # Single project configuration
├── src/lib.rs             # Shared utilities and common code
├── examples/              # All chapter code as runnable examples
│   ├── ch02_guessing_game.rs
│   ├── ch03_01_variables.rs
│   ├── ch04_01_ownership.rs
│   └── ...
└── notes/                 # Comprehensive chapter notes
    ├── INDEX.md          # Master index with cross-references
    ├── NEXT_STEPS.md     # Learning roadmap and tasks
    └── chapter-*.md      # Detailed notes for each chapter
```

## Running Examples

```bash
# Run specific chapter
cargo run --example ch04_01_ownership
cargo run --example ch02_guessing_game

# Fast syntax checking while coding
cargo check --example ch04_01_ownership

# List all examples
cargo run --example
```

## Learning Progress

Detailed notes for each chapter are in `notes/` - see [INDEX.md](notes/INDEX.md) for navigation.

## Phase 1: Getting Started (Chapters 1-3)
- [x] [Chapter 0: Introduction](notes/chapter-00.md)
- [x] [Chapter 1: Getting Started](notes/chapter-01.md) - Installation & Hello World
- [x] [Chapter 2: Programming a Guessing Game](notes/chapter-02.md)
- [x] Chapter 3: Common Programming Concepts
  - [x] [Variables and Mutability](notes/chapter-03-01.md)
  - [x] [Data Types](notes/chapter-03-02.md)
  - [x] [Functions](notes/chapter-03-03.md)
  - [x] [Comments](notes/chapter-03-04.md)
  - [x] [Control Flow](notes/chapter-03-05.md)

## Adding New Chapter Code

Create new example: `examples/chXX_YY_topic.rs`

```rust
use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter X.Y", "Topic Name");
    
    // Your chapter code here
}
```

Test: `cargo run --example chXX_YY_topic`

## RustRover Setup

**Fast Development:**
- Create run configuration: `cargo run --example ch04_01_ownership`
- Enable "Run cargo check on save" in Rust settings
- Use `cargo check --example <name>` for syntax checking

## Learning Phases

**Current Examples Ready:**
- ✅ `ch02_guessing_game` - Interactive number guessing
- ✅ `ch03_01_variables` - Variable binding and shadowing
- ✅ `ch03_02_data_types` - Scalar and compound types
- ✅ `ch03_03_functions` - Function syntax and parameters
- ✅ `ch03_05_control_flow` - if, loop, while, for
- ✅ `ch04_01_ownership` - Move semantics and ownership rules

**Next Steps:** See [NEXT_STEPS.md](notes/NEXT_STEPS.md) for prioritized learning tasks

## Resources
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings Exercises](https://github.com/rust-lang/rustlings)
- [Rust Playground](https://play.rust-lang.org/)

## Progress Tracking
Update this file as you complete each section. Mark completed items with [x].

## Notes Section
*Add your learning notes, key insights, and questions here as you progress.*

---
*Goal: Complete comprehensive Rust learning in 10 weeks*