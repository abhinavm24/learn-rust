use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 14.2", "Publishing a Crate to Crates.io");
    
    println!("📦 Publishing to Crates.io - Documentation & Metadata");
    println!();
    
    demonstrate_documentation_comments();
    demonstrate_crate_metadata();
    demonstrate_workspace_structure();
}

/// This is a documentation comment that will appear in the generated docs
/// 
/// # Examples
/// 
/// ```
/// let result = add_one(5);
/// assert_eq!(result, 6);
/// ```
/// 
/// # Panics
/// 
/// This function doesn't panic.
/// 
/// # Errors
/// 
/// This function doesn't return errors.
/// 
/// # Safety
/// 
/// This function is safe to call.
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// Calculates the area of a rectangle
/// 
/// # Arguments
/// 
/// * `width` - The width of the rectangle
/// * `height` - The height of the rectangle
/// 
/// # Examples
/// 
/// ```
/// let area = calculate_area(10.0, 5.0);
/// assert_eq!(area, 50.0);
/// ```
pub fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}

fn demonstrate_documentation_comments() {
    println!("📝 Documentation Comments:");
    println!();
    println!("Use /// for documenting public APIs:");
    println!("/// This function adds one to the input");
    println!("/// ");
    println!("/// # Examples");
    println!("/// ");
    println!("/// ```");
    println!("/// let result = add_one(5);");
    println!("/// assert_eq!(result, 6);");
    println!("/// ```");
    println!("pub fn add_one(x: i32) -> i32 {{ x + 1 }}");
    println!();
    
    println!("🧪 Testing the documented function:");
    let result = add_one(5);
    println!("add_one(5) = {}", result);
    
    let area = calculate_area(10.0, 5.0);
    println!("calculate_area(10.0, 5.0) = {}", area);
    println!();
    
    println!("💡 Documentation sections:");
    println!("• # Examples - Show how to use the function");
    println!("• # Panics - When the function might panic");
    println!("• # Errors - Error conditions for Result-returning functions");
    println!("• # Safety - Safety guarantees for unsafe functions");
    println!();
}

fn demonstrate_crate_metadata() {
    println!("📋 Crate Metadata (Cargo.toml):");
    println!();
    println!("[package]");
    println!("name = \"my-awesome-crate\"");
    println!("version = \"0.1.0\"");
    println!("edition = \"2021\"");
    println!("authors = [\"Your Name <your.email@example.com>\"]");
    println!("license = \"MIT OR Apache-2.0\"");
    println!("description = \"A brief description of what this crate does\"");
    println!("homepage = \"https://github.com/username/my-awesome-crate\"");
    println!("repository = \"https://github.com/username/my-awesome-crate\"");
    println!("readme = \"README.md\"");
    println!("keywords = [\"cli\", \"tool\", \"utility\"]");
    println!("categories = [\"command-line-utilities\"]");
    println!();
    
    println!("🏷️ Version Requirements:");
    println!("• Follow Semantic Versioning (SemVer)");
    println!("• MAJOR.MINOR.PATCH");
    println!("• Breaking changes increment MAJOR");
    println!("• New features increment MINOR");
    println!("• Bug fixes increment PATCH");
    println!();
}

fn demonstrate_workspace_structure() {
    println!("🏗️ Workspace Structure:");
    println!();
    println!("Cargo.toml (workspace root):");
    println!("[workspace]");
    println!("members = [");
    println!("    \"my-lib\",");
    println!("    \"my-binary\",");
    println!("    \"shared-utils\",");
    println!("]");
    println!();
    
    println!("📁 Directory structure:");
    println!("my-workspace/");
    println!("├── Cargo.toml");
    println!("├── my-lib/");
    println!("│   ├── Cargo.toml");
    println!("│   └── src/");
    println!("│       └── lib.rs");
    println!("├── my-binary/");
    println!("│   ├── Cargo.toml");
    println!("│   └── src/");
    println!("│       └── main.rs");
    println!("└── shared-utils/");
    println!("    ├── Cargo.toml");
    println!("    └── src/");
    println!("        └── lib.rs");
    println!();
    
    println!("🚀 Publishing Steps:");
    println!("1. cargo login <token>");
    println!("2. cargo publish --dry-run  # Test without publishing");
    println!("3. cargo publish            # Actually publish");
    println!();
    
    println!("⚠️ Important Notes:");
    println!("• Crate names must be unique on crates.io");
    println!("• You cannot delete published versions");
    println!("• You can yank versions to prevent new projects from using them");
    println!("• Use `cargo yank --vers 1.0.1` to yank a version");
    println!("• Use `cargo yank --vers 1.0.1 --undo` to un-yank");
}