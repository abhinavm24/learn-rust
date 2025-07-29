# Chapter 7.5: Separating Modules into Different Files

## Key Takeaways

### File-Based Module Organization
- **Multi-File Modules**: Split large modules into separate files for better organization
- **Module Declaration**: Use `mod module_name;` to load module from external file
- **File Naming**: Module files should match module names (snake_case)
- **Directory Structure**: Submodules can use directories for further organization

### File Organization Patterns
- **Single File**: `src/module_name.rs` for simple modules
- **Directory with mod.rs**: `src/module_name/mod.rs` for modules with submodules
- **Directory with named file**: `src/module_name.rs` + `src/module_name/submodule.rs`
- Modern style favors named files over `mod.rs` when possible

### Module Loading Process
- Compiler looks for module code in specific locations based on declaration
- `mod` declarations create the module tree structure
- File contents become the module body
- Privacy rules and `use` statements work the same across files

### Important Concepts

#### Module Declaration Syntax
```rust
// In src/lib.rs or src/main.rs
mod module_name;  // Loads from src/module_name.rs or src/module_name/mod.rs
```

#### File Paths
- `mod foo;` → `src/foo.rs` or `src/foo/mod.rs`
- `mod bar;` inside `foo` → `src/foo/bar.rs` or `src/foo/bar/mod.rs`

### Programming Concepts Introduced
- **Code Organization**: Splitting functionality across multiple files
- **Build System Integration**: How Rust compiler finds and compiles modules
- **Project Structure**: Best practices for organizing larger projects
- **Module Tree Mapping**: Relationship between file structure and module hierarchy

### Code Examples and Patterns

#### Basic File Separation Example

**Initial Single File (src/lib.rs):**
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}
    }
}

pub use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

**After Separation:**

**src/lib.rs:**
```rust
mod front_of_house;

pub use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

**src/front_of_house.rs:**
```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
    pub fn seat_at_table() {}
}

pub mod serving {
    pub fn take_order() {}
    pub fn serve_order() {}
    pub fn take_payment() {}
}
```

#### Further Separation with Subdirectories

**src/lib.rs:**
```rust
mod front_of_house;

pub use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

**src/front_of_house.rs:**
```rust
pub mod hosting;
pub mod serving;
```

**src/front_of_house/hosting.rs:**
```rust
pub fn add_to_waitlist() {}
pub fn seat_at_table() {}
```

**src/front_of_house/serving.rs:**
```rust
pub fn take_order() {}
pub fn serve_order() {}
pub fn take_payment() {}
```

#### Real-World Web Server Example

**Project Structure:**
```
src/
├── lib.rs
├── server.rs
├── config.rs
├── database/
│   ├── mod.rs
│   ├── connection.rs
│   └── models.rs
├── handlers/
│   ├── mod.rs
│   ├── user.rs
│   └── auth.rs
└── utils/
    ├── mod.rs
    ├── validation.rs
    └── logging.rs
```

**src/lib.rs:**
```rust
pub mod server;
pub mod config;
pub mod database;
pub mod handlers;
pub mod utils;

// Re-export commonly used items
pub use server::Server;
pub use config::Config;
pub use database::Database;
```

**src/server.rs:**
```rust
use crate::config::Config;
use crate::database::Database;
use crate::handlers;

pub struct Server {
    config: Config,
    database: Database,
}

impl Server {
    pub fn new(config: Config) -> Self {
        let database = Database::new(&config.database_url);
        Server { config, database }
    }
    
    pub fn start(&self) {
        println!("Server starting on port {}", self.config.port);
        // Server implementation
    }
}
```

**src/config.rs:**
```rust
#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Config {
            port: 8080,
            host: "localhost".to_string(),
            database_url: "postgres://localhost/myapp".to_string(),
        })
    }
}
```

**src/database/mod.rs:**
```rust
pub mod connection;
pub mod models;

pub use connection::Database;
pub use models::{User, Post};
```

**src/database/connection.rs:**
```rust
use super::models::{User, Post};

pub struct Database {
    connection_url: String,
}

impl Database {
    pub fn new(url: &str) -> Self {
        Database {
            connection_url: url.to_string(),
        }
    }
    
    pub fn get_user(&self, id: u32) -> Option<User> {
        // Database query implementation
        None
    }
    
    pub fn get_posts(&self) -> Vec<Post> {
        // Database query implementation
        Vec::new()
    }
}
```

**src/database/models.rs:**
```rust
#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

#[derive(Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub author_id: u32,
}
```

**src/handlers/mod.rs:**
```rust
pub mod user;
pub mod auth;

pub use user::UserHandler;
pub use auth::AuthHandler;
```

