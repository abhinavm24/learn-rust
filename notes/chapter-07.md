# Chapter 7: Managing Growing Projects with Packages, Crates, and Modules

## Key Takeaways

### Module System Hierarchy
- **Packages**: Collection of crates that provide functionality
- **Crates**: Tree of modules that produces library or executable
- **Modules**: Organize code within a crate and control privacy
- **Paths**: Way of naming items like structs, functions, modules

### Code Organization
- **Privacy by Default**: Items are private unless marked pub
- **Visibility Control**: Fine-grained control over what's exposed
- **Logical Grouping**: Related functionality grouped together
- **Namespace Management**: Avoid naming conflicts

### Module Tree Structure
- **Crate Root**: src/main.rs or src/lib.rs
- **Module Declaration**: mod keyword creates modules
- **File Organization**: Modules can be in separate files
- **Hierarchical Structure**: Nested modules create namespace hierarchy

### Code Reusability
- **use Statements**: Bring paths into scope
- **pub use**: Re-export items for external use
- **External Crates**: Include third-party functionality
- **Workspace**: Manage multiple related packages

## Chapter Structure

### 7.1: Packages and Crates
```rust
// Package structure:
// my_package/
// ├── Cargo.toml
// ├── src/
// │   ├── main.rs        // Binary crate root
// │   ├── lib.rs         // Library crate root
// │   └── bin/
// │       └── another_binary.rs

// Cargo.toml
[package]
name = "my_package"
version = "0.1.0"
edition = "2021"

[dependencies]
// External crates

// Library crate (src/lib.rs)
pub fn library_function() {
    println!("Called from library");
}

// Binary crate (src/main.rs)
use my_package::library_function;

fn main() {
    library_function();
    println!("Called from binary");
}

// Multiple binaries
// src/bin/another_binary.rs
fn main() {
    println!("Another binary");
}
```

### 7.2: Defining Modules to Control Scope and Privacy
```rust
// Module definition
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        
        fn seat_at_table() {}  // Private
    }
    
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// Using modules
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// Privacy rules
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,       // Public field
        seasonal_fruit: String,  // Private field
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    
    pub enum Appetizer {
        Soup,    // Public variant
        Salad,   // Public variant
    }
    
    fn cook_order() {}
    
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();  // Call parent module function
    }
}

fn deliver_order() {}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");  // Can modify public field
    println!("I'd like {} toast please", meal.toast);
    
    // meal.seasonal_fruit = String::from("blueberries");  // Won't compile - private
    
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

### 7.3: Paths for Referring to an Item in the Module Tree
```rust
// Absolute paths start with crate
crate::front_of_house::hosting::add_to_waitlist();

// Relative paths start from current module
front_of_house::hosting::add_to_waitlist();

// Using super to go up one level
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();  // Call parent's deliver_order
    }
    
    fn cook_order() {}
}

fn deliver_order() {}

// Making structs and enums public
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,       // Must be explicitly public
        seasonal_fruit: String,  // Private by default
    }
    
    pub enum Appetizer {
        Soup,    // Automatically public
        Salad,   // Automatically public
    }
}
```

### 7.4: Bringing Paths into Scope with the use Keyword
```rust
// Basic use
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();  // Shorter path
}

// Bringing function directly into scope
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();  // Direct call
}

// Idiomatic use patterns
use std::collections::HashMap;  // Type: bring full path
use std::fmt::Result;
use std::io::Result as IoResult;  // Alias to avoid conflicts

// Bringing multiple items
use std::cmp::Ordering;
use std::io;

// Or more concisely:
use std::{cmp::Ordering, io};

// Bringing multiple items from same module
use std::io;
use std::io::Write;

// Or more concisely:
use std::io::{self, Write};

// Glob operator
use std::collections::*;  // Brings all public items

// Re-exporting with pub use
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;  // Re-export for external use

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

### 7.5: Separating Modules into Different Files
```rust
// src/lib.rs
mod front_of_house;  // Look for src/front_of_house.rs or src/front_of_house/mod.rs

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// src/front_of_house.rs
pub mod hosting;  // Look for src/front_of_house/hosting.rs

// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}

// Alternative structure:
// src/front_of_house/mod.rs  (older style)
pub mod hosting;

// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```

## Advanced Module Patterns

### Complex Module Organization
```rust
// src/lib.rs - Library root
pub mod config;
pub mod database;
pub mod api;
pub mod utils;

pub use config::Settings;
pub use api::handlers::*;

// src/config.rs
#[derive(Debug)]
pub struct Settings {
    pub database_url: String,
    pub port: u16,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            database_url: "localhost:5432".to_string(),
            port: 8080,
        }
    }
}

// src/database/mod.rs
pub mod connection;
pub mod models;
pub mod migrations;

pub use connection::Database;
pub use models::*;

// src/database/connection.rs
pub struct Database {
    url: String,
}

impl Database {
    pub fn new(url: String) -> Self {
        Database { url }
    }
    
    pub fn connect(&self) -> Result<(), String> {
        println!("Connecting to {}", self.url);
        Ok(())
    }
}

// src/database/models.rs
#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub author_id: u32,
}

// src/api/mod.rs
pub mod handlers;
pub mod middleware;
pub mod routes;

// src/api/handlers.rs
use crate::database::models::{User, Post};

pub fn get_user(id: u32) -> Option<User> {
    // Implementation
    None
}

pub fn create_post(post: Post) -> Result<Post, String> {
    // Implementation
    Ok(post)
}

// src/utils.rs
pub fn format_timestamp(timestamp: u64) -> String {
    format!("Time: {}", timestamp)
}

pub fn validate_email(email: &str) -> bool {
    email.contains('@')
}
```

