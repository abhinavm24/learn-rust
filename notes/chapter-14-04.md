# Chapter 14.4: Installing Binaries with cargo install

## Key Takeaways

### cargo install Overview
- **Binary Installation**: Install executable crates from crates.io or local sources
- **Global Installation**: Binaries available system-wide in PATH
- **Source Building**: Compiles from source code, not pre-built binaries
- **Local Installation**: Installs to ~/.cargo/bin by default

### Basic Installation Commands
```bash
# Install from crates.io
cargo install ripgrep          # Install latest version
cargo install ripgrep --version 13.0.0  # Install specific version

# Install from local path
cargo install --path .         # Install current directory
cargo install --path ../my-tool

# Install from git repository
cargo install --git https://github.com/user/repo
cargo install --git https://github.com/user/repo --branch main
cargo install --git https://github.com/user/repo --tag v1.0.0

# Install with features
cargo install ripgrep --features pcre2
cargo install my-tool --no-default-features --features minimal
```

### Installation Location
```bash
# Default installation directory
~/.cargo/bin/               # Unix-like systems
%USERPROFILE%\.cargo\bin\   # Windows

# Custom installation directory
cargo install --root /custom/path my-tool

# List installed packages
cargo install --list
```

### Managing Installed Binaries
```bash
# List all installed binaries
cargo install --list

# Uninstall a binary
cargo uninstall ripgrep

# Reinstall/update a binary
cargo install ripgrep --force  # Force reinstall even if installed

# Update all installed binaries
# (No built-in command, use external tools like cargo-update)
```

### Installation Requirements
- **Binary Crate**: Must be a crate with [[bin]] targets
- **Compilation**: Requires Rust toolchain for building
- **Dependencies**: Must be able to resolve all dependencies
- **Platform**: Must be compatible with target platform

### Example: Installing Development Tools
```bash
# Useful development tools from crates.io
cargo install cargo-watch     # Auto-rebuild on file changes
cargo install cargo-edit      # Add/remove dependencies from CLI
cargo install cargo-audit     # Security vulnerability scanner
cargo install cargo-outdated  # Check for outdated dependencies
cargo install cargo-tree      # Visualize dependency tree
cargo install cargo-expand    # Show macro expansions
cargo install tokei           # Count lines of code
cargo install fd-find         # Fast file finder
cargo install bat             # Cat clone with syntax highlighting
cargo install exa             # Modern ls replacement
```

### Installing from Different Sources
```bash
# From crates.io (default)
cargo install my-tool

# From local development
cd my-project
cargo install --path .

# From git with specific commit
cargo install --git https://github.com/user/repo --rev abc123

# From alternative registry
cargo install --registry my-registry my-tool
```

### Installation with Build Options
```bash
# Install with release optimizations
cargo install my-tool --profile release

# Install with specific target
cargo install --target x86_64-pc-windows-gnu my-tool

# Install with custom features
cargo install diesel_cli --no-default-features --features postgres

# Install with environment variables
RUSTFLAGS="-C target-cpu=native" cargo install my-tool
```

### Creating Installable Binaries
```toml
# Cargo.toml for installable binary
[package]
name = "my-tool"
version = "1.0.0"
edition = "2021"

[[bin]]
name = "my-tool"
path = "src/main.rs"

# Multiple binaries
[[bin]]
name = "tool-cli"
path = "src/cli.rs"

[[bin]]
name = "tool-daemon"
path = "src/daemon.rs"

# Installable library with binary
[lib]
name = "my_lib"

[[bin]]
name = "my-tool"
path = "src/bin/main.rs"
```

### Installation Scripts and Automation
```bash
#!/bin/bash
# install-dev-tools.sh

tools=(
    "ripgrep"
    "fd-find" 
    "bat"
    "exa"
    "tokei"
    "cargo-watch"
    "cargo-edit"
)

for tool in "${tools[@]}"; do
    echo "Installing $tool..."
    cargo install "$tool"
done
```

### PATH Configuration
```bash
# Add cargo bin directory to PATH
# ~/.bashrc or ~/.zshrc
export PATH="$HOME/.cargo/bin:$PATH"

# Windows PowerShell profile
$env:PATH += ";$env:USERPROFILE\.cargo\bin"

# Verify installation
which ripgrep
ripgrep --version
```

### Troubleshooting Installation
```bash
# Check for compilation errors
cargo install my-tool --verbose

# Clear cargo cache if needed
cargo clean
rm -rf ~/.cargo/registry/cache

# Install with different rust version
rustup install stable
rustup run stable cargo install my-tool

# Check system dependencies
# Some tools may require system libraries
sudo apt-get install build-essential  # Ubuntu/Debian
brew install gcc                       # macOS
```

### Alternative Installation Methods
```bash
# Using cargo-binstall (faster, uses pre-built binaries when available)
cargo install cargo-binstall
cargo binstall ripgrep  # Uses binary if available, falls back to source

# Using system package managers
brew install ripgrep    # macOS
apt install ripgrep     # Ubuntu/Debian
scoop install ripgrep   # Windows
```

### Installation Best Practices
- **Check Documentation**: Read crate documentation for installation notes
- **Version Pinning**: Pin versions for reproducible environments
- **Feature Selection**: Only install needed features for smaller binaries
- **Regular Updates**: Keep installed tools updated for security
- **Backup Lists**: Maintain list of installed tools for easy restoration

### Common Issues and Solutions
```bash
# Permission denied
sudo chown -R $(whoami) ~/.cargo

# Out of date registry
cargo install --force my-tool

# Missing system dependencies
# Check crate documentation for required libraries

# Compilation failures
# May need specific Rust version or system setup
```

### Cargo Install vs System Package Managers
**Use cargo install when:**
- Latest version needed
- Custom features required
- Development/testing versions
- Rust-specific tools

**Use system package manager when:**
- Stable, well-tested versions sufficient
- System integration important
- Automatic security updates desired
- Non-technical users installing

Official Chapter: https://doc.rust-lang.org/book/ch14-04-installing-binaries-with-cargo-install.html

---
*Completed: ✓*