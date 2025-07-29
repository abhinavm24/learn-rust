# Chapter 7.3: Paths for Referring to an Item in the Module Tree

## Key Takeaways

### Path Fundamentals
- **Paths**: Navigate the module tree to find items, similar to filesystem paths
- **Two Path Types**: Absolute paths (from crate root) and relative paths (from current module)
- **Path Resolution**: Compiler uses paths to locate functions, structs, modules, etc.
- **Privacy Rules**: Paths must respect module privacy boundaries

### Absolute vs Relative Paths
- **Absolute Path**: Starts from crate root using `crate` keyword
- **Relative Path**: Starts from current module using `self`, `super`, or identifier
- **External Crates**: Use crate name as the root (e.g., `std::collections::HashMap`)
- **Path Separators**: Use `::` to separate path components

### Special Path Keywords
- `crate` - Root of current crate (absolute path start)
- `self` - Current module (relative path)
- `super` - Parent module (relative path)
- Module names and item names for navigation

### Important Syntax and Operators

#### Absolute Path Syntax
```rust
crate::module_name::item_name
external_crate::module_name::item_name
```

#### Relative Path Syntax
```rust
module_name::item_name
self::item_name
super::item_name
super::super::item_name
```

### Programming Concepts Introduced
- **Module Navigation**: Traversing hierarchical code organization
- **Scope Resolution**: Finding the correct item when names might conflict
- **Path Strategy**: Choosing between absolute and relative paths
- **Privacy Traversal**: Understanding what paths are accessible

### Code Examples and Patterns

#### Basic Path Usage
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

#### Using super for Parent Access
```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // Access parent function
    }
    
    fn cook_order() {}
}
```

#### Complex Module Structure with Paths
```rust
mod restaurant {
    pub mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("Adding to waitlist");
            }
            
            pub fn seat_at_table() {
                println!("Seating at table");
            }
        }
        
        pub mod serving {
            pub fn take_order() {
                println!("Taking order");
                
                // Call function in sibling module
                super::hosting::add_to_waitlist();
                
                // Call function in back of house
                crate::restaurant::back_of_house::prepare_food();
            }
            
            pub fn serve_order() {
                println!("Serving order");
            }
        }
    }
    
    pub mod back_of_house {
        pub fn prepare_food() {
            println!("Preparing food");
        }
        
        fn clean_kitchen() {
            println!("Cleaning kitchen");
        }
        
        pub mod kitchen {
            pub fn cook() {
                println!("Cooking");
                
                // Access parent module function
                super::prepare_food();
                
                // Access grandparent function (won't work - clean_kitchen is private)
                // super::clean_kitchen();
            }
        }
    }
}

pub fn run_restaurant() {
    // Absolute paths
    crate::restaurant::front_of_house::hosting::add_to_waitlist();
    crate::restaurant::front_of_house::serving::take_order();
    crate::restaurant::back_of_house::kitchen::cook();
    
    // Relative paths
    restaurant::front_of_house::hosting::seat_at_table();
    restaurant::front_of_house::serving::serve_order();
}
```

#### Struct Field Privacy
```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // Private field
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
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    
    // Enum variants are public when enum is public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

#### Real-World Library Example
```rust
// File: src/lib.rs
pub mod network {
    pub mod client {
        pub struct HttpClient {
            base_url: String,
            timeout: u64,
        }
        
        impl HttpClient {
            pub fn new(base_url: String) -> Self {
                HttpClient {
                    base_url,
                    timeout: 30,
                }
            }
            
            pub fn get(&self, path: &str) -> Result<String, String> {
                // Use internal helper
                let full_url = self.build_url(path);
                
                // Use utility from parent module
                super::utils::log_request(&full_url);
                
                // Simulate HTTP request
                Ok(format!("Response from {}", full_url))
            }
            
            fn build_url(&self, path: &str) -> String {
                format!("{}/{}", self.base_url, path.trim_start_matches('/'))
            }
        }
    }
    
    pub mod server {
        pub struct HttpServer {
            port: u16,
        }
        
        impl HttpServer {
            pub fn new(port: u16) -> Self {
                HttpServer { port }
            }
            
            pub fn start(&self) {
                // Use utility from parent module
                super::utils::log_server_start(self.port);
                
                // Use client functionality (unusual but demonstrates paths)
                let client = super::client::HttpClient::new("http://localhost:8081".to_string());
                
                println!("Server starting on port {}", self.port);
            }
        }
    }
    
    mod utils {
        pub fn log_request(url: &str) {
            println!("[LOG] Making request to: {}", url);
        }
        
        pub fn log_server_start(port: u16) {
            println!("[LOG] Starting server on port: {}", port);
        }
    }
}

pub mod config {
    pub struct AppConfig {
        pub server_port: u16,
        pub client_base_url: String,
        debug_mode: bool, // Private field
    }
    
