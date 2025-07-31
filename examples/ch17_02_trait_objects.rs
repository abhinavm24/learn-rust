//! Chapter 17.2: Using Trait Objects That Allow for Values of Different Types
//! 
//! This example demonstrates:
//! - Trait objects for runtime polymorphism
//! - Dynamic dispatch vs static dispatch
//! - Object safety rules
//! - Creating heterogeneous collections
//! - GUI component system using trait objects

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 17.2", "Using Trait Objects That Allow for Values of Different Types");
    
    println!("=== Basic Trait Objects ===");
    basic_trait_objects();
    
    println!("\n=== GUI Component System ===");
    gui_component_system();
    
    println!("\n=== Dynamic vs Static Dispatch ===");
    dispatch_comparison();
    
    println!("\n=== Object Safety ===");
    object_safety_examples();
    
    println!("\n=== Advanced Trait Object Patterns ===");
    advanced_patterns();
}

trait Draw {
    fn draw(&self);
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle {}x{}", self.width, self.height);
    }
}

#[derive(Debug)]
struct Triangle {
    base: f64,
    height: f64,
}

impl Draw for Triangle {
    fn draw(&self) {
        println!("Drawing a triangle with base {} and height {}", self.base, self.height);
    }
}

fn basic_trait_objects() {
    // Vector of trait objects - different types that implement Draw
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 20.0 }),
        Box::new(Triangle { base: 15.0, height: 8.0 }),
    ];
    
    // Iterate and call methods dynamically - runtime dispatch
    for shape in shapes {
        shape.draw();
    }
    
    // Using references to trait objects
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle { width: 4.0, height: 6.0 };
    
    draw_shape(&circle);
    draw_shape(&rectangle);
}

fn draw_shape(shape: &dyn Draw) {
    shape.draw();
}

// GUI Component System
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            components: Vec::new(),
        }
    }
    
    pub fn add_component(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }
    
    pub fn run(&self) {
        println!("Rendering screen with {} components:", self.components.len());
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button: '{}' ({}x{})", self.label, self.width, self.height);
    }
}

pub struct TextField {
    pub width: u32,
    pub height: u32,
    pub placeholder: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("Drawing text field: '{}' ({}x{})", self.placeholder, self.width, self.height);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing select box ({}x{}) with {} options", 
                self.width, self.height, self.options.len());
    }
}

fn gui_component_system() {
    let mut screen = Screen::new();
    
    screen.add_component(Box::new(Button {
        width: 75,
        height: 10,
        label: String::from("OK"),
    }));
    
    screen.add_component(Box::new(TextField {
        width: 200,
        height: 20,
        placeholder: String::from("Enter your name"),
    }));
    
    screen.add_component(Box::new(SelectBox {
        width: 150,
        height: 25,
        options: vec![
            String::from("Option 1"),
            String::from("Option 2"),
            String::from("Option 3"),
        ],
    }));
    
    screen.run();
}

// Dynamic vs Static Dispatch comparison
trait Animal {
    fn make_sound(&self);
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("{} says: Woof!", self.name);
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("{} says: Meow!", self.name);
    }
}

// Static dispatch - compile-time polymorphism
fn make_animals_sound_static<T: Animal>(animals: Vec<T>) {
    println!("Static dispatch:");
    for animal in animals {
        animal.make_sound(); // Method call resolved at compile time
    }
}

// Dynamic dispatch - runtime polymorphism
fn make_animals_sound_dynamic(animals: Vec<Box<dyn Animal>>) {
    println!("Dynamic dispatch:");
    for animal in animals {
        animal.make_sound(); // Method call resolved at runtime via vtable
    }
}

fn dispatch_comparison() {
    // Static dispatch - all animals must be the same type
    let dogs = vec![
        Dog { name: String::from("Rex") },
        Dog { name: String::from("Buddy") },
    ];
    make_animals_sound_static(dogs);
    
    // Dynamic dispatch - animals can be different types
    let mixed_animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: String::from("Max") }),
        Box::new(Cat { name: String::from("Whiskers") }),
        Box::new(Dog { name: String::from("Bella") }),
    ];
    make_animals_sound_dynamic(mixed_animals);
}

// Object Safety Examples
trait ObjectSafe {
    fn draw(&self);
    fn area(&self) -> f64;
}

