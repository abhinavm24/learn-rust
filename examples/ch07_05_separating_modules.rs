//! # Chapter 7.5: Separating Modules into Different Files
//! 
//! This example demonstrates:
//! - How to split modules into separate files
//! - Module file structure and naming conventions
//! - Using mod.rs files for module organization
//! - Loading modules from external files
//! 
//! Note: This example shows the concepts, but in a real project,
//! you would actually separate the modules into different files.

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 7.5", "Separating Modules into Different Files");

    println!("=== File-based Module Organization ===");
    
    // In a real project, these would be in separate files:
    // src/front_of_house.rs
    // src/front_of_house/hosting.rs  
    // src/front_of_house/serving.rs
    
    restaurant::eat_at_restaurant();
    
    println!("\n=== Module File Structure Examples ===");
    demonstrate_file_structure();
}

// In a real application, this would be declared as:
// mod front_of_house;
// And the implementation would be in src/front_of_house.rs

pub mod front_of_house {
    // In front_of_house.rs, this would be:
    // pub mod hosting;
    // And the implementation would be in src/front_of_house/hosting.rs
    
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to waitlist (from separate file)");
        }
        
        pub fn seat_at_table() {
            println!("Seating at table (from separate file)");
        }
    }
    
    // In front_of_house.rs, this would be:
    // pub mod serving;
    // And the implementation would be in src/front_of_house/serving.rs
    
    pub mod serving {
        pub fn take_order() {
            println!("Taking order (from separate file)");
        }
        
        pub fn serve_order() {
            println!("Serving order (from separate file)");
        }
        
        pub fn take_payment() {
            println!("Taking payment (from separate file)");
        }
    }
}

mod restaurant {
    use crate::front_of_house;
    
    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();
        
        // Relative path
        front_of_house::hosting::seat_at_table();
        front_of_house::serving::take_order();
        front_of_house::serving::serve_order();
        front_of_house::serving::take_payment();
    }
}

fn demonstrate_file_structure() {
    println!("In a real Rust project, you would organize modules like this:");
    println!();
    println!("src/");
    println!("├── main.rs              // Main application entry point");
    println!("├── lib.rs               // Library crate root (if applicable)");
    println!("├── front_of_house.rs    // front_of_house module");
    println!("└── front_of_house/      // Alternative: directory for submodules");
    println!("    ├── mod.rs           // Module declaration file");
    println!("    ├── hosting.rs       // hosting submodule");
    println!("    └── serving.rs       // serving submodule");
    println!();
    
    println!("=== Two ways to organize submodules ===");
    println!();
    println!("Method 1: Single file per module");
    println!("src/front_of_house.rs contains:");
    println!("pub mod hosting;  // Loads src/front_of_house/hosting.rs");
    println!("pub mod serving;  // Loads src/front_of_house/serving.rs");
    println!();
    
    println!("Method 2: Directory with mod.rs");
    println!("src/front_of_house/mod.rs contains:");
    println!("pub mod hosting;  // Loads src/front_of_house/hosting.rs");
    println!("pub mod serving;  // Loads src/front_of_house/serving.rs");
    println!();
    
    demonstrate_file_examples();
}

fn demonstrate_file_examples() {
    println!("=== Example File Contents ===");
    println!();
    
    println!("// src/main.rs");
    println!("mod front_of_house;  // Declares the front_of_house module");
    println!();
    println!("fn main() {{");
    println!("    front_of_house::hosting::add_to_waitlist();");
    println!("}}");
    println!();
    
    println!("// src/front_of_house.rs");
    println!("pub mod hosting;  // Declares hosting submodule");
    println!("pub mod serving;  // Declares serving submodule");
    println!();
    
    println!("// src/front_of_house/hosting.rs");
    println!("pub fn add_to_waitlist() {{");
    println!("    println!(\"Adding to waitlist\");");
    println!("}}");
    println!();
    println!("pub fn seat_at_table() {{");
    println!("    println!(\"Seating at table\");");
    println!("}}");
    println!();
    
    println!("// src/front_of_house/serving.rs");
    println!("pub fn take_order() {{");
    println!("    println!(\"Taking order\");");
    println!("}}");
    println!();
    println!("pub fn serve_order() {{");
    println!("    println!(\"Serving order\");");
    println!("}}");
}

// Example of how you might organize a larger project
mod project_organization_example {
    pub fn demonstrate_large_project() {
        println!("=== Large Project Organization ===");
        println!();
        println!("src/");
        println!("├── main.rs");
        println!("├── lib.rs");
        println!("├── config/");
        println!("│   ├── mod.rs");
        println!("│   ├── database.rs");
        println!("│   └── server.rs");
        println!("├── models/");
        println!("│   ├── mod.rs");
        println!("│   ├── user.rs");
        println!("│   └── post.rs");
        println!("├── handlers/");
        println!("│   ├── mod.rs");
        println!("│   ├── auth.rs");
        println!("│   └── api.rs");
        println!("└── utils/");
        println!("    ├── mod.rs");
        println!("    ├── validation.rs");
        println!("    └── helpers.rs");
        println!();
        
        println!("Each mod.rs file would contain:");
        println!("pub mod database;  // Re-exports the submodules");
        println!("pub mod server;");
        println!();
        
        println!("And main.rs or lib.rs would contain:");
        println!("mod config;");
        println!("mod models;");
        println!("mod handlers;");
        println!("mod utils;");
    }
}

// Demonstrating privacy across files
mod privacy_across_files {
    pub fn demonstrate_privacy() {
        println!("=== Privacy Rules Across Files ===");
        println!();
        println!("Privacy rules work the same across files:");
        println!("- Items are private by default");
        println!("- Use 'pub' to make items public");
        println!("- Child modules can access parent private items");
        println!("- Parent modules cannot access child private items");
        println!();
        
        println!("Example:");
        println!("// src/network.rs");
        println!("fn connect() {{          // Private function");
        println!("    println!(\"Connecting...\");");
        println!("}}");
        println!();
        println!("pub fn start_server() {{ // Public function");
        println!("    connect();          // Can call private function in same module");
        println!("}}");
        println!();
        
        println!("// src/main.rs");
        println!("mod network;");
        println!();
        println!("fn main() {{");
        println!("    network::start_server(); // OK - public function");
        println!("    // network::connect();   // Error - private function");
        println!("}}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_separation_concepts() {
        // Test that our example modules work
        restaurant::eat_at_restaurant();
        project_organization_example::demonstrate_large_project();
        privacy_across_files::demonstrate_privacy();
    }

    #[test] 
    fn test_front_of_house_modules() {
        front_of_house::hosting::add_to_waitlist();
        front_of_house::hosting::seat_at_table();
        front_of_house::serving::take_order();
        front_of_house::serving::serve_order();
        front_of_house::serving::take_payment();
    }
}