    impl AppConfig {
        pub fn new() -> Self {
            AppConfig {
                server_port: 8080,
                client_base_url: "https://api.example.com".to_string(),
                debug_mode: false,
            }
        }
        
        pub fn enable_debug(&mut self) {
            self.debug_mode = true;
        }
        
        pub fn is_debug(&self) -> bool {
            self.debug_mode
        }
    }
}

// File: src/main.rs
use my_lib::network::{client::HttpClient, server::HttpServer};
use my_lib::config::AppConfig;

fn main() {
    let config = AppConfig::new();
    
    // Create and use HTTP client
    let client = HttpClient::new(config.client_base_url.clone());
    match client.get("/users/1") {
        Ok(response) => println!("Client response: {}", response),
        Err(error) => println!("Client error: {}", error),
    }
    
    // Create and start HTTP server
    let server = HttpServer::new(config.server_port);
    server.start();
}
```

#### Path Strategy Examples
```rust
mod game {
    pub mod graphics {
        pub mod renderer {
            pub fn render() {
                // Access sibling module with relative path
                super::textures::load_texture("player.png");
                
                // Access cousin module with absolute path
                crate::game::audio::play_sound("render_complete.wav");
                
                // Access parent's parent with super::super
                super::super::input::get_mouse_position();
            }
        }
        
        pub mod textures {
            pub fn load_texture(name: &str) {
                println!("Loading texture: {}", name);
            }
        }
    }
    
    pub mod audio {
        pub fn play_sound(name: &str) {
            println!("Playing sound: {}", name);
        }
    }
    
    pub mod input {
        pub fn get_mouse_position() -> (i32, i32) {
            (100, 200)
        }
    }
}

// Usage with different path strategies
fn main() {
    // Absolute paths - always work regardless of current module
    crate::game::graphics::renderer::render();
    crate::game::audio::play_sound("background.mp3");
    
    // Relative paths - shorter but context-dependent
    game::graphics::textures::load_texture("background.jpg");
    game::input::get_mouse_position();
}
```

#### Multiple super Usage
```rust
mod level1 {
    pub fn level1_function() {
        println!("Level 1 function");
    }
    
    pub mod level2 {
        pub fn level2_function() {
            // Access parent
            super::level1_function();
        }
        
        pub mod level3 {
            pub fn level3_function() {
                // Access parent
                super::level2_function();
                
                // Access grandparent
                super::super::level1_function();
                
                // Access root level function
                crate::root_function();
            }
        }
    }
}

fn root_function() {
    println!("Root function");
}

fn main() {
    level1::level2::level3::level3_function();
}
```

#### Privacy and Path Interaction
```rust
mod outer {
    pub mod inner {
        pub fn public_function() {
            // Can access private function in same module
            private_function();
            
            // Can access parent's private items
            super::outer_private();
        }
        
        fn private_function() {
            println!("Inner private function");
        }
    }
    
    fn outer_private() {
        println!("Outer private function");
    }
    
    pub fn outer_public() {
        // Can access child's public items
        inner::public_function();
        
        // Cannot access child's private items
        // inner::private_function(); // Error!
    }
}

fn main() {
    // Can access public path
    outer::inner::public_function();
    outer::outer_public();
    
    // Cannot access private items
    // outer::inner::private_function(); // Error!
    // outer::outer_private(); // Error!
}
```

### Practical Applications
- Organizing large codebases with clear module boundaries
- Creating clean public APIs while hiding implementation details
- Building reusable libraries with intuitive access patterns
- Managing dependencies between different parts of an application
- Structuring projects for maintainability and discoverability

### Path Strategy Guidelines

#### When to Use Absolute Paths
- When the item location is unlikely to change
- When clarity is more important than brevity
- In public API examples and documentation
- When referring to items far away in the module tree

#### When to Use Relative Paths
- When items are likely to move together
- Within closely related modules
- When the relationship between modules is important
- For shorter, more readable code

### Integration with Previous Chapters
- Builds on module definitions from Chapter 7.2
- Uses package and crate structure from Chapter 7.1
- Applies privacy rules to determine accessible paths
- Enables importing and using items (covered in next chapters)

### Community Conventions and Idioms
- Prefer relative paths for closely related code
- Use absolute paths for stable, well-known items
- Keep module hierarchies reasonably shallow
- Group related functionality to minimize complex paths
- Use descriptive module names that make paths self-documenting

### Personal Notes
- Path resolution is compile-time, so invalid paths are caught early
- Understanding paths is crucial for navigating large Rust codebases
- The `super` keyword is particularly useful for sibling module access
- Absolute vs relative path choice affects code maintainability
- Privacy rules combined with paths provide powerful encapsulation

Official Chapter: https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html

---
*Completed: ✓*