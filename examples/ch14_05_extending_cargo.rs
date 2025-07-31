use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 14.5", "Extending Cargo with Custom Commands");
    
    println!("🔧 Extending Cargo with Custom Commands");
    println!();
    
    demonstrate_cargo_subcommands();
    demonstrate_popular_extensions();
    create_custom_command_example();
    demonstrate_cargo_configuration();
}

fn demonstrate_cargo_subcommands() {
    println!("⚡ How Cargo Subcommands Work:");
    println!();
    
    println!("Cargo looks for binaries named `cargo-<subcommand>` in PATH");
    println!("When you run `cargo hello`, Cargo executes `cargo-hello`");
    println!();
    
    println!("📋 Subcommand Discovery:");
    println!("• Cargo searches PATH for binaries starting with 'cargo-'");
    println!("• The part after 'cargo-' becomes the subcommand name");
    println!("• Subcommands can be written in any language");
    println!("• Rust-based subcommands are most common and easiest to distribute");
    println!();
    
    println!("🔍 List Available Subcommands:");
    println!("cargo --list");
    println!();
    
    println!("Built-in commands vs extensions:");
    println!("• Built-in: build, run, test, doc, publish, etc.");
    println!("• Extensions: watch, edit, audit, outdated, etc.");
    println!();
}

fn demonstrate_popular_extensions() {
    println!("🌟 Popular Cargo Extensions:");
    println!();
    
    let extensions = vec![
        ("cargo-watch", "Automatically run commands when files change", "cargo watch -x build"),
        ("cargo-edit", "Add, remove, and upgrade dependencies", "cargo add serde"),
        ("cargo-outdated", "Check for outdated dependencies", "cargo outdated"),
        ("cargo-audit", "Audit dependencies for security vulnerabilities", "cargo audit"),
        ("cargo-tree", "Display dependency tree (now built-in)", "cargo tree"),
        ("cargo-expand", "Show macro expansions", "cargo expand"),
        ("cargo-clippy", "Additional lints (now built-in)", "cargo clippy"),
        ("cargo-fmt", "Code formatting (now built-in)", "cargo fmt"),
        ("cargo-bench", "Benchmarking", "cargo bench"),
        ("cargo-doc", "Generate documentation", "cargo doc --open"),
        ("cargo-test", "Run tests with options", "cargo test --nocapture"),
        ("cargo-check", "Fast compilation check", "cargo check"),
        ("cargo-update", "Update installed cargo extensions", "cargo install-update -a"),
        ("cargo-generate", "Generate projects from templates", "cargo generate --git template-repo"),
        ("cargo-deny", "Cargo plugin for linting dependencies", "cargo deny check"),
        ("cargo-make", "Task runner and build tool", "cargo make build-all"),
    ];
    
    for (name, description, example) in extensions {
        println!("📦 {}", name);
        println!("   Description: {}", description);
        println!("   Usage: {}", example);
        println!();
    }
}

fn create_custom_command_example() {
    println!("🛠️ Creating a Custom Cargo Command:");
    println!();
    
    println!("1. Create a new binary project:");
    println!("cargo new --bin cargo-hello");
    println!();
    
    println!("2. main.rs example:");
    println!("use std::env;");
    println!();
    println!("fn main() {{");
    println!("    let args: Vec<String> = env::args().collect();");
    println!("    ");
    println!("    // Skip the first argument (program name)");
    println!("    // and the second (\"hello\" subcommand name)");
    println!("    let message_args = &args[2..];");
    println!("    ");
    println!("    if message_args.is_empty() {{");
    println!("        println!(\"Hello from cargo-hello!\");");
    println!("    }} else {{");
    println!("        println!(\"Hello {{}}!\", message_args.join(\" \"));");
    println!("    }}");
    println!("}}");
    println!();
    
    println!("3. Build and install:");
    println!("cargo build --release");
    println!("cargo install --path .");
    println!();
    
    println!("4. Use the command:");
    println!("cargo hello");
    println!("cargo hello world");
    println!();
    
    // Simulate the custom command
    println!("🧪 Simulating custom command:");
    simulate_cargo_hello(&[]);
    simulate_cargo_hello(&["world", "from", "Rust"]);
    println!();
}

fn simulate_cargo_hello(args: &[&str]) {
    if args.is_empty() {
        println!("Hello from cargo-hello!");
    } else {
        println!("Hello {}!", args.join(" "));
    }
}

fn demonstrate_cargo_configuration() {
    println!("⚙️ Cargo Configuration and Aliases:");
    println!();
    
    println!("~/.cargo/config.toml:");
    println!("[alias]");
    println!("b = \"build\"");
    println!("c = \"check\"");
    println!("t = \"test\"");
    println!("r = \"run\"");
    println!("br = \"build --release\"");
    println!("wr = \"watch -x run\"");
    println!("wt = \"watch -x test\"");
    println!("tree = \"tree --all-features\"");
    println!();
    
    println!("Advanced aliases with shell commands:");
    println!("[alias]");
    println!("# Run tests and then build");
    println!("test-build = \"!f() {{ cargo test && cargo build; }}; f\"");
    println!();
    println!("# Format, lint, and test");
    println!("check-all = \"!f() {{ cargo fmt && cargo clippy && cargo test; }}; f\"");
    println!();
    
    println!("🎯 Target-specific configuration:");
    println!("[target.x86_64-unknown-linux-gnu]");
    println!("linker = \"clang\"");
    println!("rustflags = [\"-C\", \"link-arg=-fuse-ld=lld\"]");
    println!();
    
    println!("[build]");
    println!("target = \"x86_64-unknown-linux-gnu\"  # Default target");
    println!("jobs = 4                              # Parallel build jobs");
    println!();
    
    println!("🌐 Registry configuration:");
    println!("[registry]");
    println!("default = \"crates-io\"");
    println!();
    println!("[registries.crates-io]");
    println!("index = \"https://github.com/rust-lang/crates.io-index\"");
    println!();
    
    println!("🔐 Credentials configuration:");
    println!("~/.cargo/credentials.toml:");
    println!("[registry]");
    println!("token = \"your-api-token-here\"");
    println!();
    
    println!("💡 Environment Variables:");
    println!("• CARGO_HOME - Cargo's home directory");
    println!("• CARGO_TARGET_DIR - Build output directory");  
    println!("• RUSTFLAGS - Additional rustc flags");
    println!("• CARGO_INCREMENTAL - Enable/disable incremental compilation");
    println!();
    
    demonstrate_environment_variables();
}

fn demonstrate_environment_variables() {
    println!("🔍 Current Cargo Environment:");
    
    let env_vars = [
        ("CARGO_HOME", "Cargo home directory"),
        ("CARGO_TARGET_DIR", "Build target directory"),
        ("RUSTFLAGS", "Additional rustc flags"),
        ("CARGO_INCREMENTAL", "Incremental compilation"),
    ];
    
    for (var, description) in env_vars {
        match std::env::var(var) {
            Ok(value) => println!("• {}: {} ({})", var, value, description),
            Err(_) => println!("• {}: Not set ({})", var, description),
        }
    }
    
    println!();
    println!("🚀 Pro Tips for Custom Commands:");
    println!("• Use clap or structopt for argument parsing");
    println!("• Follow Unix conventions for flags and options");
    println!("• Provide helpful error messages and usage information");
    println!("• Consider publishing useful commands to crates.io");
    println!("• Use cargo metadata to interact with Cargo project info");
    println!("• Test your commands with different project structures");
}