// This trait is NOT object-safe
trait NotObjectSafe {
    fn clone(&self) -> Self; // Returns Self - not object-safe
    fn compare<T>(&self, other: &T) -> bool; // Generic method - not object-safe
}

struct SafeCircle {
    radius: f64,
}

impl ObjectSafe for SafeCircle {
    fn draw(&self) {
        println!("Drawing safe circle with radius {}", self.radius);
    }
    
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn object_safety_examples() {
    println!("Object-safe trait usage:");
    
    let circle = SafeCircle { radius: 4.0 };
    let drawable: &dyn ObjectSafe = &circle;
    
    drawable.draw();
    println!("Area: {:.2}", drawable.area());
    
    // This would work - object-safe trait can be made into trait object
    let objects: Vec<Box<dyn ObjectSafe>> = vec![
        Box::new(SafeCircle { radius: 2.0 }),
        Box::new(SafeCircle { radius: 5.0 }),
    ];
    
    for obj in objects {
        obj.draw();
    }
}

// Advanced patterns
trait Drawable {
    fn draw(&self);
}

trait Clickable {
    fn click(&self);
}

struct InteractiveButton {
    label: String,
}

impl Drawable for InteractiveButton {
    fn draw(&self) {
        println!("Drawing interactive button: {}", self.label);
    }
}

impl Clickable for InteractiveButton {
    fn click(&self) {
        println!("Button '{}' was clicked!", self.label);
    }
}

// Multiple trait bounds with a helper trait
trait Interactive: Drawable + Clickable {}

impl<T: Drawable + Clickable> Interactive for T {}

fn handle_interactive_component<T: Interactive>(component: &T) {
    component.draw();
    component.click();
}

use std::rc::Rc;
use std::sync::Arc;

fn advanced_patterns() {
    println!("Multiple trait bounds:");
    let button = InteractiveButton {
        label: String::from("Submit"),
    };
    handle_interactive_component(&button);
    
    println!("\nDifferent smart pointer types:");
    
    // Box<dyn Trait> - owned trait object
    let boxed_drawable: Box<dyn Drawable> = Box::new(InteractiveButton {
        label: String::from("Boxed Button"),
    });
    boxed_drawable.draw();
    
    // Rc<dyn Trait> - reference counted for single-threaded sharing
    let rc_drawable: Rc<dyn Drawable> = Rc::new(InteractiveButton {
        label: String::from("RC Button"),
    });
    let rc_clone = Rc::clone(&rc_drawable);
    rc_drawable.draw();
    rc_clone.draw();
    
    // Arc<dyn Trait> - atomic reference counted for multi-threaded sharing
    let arc_drawable: Arc<dyn Drawable> = Arc::new(InteractiveButton {
        label: String::from("Arc Button"),
    });
    let arc_clone = Arc::clone(&arc_drawable);
    arc_drawable.draw();
    arc_clone.draw();
    
    println!("\nTrait objects with lifetimes:");
    trait_objects_with_lifetimes();
}

fn trait_objects_with_lifetimes() {
    trait Process {
        fn process(&self, data: &str) -> String;
    }
    
    struct Processor {
        prefix: String,
    }
    
    impl Process for Processor {
        fn process(&self, data: &str) -> String {
            format!("{}: {}", self.prefix, data)
        }
    }
    
    fn process_data<'a>(processor: &'a dyn Process, data: &str) -> String {
        processor.process(data)
    }
    
    let processor = Processor {
        prefix: String::from("LOG"),
    };
    
    let result = process_data(&processor, "Hello World");
    println!("Processed: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_objects() {
        let shapes: Vec<Box<dyn Draw>> = vec![
            Box::new(Circle { radius: 1.0 }),
            Box::new(Rectangle { width: 2.0, height: 3.0 }),
        ];
        
        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_gui_system() {
        let mut screen = Screen::new();
        screen.add_component(Box::new(Button {
            width: 50,
            height: 20,
            label: String::from("Test"),
        }));
        
        assert_eq!(screen.components.len(), 1);
    }

    #[test]
    fn test_object_safety() {
        let circle = SafeCircle { radius: 3.0 };
        let area = circle.area();
        assert!((area - 28.274).abs() < 0.01);
    }
}