### Conditional Compilation and Features
```rust
// src/lib.rs
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Config {
    pub name: String,
    pub version: String,
}

#[cfg(debug_assertions)]
pub fn debug_info() {
    println!("Debug mode enabled");
}

#[cfg(not(debug_assertions))]
pub fn debug_info() {
    // No-op in release mode
}

// Platform-specific modules
#[cfg(target_os = "windows")]
mod windows_specific {
    pub fn get_system_info() -> String {
        "Windows system".to_string()
    }
}

#[cfg(target_os = "linux")]
mod linux_specific {
    pub fn get_system_info() -> String {
        "Linux system".to_string()
    }
}

#[cfg(target_os = "windows")]
pub use windows_specific::get_system_info;

#[cfg(target_os = "linux")]
pub use linux_specific::get_system_info;
```

### Workspace Management
```rust
// Cargo.toml (workspace root)
[workspace]
members = [
    "app",
    "shared",
    "database",
]

# Shared dependencies
[workspace.dependencies]
serde = "1.0"
tokio = "1.0"

// app/Cargo.toml
[package]
name = "app"
version = "0.1.0"
edition = "2021"

[dependencies]
shared = { path = "../shared" }
database = { path = "../database" }
serde = { workspace = true }

// shared/Cargo.toml
[package]
name = "shared"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { workspace = true }

// shared/src/lib.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

pub fn validate_user(user: &User) -> bool {
    !user.name.is_empty()
}

// app/src/main.rs
use shared::{User, validate_user};
use database::connect;

fn main() {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
    };
    
    if validate_user(&user) {
        println!("Valid user: {:?}", user);
    }
    
    connect().expect("Failed to connect to database");
}
```

## Privacy and Encapsulation Patterns

### Module-Level Privacy
```rust
mod authentication {
    use std::collections::HashMap;
    
    // Private internal state
    static mut SESSIONS: Option<HashMap<String, User>> = None;
    
    #[derive(Clone)]
    struct User {
        id: u32,
        name: String,
    }
    
    // Private helper function
    fn init_sessions() {
        unsafe {
            if SESSIONS.is_none() {
                SESSIONS = Some(HashMap::new());
            }
        }
    }
    
    // Public API
    pub fn login(username: &str, password: &str) -> Option<String> {
        init_sessions();
        
        // Validate credentials (simplified)
        if username == "admin" && password == "secret" {
            let session_id = format!("session_{}", username);
            let user = User {
                id: 1,
                name: username.to_string(),
            };
            
            unsafe {
                if let Some(ref mut sessions) = SESSIONS {
                    sessions.insert(session_id.clone(), user);
                }
            }
            
            Some(session_id)
        } else {
            None
        }
    }
    
    pub fn logout(session_id: &str) -> bool {
        unsafe {
            if let Some(ref mut sessions) = SESSIONS {
                sessions.remove(session_id).is_some()
            } else {
                false
            }
        }
    }
    
    pub fn is_authenticated(session_id: &str) -> bool {
        unsafe {
            if let Some(ref sessions) = SESSIONS {
                sessions.contains_key(session_id)
            } else {
                false
            }
        }
    }
}

pub fn main() {
    let session = authentication::login("admin", "secret");
    
    if let Some(session_id) = session {
        println!("Logged in with session: {}", session_id);
        
        if authentication::is_authenticated(&session_id) {
            println!("Session is valid");
        }
        
        authentication::logout(&session_id);
        println!("Logged out");
    }
}
```

