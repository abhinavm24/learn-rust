use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 14.4", "Installing Binaries from Crates.io");
    
    println!("⚡ Installing and Managing Rust Binaries");
    println!();
    
    demonstrate_cargo_install();
    demonstrate_binary_management();
    demonstrate_custom_registries();
    simulate_common_tools();
}

fn demonstrate_cargo_install() {
    println!("📦 Installing Binaries with `cargo install`:");
    println!();
    
    println!("# Install a binary from crates.io");
    println!("cargo install ripgrep");
    println!();
    
    println!("# Install from a git repository");
    println!("cargo install --git https://github.com/rust-lang/mdBook.git mdbook");
    println!();
    
    println!("# Install from local path");
    println!("cargo install --path .");
    println!();
    
    println!("# Install specific version");
    println!("cargo install ripgrep --version 13.0.0");
    println!();
    
    println!("# Install with specific features");
    println!("cargo install cargo-watch --features \"colors\"");
    println!();
    
    println!("🏠 Installation Location:");
    println!("• Binaries are installed to ~/.cargo/bin by default");
    println!("• This directory should be in your PATH");
    println!("• You can override with --root flag");
    println!();
    
    println!("💡 Current installation directory:");
    if let Ok(cargo_home) = std::env::var("CARGO_HOME") {
        println!("CARGO_HOME: {}/bin", cargo_home);
    } else if let Ok(home) = std::env::var("HOME") {
        println!("Default: {}/.cargo/bin", home);
    } else {
        println!("Check your CARGO_HOME or HOME environment variable");
    }
    println!();
}

fn demonstrate_binary_management() {
    println!("🛠️ Managing Installed Binaries:");
    println!();
    
    println!("# List installed binaries");
    println!("cargo install --list");
    println!();
    
    println!("# Uninstall a binary");
    println!("cargo uninstall ripgrep");
    println!();
    
    println!("# Force reinstall (useful for updates)");
    println!("cargo install --force ripgrep");
    println!();
    
    println!("# Install to custom location");
    println!("cargo install --root /usr/local ripgrep");
    println!();
    
    println!("⚠️ Important Notes:");
    println!("• `cargo install` only installs packages with binary targets");
    println!("• It doesn't automatically update packages");
    println!("• Use `cargo install --force` to update to latest version");
    println!("• Binaries are statically linked and self-contained");
    println!();
}

fn demonstrate_custom_registries() {
    println!("🌐 Alternative Registries:");
    println!();
    
    println!("# Install from alternative registry");
    println!("cargo install --registry my-registry some-tool");
    println!();
    
    println!("# Configure alternative registry in ~/.cargo/config.toml");
    println!("[registries]");
    println!("my-registry = {{ index = \"https://my-intranet:8080/index\" }}");
    println!();
    
    println!("[registry]");
    println!("default = \"my-registry\"  # Use as default registry");
    println!();
    
    println!("📋 Registry Configuration:");
    println!("• Useful for private/corporate registries");
    println!("• Can have multiple registries configured");
    println!("• Each registry needs authentication setup");
    println!();
}

fn simulate_common_tools() {
    println!("🔧 Popular Rust Tools to Install:");
    println!();
    
    let tools = vec![
        ("ripgrep", "Super fast text search tool (rg command)"),
        ("bat", "Enhanced cat with syntax highlighting"),
        ("exa", "Modern replacement for ls with colors"),
        ("fd-find", "Simple, fast alternative to find"),
        ("tokei", "Count lines of code quickly"),
        ("cargo-watch", "Watch for changes and run cargo commands"),
        ("cargo-edit", "Add, remove, upgrade dependencies via CLI"),
        ("cargo-outdated", "Check for outdated dependencies"),
        ("cargo-audit", "Audit dependencies for security vulnerabilities"),
        ("mdbook", "Create books from markdown files"),
        ("wasm-pack", "Build and package Rust for WebAssembly"),
        ("cargo-generate", "Generate projects from templates"),
    ];
    
    println!("📦 Essential Development Tools:");
    for (tool, description) in &tools {
        println!("• {:<20} - {}", tool, description);
    }
    println!();
    
    println!("🚀 Installation Commands:");
    println!("# Core development tools");
    println!("cargo install ripgrep bat fd-find");
    println!();
    println!("# Cargo extensions");
    println!("cargo install cargo-watch cargo-edit cargo-outdated");
    println!();
    println!("# Code analysis");
    println!("cargo install tokei cargo-audit");
    println!();
    
    println!("⚡ Tool Usage Examples:");
    println!();
    println!("# ripgrep - search for patterns");
    println!("rg \"fn main\" --type rust");
    println!();
    println!("# cargo-watch - auto-rebuild on changes");
    println!("cargo watch -x build");
    println!();
    println!("# cargo-edit - add dependencies");
    println!("cargo add serde --features derive");
    println!();
    println!("# tokei - count lines of code");
    println!("tokei .");
    println!();
    
    println!("🔄 Keeping Tools Updated:");
    println!("• Rust tools don't auto-update");
    println!("• Use `cargo install --force <tool>` to update");
    println!("• Consider using `cargo-update` crate for batch updates");
    println!("• Some tools provide their own update mechanisms");
    println!();
    
    println!("💡 Pro Tips:");
    println!("• Add ~/.cargo/bin to your PATH if not already there");
    println!("• Use shell completion for installed tools where available");
    println!("• Check tool documentation for configuration options");
    println!("• Some tools can be configured via environment variables");
    
    demonstrate_path_check();
}

fn demonstrate_path_check() {
    println!();
    println!("🔍 PATH Verification:");
    
    if let Ok(path) = std::env::var("PATH") {
        let cargo_bin_in_path = path.split(':')
            .any(|dir| dir.contains(".cargo/bin"));
        
        if cargo_bin_in_path {
            println!("✅ ~/.cargo/bin appears to be in your PATH");
        } else {
            println!("⚠️  ~/.cargo/bin might not be in your PATH");
            println!("Add this to your shell profile:");
            println!("export PATH=\"$HOME/.cargo/bin:$PATH\"");
        }
    } else {
        println!("❌ PATH environment variable not found");
    }
}