**src/handlers/user.rs:**
```rust
use crate::database::{Database, User};

pub struct UserHandler {
    database: Database,
}

impl UserHandler {
    pub fn new(database: Database) -> Self {
        UserHandler { database }
    }
    
    pub fn get_user(&self, id: u32) -> Option<User> {
        self.database.get_user(id)
    }
    
    pub fn create_user(&self, username: String, email: String) -> Result<User, String> {
        // User creation logic
        Ok(User {
            id: 1,
            username,
            email,
        })
    }
}
```

**src/handlers/auth.rs:**
```rust
use crate::database::{Database, User};

pub struct AuthHandler {
    database: Database,
}

impl AuthHandler {
    pub fn new(database: Database) -> Self {
        AuthHandler { database }
    }
    
    pub fn login(&self, username: &str, password: &str) -> Result<User, String> {
        // Authentication logic
        Err("Invalid credentials".to_string())
    }
    
    pub fn logout(&self, user_id: u32) -> Result<(), String> {
        // Logout logic
        Ok(())
    }
}
```

**src/utils/mod.rs:**
```rust
pub mod validation;
pub mod logging;

pub use validation::Validator;
pub use logging::Logger;
```

#### Alternative Organization with mod.rs

**Alternative Structure:**
```
src/
├── lib.rs
├── server/
│   ├── mod.rs
│   ├── http.rs
│   └── websocket.rs
├── database/
│   ├── mod.rs
│   ├── connection.rs
│   └── models.rs
```

**src/server/mod.rs:**
```rust
pub mod http;
pub mod websocket;

pub use http::HttpServer;
pub use websocket::WsServer;

pub enum ServerType {
    Http(HttpServer),
    WebSocket(WsServer),
}
```

**src/server/http.rs:**
```rust
pub struct HttpServer {
    port: u16,
}

impl HttpServer {
    pub fn new(port: u16) -> Self {
        HttpServer { port }
    }
    
    pub fn start(&self) {
        println!("HTTP server starting on port {}", self.port);
    }
}
```

#### Binary with Library Structure

**Cargo.toml:**
```toml
[package]
name = "my-app"
version = "0.1.0"

[[bin]]
name = "server"
path = "src/bin/server.rs"

[[bin]]
name = "client"
path = "src/bin/client.rs"
```

**src/lib.rs:**
```rust
pub mod config;
pub mod network;
pub mod utils;

pub use config::Config;
pub use network::{Client, Server};
```

**src/bin/server.rs:**
```rust
use my_app::{Config, Server};

fn main() {
    let config = Config::from_env().expect("Failed to load config");
    let server = Server::new(config);
    server.start();
}
```

**src/bin/client.rs:**
```rust
use my_app::{Config, Client};

fn main() {
    let config = Config::from_env().expect("Failed to load config");
    let client = Client::new(config);
    client.connect();
}
```

#### Testing Organization

**tests/ Directory Structure:**
```
tests/
├── common/
│   └── mod.rs
├── integration_test.rs
└── api_test.rs
```

**tests/common/mod.rs:**
```rust
use my_app::{Config, Server};

pub fn setup_test_server() -> Server {
    let config = Config {
        port: 0, // Random port for testing
        host: "127.0.0.1".to_string(),
        database_url: ":memory:".to_string(),
    };
    Server::new(config)
}
```

**tests/integration_test.rs:**
```rust
mod common;

use common::setup_test_server;

#[test]
fn test_server_creation() {
    let server = setup_test_server();
    // Test server functionality
}
```

### File Organization Best Practices

#### When to Split Modules
- Module file becomes too large (>200-300 lines)
- Module has multiple related but distinct responsibilities
- You want to separate public API from implementation
- Team members work on different parts of the module

#### Naming Conventions
- Use snake_case for module file names
- Match file names to module names exactly
- Use descriptive directory names for module groups
- Keep file names concise but clear

#### Directory Structure Guidelines
- Group related modules in directories
- Keep flat structure when possible (avoid deep nesting)
- Use `mod.rs` for modules that need to export submodules
- Consider using workspace for very large projects

### Integration with Previous Chapters
- Uses module privacy rules from Chapter 7.2
- Applies path resolution from Chapter 7.3
- Works with `use` statements from Chapter 7.4
- Builds on package structure from Chapter 7.1

### Community Conventions and Idioms
- Prefer named files (`module.rs`) over `mod.rs` when possible
- Keep `lib.rs` focused on public API and module declarations
- Use consistent organization patterns throughout the project
- Document module organization in README files
- Consider using workspaces for very large, multi-crate projects

### Personal Notes
- File-based modules make large projects much more manageable
- Good file organization is crucial for team collaboration
- The module tree structure should match the problem domain
- Refactoring to separate files is usually straightforward
- Proper organization makes code more discoverable and maintainable

Official Chapter: https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html

---
*Completed: ✓*