### Builder Pattern with Modules
```rust
pub mod config {
    #[derive(Debug)]
    pub struct ServerConfig {
        host: String,
        port: u16,
        max_connections: usize,
        timeout_seconds: u64,
    }
    
    impl ServerConfig {
        pub fn host(&self) -> &str {
            &self.host
        }
        
        pub fn port(&self) -> u16 {
            self.port
        }
        
        pub fn max_connections(&self) -> usize {
            self.max_connections
        }
        
        pub fn timeout_seconds(&self) -> u64 {
            self.timeout_seconds
        }
    }
    
    pub struct ServerConfigBuilder {
        host: Option<String>,
        port: Option<u16>,
        max_connections: Option<usize>,
        timeout_seconds: Option<u64>,
    }
    
    impl ServerConfigBuilder {
        pub fn new() -> Self {
            ServerConfigBuilder {
                host: None,
                port: None,
                max_connections: None,
                timeout_seconds: None,
            }
        }
        
        pub fn host(mut self, host: impl Into<String>) -> Self {
            self.host = Some(host.into());
            self
        }
        
        pub fn port(mut self, port: u16) -> Self {
            self.port = Some(port);
            self
        }
        
        pub fn max_connections(mut self, max: usize) -> Self {
            self.max_connections = Some(max);
            self
        }
        
        pub fn timeout_seconds(mut self, timeout: u64) -> Self {
            self.timeout_seconds = Some(timeout);
            self
        }
        
        pub fn build(self) -> Result<ServerConfig, String> {
            Ok(ServerConfig {
                host: self.host.unwrap_or_else(|| "localhost".to_string()),
                port: self.port.unwrap_or(8080),
                max_connections: self.max_connections.unwrap_or(100),
                timeout_seconds: self.timeout_seconds.unwrap_or(30),
            })
        }
    }
    
    // Convenience function
    pub fn builder() -> ServerConfigBuilder {
        ServerConfigBuilder::new()
    }
}

// Usage
use config::builder;

fn main() {
    let config = builder()
        .host("0.0.0.0")
        .port(3000)
        .max_connections(1000)
        .build()
        .expect("Failed to build config");
    
    println!("Server config: {:?}", config);
}
```

## Testing Module Organization
```rust
// src/lib.rs
pub mod math;
pub mod string_utils;

// src/math.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
    
    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
    }
}

// src/string_utils.rs
pub fn capitalize(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

// tests/integration_test.rs
use my_crate::math;
use my_crate::string_utils;

#[test]
fn test_math_operations() {
    assert_eq!(math::add(1, 2), 3);
    assert_eq!(math::multiply(2, 3), 6);
}

#[test]
fn test_string_operations() {
    assert_eq!(string_utils::capitalize("hello"), "Hello");
    assert_eq!(string_utils::capitalize(""), "");
}
```

## Performance and Compilation

### Module Compilation Units
```rust
// Each module compiles separately
// Changes to one module don't recompile others unnecessarily

// src/lib.rs
pub mod expensive_computation;  // Only recompiles when this file changes
pub mod fast_operations;        // Independent compilation

// Large modules can be split for faster compilation
// src/large_module/mod.rs
pub mod part_a;  // src/large_module/part_a.rs
pub mod part_b;  // src/large_module/part_b.rs
pub mod part_c;  // src/large_module/part_c.rs
```

### Lazy Static and Module-Level State
```rust
use std::sync::Mutex;
use std::collections::HashMap;

// Module-level static state
lazy_static::lazy_static! {
    static ref GLOBAL_CONFIG: Mutex<HashMap<String, String>> = {
        let mut m = HashMap::new();
        m.insert("app_name".to_string(), "MyApp".to_string());
        Mutex::new(m)
    };
}

pub fn get_config(key: &str) -> Option<String> {
    GLOBAL_CONFIG.lock().unwrap().get(key).cloned()
}

pub fn set_config(key: String, value: String) {
    GLOBAL_CONFIG.lock().unwrap().insert(key, value);
}
```

## Best Practices

### Module Organization
```rust
// ✅ Good: Logical grouping
mod user_management {
    pub mod authentication;
    pub mod authorization;
    pub mod profiles;
}

mod data_processing {
    pub mod parsers;
    pub mod validators;
    pub mod transformers;
}

// ✅ Good: Clear public API
pub use user_management::authentication::{login, logout};
pub use data_processing::parsers::JsonParser;

// ❌ Bad: Everything public
// pub mod internal_helpers;  // Don't expose internal details

// ✅ Good: Descriptive module names
mod http_client;
mod database_connection;
mod configuration_parser;

// ❌ Bad: Unclear names
// mod utils;
// mod stuff;
// mod misc;
```

### Privacy Design
```rust
// ✅ Good: Minimal public surface
pub struct Database {
    // Private fields
    connection_pool: ConnectionPool,
    config: DatabaseConfig,
}

impl Database {
    // Public constructor
    pub fn new(config: DatabaseConfig) -> Result<Self, DatabaseError> {
        // Implementation
    }
    
    // Public methods
    pub fn query(&self, sql: &str) -> Result<QueryResult, DatabaseError> {
        // Implementation
    }
    
    // Private helper methods
    fn validate_query(&self, sql: &str) -> bool {
        // Implementation
    }
}

// ✅ Good: Use type aliases for complex types
pub type UserId = u64;
pub type QueryResult = Vec<HashMap<String, String>>;
```

### Code Reusability
```rust
// ✅ Good: Generic utility modules
pub mod serialization {
    pub fn to_json<T: serde::Serialize>(value: &T) -> Result<String, String> {
        serde_json::to_string(value).map_err(|e| e.to_string())
    }
    
    pub fn from_json<T: serde::de::DeserializeOwned>(json: &str) -> Result<T, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }
}

// ✅ Good: Feature-gated functionality
#[cfg(feature = "async")]
pub mod async_utils {
    pub async fn timeout_operation<F, T>(
        future: F,
        duration: std::time::Duration,
    ) -> Result<T, String>
    where
        F: std::future::Future<Output = T>,
    {
        // Implementation
    }
}
```

Official Chapter: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

---
*Completed: ✓*