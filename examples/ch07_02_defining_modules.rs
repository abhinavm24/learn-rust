//! # Chapter 7.2: Defining Modules to Control Scope and Privacy
//! 
//! This example demonstrates:
//! - Defining modules with the `mod` keyword
//! - Module tree structure and privacy rules
//! - Public vs private items
//! - How to organize code using modules

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 7.2", "Defining Modules to Control Scope and Privacy");

    // Using items from our modules
    restaurant::eat_at_restaurant();
    
    println!("\n=== Module Privacy Examples ===");
    
    // This works - front_of_house module and hosting are public
    restaurant::front_of_house::hosting::add_to_waitlist();
    
    // This would not work - back_of_house is private
    // restaurant::back_of_house::fix_incorrect_order(); // Error!
    
    println!("\n=== Nested Module Examples ===");
    sound::instrument::clarinet::breathe_in();
}

// Restaurant module demonstrating nested modules and privacy
mod restaurant {
    // Public module - can be accessed from outside
    pub mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("Added to waitlist in hosting");
            }

            pub fn seat_at_table() {
                println!("Seated at table");
            }
        }

        pub mod serving {
            pub fn take_order() {
                println!("Taking order");
            }

            pub fn serve_order() {
                println!("Serving order");
            }

            pub fn take_payment() {
                println!("Taking payment");
            }
        }
    }

    // Private module - can only be accessed from within the restaurant module
    mod back_of_house {
        pub fn fix_incorrect_order() {
            cook_order();
            // We can call our parent module's function
            super::deliver_order();
        }

        fn cook_order() {
            println!("Cooking order in back of house");
        }

        // Public struct with mixed field visibility
        pub struct Breakfast {
            pub toast: String,        // Public field
            seasonal_fruit: String,   // Private field
        }

        impl Breakfast {
            // Associated function to create a Breakfast
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }

        // Public enum - all variants are automatically public
        pub enum Appetizer {
            Soup,
            Salad,
        }
    }

    fn deliver_order() {
        println!("Delivering order");
    }

    pub fn eat_at_restaurant() {
        println!("\n=== Restaurant Experience ===");
        
        // Absolute path
        crate::restaurant::front_of_house::hosting::add_to_waitlist();
        
        // Relative path
        front_of_house::hosting::seat_at_table();
        front_of_house::serving::take_order();
        
        // Order a breakfast in the summer
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");

        // We can create enum variants
        let _order1 = back_of_house::Appetizer::Soup;
        let _order2 = back_of_house::Appetizer::Salad;
        
        front_of_house::serving::serve_order();
        front_of_house::serving::take_payment();
    }
}

// Another module tree example demonstrating deeply nested modules
mod sound {
    pub mod instrument {
        pub mod woodwind {
            pub fn clarinet() {
                println!("Playing clarinet");
            }
        }
        
        pub mod clarinet {
            pub fn breathe_in() {
                println!("Breathing in for clarinet");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_access() {
        // Test that we can access public modules
        restaurant::front_of_house::hosting::add_to_waitlist();
        sound::instrument::clarinet::breathe_in();
    }
}