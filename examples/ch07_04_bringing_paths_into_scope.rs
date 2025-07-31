//! # Chapter 7.4: Bringing Paths into Scope with the use Keyword
//! 
//! This example demonstrates:
//! - Using the `use` keyword to bring paths into scope
//! - Creating idiomatic `use` statements
//! - Using `as` to provide new names (aliases)
//! - Re-exporting names with `pub use`
//! - Using nested paths and the glob operator

use rust_book_examples::print_chapter_header;

// Example 1: Basic use statements
use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult; // Using 'as' to avoid name collision

// Example 2: Bringing parent modules into scope (idiomatic for functions)
use crate::front_of_house::hosting;

// Example 3: Bringing structs and enums into scope (idiomatic)
use std::collections::BTreeMap;

// Example 4: Nested paths to clean up multiple use statements
use std::cmp::Ordering;

// Example 5: Self keyword in nested paths
use std::io::{self, Write};

// Example 6: Glob operator (use sparingly)
// use std::collections::*; // Brings all public items into scope

fn main() {
    print_chapter_header("Chapter 7.4", "Bringing Paths into Scope with the use Keyword");

    println!("=== Basic use Examples ===");
    basic_use_examples();
    
    println!("\n=== Function vs Struct/Enum use Patterns ===");
    idiomatic_use_patterns();
    
    println!("\n=== Handling Name Collisions with 'as' ===");
    name_collision_examples();
    
    println!("\n=== Re-exporting with pub use ===");
    demonstrate_reexporting();
    
    println!("\n=== Nested Paths Examples ===");
    nested_path_examples();
}

fn basic_use_examples() {
    // Without use, we'd have to write: std::collections::HashMap::new()
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    
    println!("HashMap created with {} items", map.len());
    
    // Using the hosting module we brought into scope
    hosting::add_to_waitlist();
}

fn idiomatic_use_patterns() {
    // For functions, it's idiomatic to bring the parent module into scope
    // This makes it clear that the function isn't locally defined
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    
    // For structs, enums, and other items, it's idiomatic to specify the full path
    let mut tree_map = BTreeMap::new();
    tree_map.insert("first", 1);
    tree_map.insert("second", 2);
    
    println!("BTreeMap created with {} items", tree_map.len());
}

fn name_collision_examples() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // We can use both Result types because we aliased one
    let _fmt_result: Result = Ok(());
    let _io_result: IoResult<()> = Ok(());
    
    println!("Successfully handled both Result types using aliases");
    Ok(())
}

fn demonstrate_reexporting() {
    // Using items from our re_export module
    re_export::add_to_waitlist();
    re_export::PrimaryColor::Red;
}

fn nested_path_examples() {
    // We can use items from nested paths
    let _ordering = Ordering::Equal;
    
    // Using io that we brought in with nested path
    match io::stdout().flush() {
        Ok(_) => println!("Flushed stdout successfully"),
        Err(_) => println!("Failed to flush stdout"),
    }
}

// Module definitions for our examples
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
        }
        
        pub fn serve_order() {
            println!("Serving order");
        }
    }
}

// Example of re-exporting with pub use
pub mod re_export {
    // Re-export hosting functionality
    pub use crate::front_of_house::hosting::add_to_waitlist;
    
    // Define some items to re-export
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
    
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
    
    // Re-export with different name
    pub use PrimaryColor as Color;
}

// Example module showing different use patterns
mod use_patterns {
    // Bringing specific items into scope
    use std::collections::{HashMap, BTreeMap};
    use std::fmt::{self, Display, Formatter};
    
    pub struct CustomStruct {
        name: String,
        value: i32,
    }
    
    impl Display for CustomStruct {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "CustomStruct {{ name: {}, value: {} }}", self.name, self.value)
        }
    }
    
    pub fn demonstrate_patterns() {
        let mut hash_map = HashMap::new();
        hash_map.insert("test", 42);
        
        let mut btree_map = BTreeMap::new();
        btree_map.insert("test", 42);
        
        let custom = CustomStruct {
            name: String::from("example"),
            value: 100,
        };
        
        println!("Custom struct: {}", custom);
    }
}

// Module demonstrating glob imports (generally discouraged)
mod glob_example {
    // This brings all public items from collections into scope
    // Generally only used in tests or when creating a prelude
    use std::collections::*;
    
    pub fn demonstrate_glob() {
        let _map: HashMap<i32, i32> = HashMap::new();
        let _set: HashSet<i32> = HashSet::new();
        let _btree: BTreeMap<i32, String> = BTreeMap::new();
        
        println!("Created collections using glob import");
    }
}

// External crate usage examples
mod external_crate_example {
    // If we had external crates, we'd use them like this:
    // use rand::Rng;
    // use serde::{Serialize, Deserialize};
    
    pub fn demonstrate_external_usage() {
        println!("This would demonstrate external crate usage");
        // let mut rng = rand::thread_rng();
        // let n: u8 = rng.gen();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_use_examples() {
        let mut map = HashMap::new();
        map.insert("test", "value");
        assert_eq!(map.get("test"), Some(&"value"));
    }
    
    #[test]
    fn test_hosting_functions() {
        // These should not panic
        hosting::add_to_waitlist();
        hosting::seat_at_table();
    }
    
    #[test]
    fn test_reexport() {
        re_export::add_to_waitlist();
        let _color = re_export::PrimaryColor::Red;
        let _alias_color = re_export::Color::Blue;
    }
    
    #[test]
    fn test_patterns() {
        use_patterns::demonstrate_patterns();
        glob_example::demonstrate_glob();
    }
}