//! # Chapter 7.3: Paths for Referring to an Item in the Module Tree
//! 
//! This example demonstrates:
//! - Absolute paths starting with `crate`
//! - Relative paths starting from the current module
//! - Using `super` to refer to parent modules
//! - Path privacy rules and how they work

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 7.3", "Paths for Referring to an Item in the Module Tree");

    println!("=== Absolute vs Relative Paths ===");
    
    // Both of these call the same function, but use different path styles
    println!("\nUsing absolute path:");
    crate::front_of_house::hosting::add_to_waitlist();
    
    println!("\nUsing relative path:");
    front_of_house::hosting::add_to_waitlist();
    
    println!("\n=== Using super keyword ===");
    back_of_house::fix_incorrect_order();
    
    println!("\n=== Struct and Enum Privacy ===");
    demonstrate_struct_privacy();
    demonstrate_enum_privacy();
}

// Public module that can be accessed from outside
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding customer to waitlist");
        }

        pub fn seat_at_table() {
            println!("Seating customer at table");
        }
    }

    pub mod serving {
        fn take_order() {
            println!("Taking order");
        }

        pub fn serve_order() {
            println!("Serving order");
        }

        pub fn take_payment() {
            println!("Processing payment");
        }
    }
}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        // Using super to access the parent module's function
        super::deliver_order();
    }

    fn cook_order() {
        println!("Cooking the order");
    }

    // Demonstrating struct field privacy
    pub struct Breakfast {
        pub toast: String,           // Public field - can be accessed
        seasonal_fruit: String,      // Private field - cannot be accessed directly
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        // Getter method for private field
        pub fn get_seasonal_fruit(&self) -> &str {
            &self.seasonal_fruit
        }
    }

    // Public enum - all variants are automatically public
    pub enum Appetizer {
        Soup,
        Salad,
        BreadSticks,
    }

    pub fn process_order() {
        fix_incorrect_order(); // Call private function within same module
        cook_order();          // Call private function within same module
        super::deliver_order(); // Call function in parent module using super
    }
}

fn deliver_order() {
    println!("Delivering order to customer");
}

fn demonstrate_struct_privacy() {
    // Order a breakfast in the summer
    let mut meal = back_of_house::Breakfast::summer("Rye");
    
    // We can access and modify the public field
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    
    // We can access private field through a public method
    println!("Seasonal fruit: {}", meal.get_seasonal_fruit());
    
    // The next line won't compile - we can't access private fields directly
    // println!("Direct access: {}", meal.seasonal_fruit); // This would be an error!
}

fn demonstrate_enum_privacy() {
    // All enum variants are public when the enum is public
    let soup = back_of_house::Appetizer::Soup;
    let salad = back_of_house::Appetizer::Salad;
    let bread_sticks = back_of_house::Appetizer::BreadSticks;
    
    match soup {
        back_of_house::Appetizer::Soup => println!("Ordered soup"),
        back_of_house::Appetizer::Salad => println!("Ordered salad"),  
        back_of_house::Appetizer::BreadSticks => println!("Ordered breadsticks"),
    }
}

// Example of deeply nested modules to show complex paths
mod complex_paths {
    pub mod level1 {
        pub mod level2 {
            pub mod level3 {
                pub fn deep_function() {
                    println!("Called function at level 3");
                    
                    // Absolute path to sibling function
                    crate::complex_paths::level1::level2::sibling_function();
                    
                    // Relative path using super to go up levels
                    super::sibling_function();
                    super::super::level2_function();
                    crate::complex_paths::level1_function();
                }
            }
            
            pub fn sibling_function() {
                println!("Called sibling function at level 2");
            }
            
            pub fn level2_function() {
                println!("Called level2 function");
            }
        }
        
        pub fn level1_function() {
            println!("Called level1 function");
        }
        
        pub fn level2_function() {
            println!("Called level2 function");
        }
    }
    
    pub fn demonstrate_complex_paths() {
        println!("\n=== Complex Path Examples ===");
        
        // Absolute path to deeply nested function
        crate::complex_paths::level1::level2::level3::deep_function();
        
        // Relative path to deeply nested function
        level1::level2::level3::deep_function();
    }
}

// Module demonstrating privacy rules
mod privacy_examples {
    // Private function - only accessible within this module
    fn private_function() {
        println!("This is a private function");
    }
    
    // Public function that can call private functions
    pub fn public_function() {
        println!("This is a public function");
        private_function(); // Can call private function from same module
    }
    
    pub mod child_module {
        pub fn child_function() {
            // Can't call parent's private function directly
            // super::private_function(); // This would be an error!
            
            // But can call parent's public function
            super::public_function();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absolute_paths() {
        crate::front_of_house::hosting::add_to_waitlist();
    }

    #[test]
    fn test_relative_paths() {
        front_of_house::hosting::add_to_waitlist();
    }

    #[test]
    fn test_super_keyword() {
        back_of_house::fix_incorrect_order();
    }

    #[test]
    fn test_complex_paths() {
        complex_paths::demonstrate_complex_paths();
    }

    #[test]
    fn test_privacy() {
        privacy_examples::public_function();
        privacy_examples::child_module::child_function();
    }
}