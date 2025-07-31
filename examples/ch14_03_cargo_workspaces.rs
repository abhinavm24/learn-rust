use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 14.3", "Cargo Workspaces");
    
    println!("🏗️ Cargo Workspaces - Managing Multiple Related Packages");
    println!();
    
    demonstrate_workspace_concept();
    demonstrate_workspace_benefits();
    demonstrate_dependency_management();
    simulate_workspace_commands();
}

fn demonstrate_workspace_concept() {
    println!("📦 What is a Workspace?");
    println!();
    println!("A workspace is a set of packages that share the same Cargo.lock");
    println!("and output directory. This is useful for:");
    println!("• Large projects split into multiple crates");
    println!("• Libraries with multiple related packages");
    println!("• Projects with both library and binary crates");
    println!();
    
    println!("🏗️ Workspace Structure Example:");
    println!();
    println!("my-workspace/");
    println!("├── Cargo.toml          # Workspace root");
    println!("├── Cargo.lock          # Shared lock file");
    println!("├── target/             # Shared build directory");
    println!("├── add-one/            # Library crate");
    println!("│   ├── Cargo.toml");
    println!("│   └── src/");
    println!("│       └── lib.rs");
    println!("├── add-two/            # Another library crate");
    println!("│   ├── Cargo.toml");
    println!("│   └── src/");
    println!("│       └── lib.rs");
    println!("└── adder/              # Binary crate");
    println!("    ├── Cargo.toml");
    println!("    └── src/");
    println!("        └── main.rs");
    println!();
}

fn demonstrate_workspace_benefits() {
    println!("✅ Workspace Benefits:");
    println!();
    println!("1. 🔄 Shared Dependencies:");
    println!("   • All crates share the same versions of dependencies");
    println!("   • Prevents version conflicts between workspace members");
    println!("   • Single Cargo.lock file ensures reproducible builds");
    println!();
    
    println!("2. 🚀 Unified Build Process:");
    println!("   • Build all crates with one command");
    println!("   • Shared target directory saves disk space");
    println!("   • Incremental compilation across the workspace");
    println!();
    
    println!("3. 🧪 Coordinated Testing:");
    println!("   • Run tests for all crates simultaneously");
    println!("   • Integration tests can easily use multiple crates");
    println!("   • Consistent test environment");
    println!();
    
    println!("4. 📝 Simplified Release Management:");
    println!("   • Coordinate releases of related crates");
    println!("   • Maintain version compatibility");
    println!("   • Share common metadata and configuration");
    println!();
}

fn demonstrate_dependency_management() {
    println!("🔗 Dependency Management in Workspaces:");
    println!();
    
    println!("Root Cargo.toml:");
    println!("[workspace]");
    println!("members = [\"add-one\", \"add-two\", \"adder\"]");
    println!();
    println!("[workspace.dependencies]");
    println!("serde = {{ version = \"1.0\", features = [\"derive\"] }}");
    println!("tokio = {{ version = \"1.0\", features = [\"full\"] }}");
    println!();
    
    println!("Member Cargo.toml (add-one/Cargo.toml):");
    println!("[package]");
    println!("name = \"add-one\"");
    println!("version = \"0.1.0\"");
    println!("edition = \"2021\"");
    println!();
    println!("[dependencies]");
    println!("serde = {{ workspace = true }}  # Use workspace version");
    println!();
    
    println!("Binary Cargo.toml (adder/Cargo.toml):");
    println!("[package]");
    println!("name = \"adder\"");
    println!("version = \"0.1.0\"");
    println!("edition = \"2021\"");
    println!();
    println!("[dependencies]");
    println!("add-one = {{ path = \"../add-one\" }}  # Local dependency");
    println!("add-two = {{ path = \"../add-two\" }}");
    println!("serde = {{ workspace = true }}");
    println!();
}

fn simulate_workspace_commands() {
    println!("🛠️ Common Workspace Commands:");
    println!();
    
    println!("# Build all workspace members");
    println!("cargo build");
    println!();
    
    println!("# Build specific package");
    println!("cargo build -p add-one");
    println!();
    
    println!("# Run tests for all members");
    println!("cargo test");
    println!();
    
    println!("# Run tests for specific package");
    println!("cargo test -p adder");
    println!();
    
    println!("# Run binary from workspace");
    println!("cargo run -p adder");
    println!();
    
    println!("# Check all packages");
    println!("cargo check");
    println!();
    
    println!("# Format all code in workspace");
    println!("cargo fmt");
    println!();
    
    println!("# Lint all code in workspace");
    println!("cargo clippy");
    println!();
    
    println!("📋 Workspace Example Files:");
    println!();
    
    println!("add-one/src/lib.rs:");
    println!("pub fn add_one(x: i32) -> i32 {{");
    println!("    x + 1");
    println!("}}");
    println!();
    
    println!("add-two/src/lib.rs:");
    println!("pub fn add_two(x: i32) -> i32 {{");
    println!("    x + 2");
    println!("}}");
    println!();
    
    println!("adder/src/main.rs:");
    println!("use add_one::add_one;");
    println!("use add_two::add_two;");
    println!();
    println!("fn main() {{");
    println!("    let num = 10;");
    println!("    println!(\"{{}} plus one is {{}}\", num, add_one(num));");
    println!("    println!(\"{{}} plus two is {{}}\", num, add_two(num));");
    println!("}}");
    println!();
    
    // Simulate the functions
    println!("🧪 Simulating workspace functionality:");
    let num = 10;
    println!("{} plus one is {}", num, add_one_sim(num));
    println!("{} plus two is {}", num, add_two_sim(num));
    
    println!();
    println!("💡 Best Practices:");
    println!("• Keep workspace root Cargo.toml minimal");
    println!("• Use consistent versioning across related crates");
    println!("• Consider using workspace.dependencies for shared deps");
    println!("• Test the entire workspace before releases");
    println!("• Use path dependencies for workspace members");
}

// Simulate workspace crate functions
fn add_one_sim(x: i32) -> i32 {
    x + 1
}

fn add_two_sim(x: i32) -> i32 {
    x + 2
}