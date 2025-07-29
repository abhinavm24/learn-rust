# Chapter 17.2: Using Trait Objects That Allow for Values of Different Types

## Key Takeaways
- **Trait Objects**: Enable runtime polymorphism by allowing different types that implement the same trait
- **Dynamic Dispatch**: Method calls are resolved at runtime rather than compile time
- **Type Erasure**: Concrete types are "erased" and only trait methods are available
- **Object Safety**: Not all traits can be made into trait objects due to specific rules

## Understanding Trait Objects

### Basic Trait Object Syntax
```rust
// Trait definition
pub trait Draw {
    fn draw(&self);
}

// Different types implementing the trait
pub struct Circle {
    radius: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle {}x{}", self.width, self.height);
    }
}
```

### Using Trait Objects
```rust
// Vector of trait objects
let shapes: Vec<Box<dyn Draw>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Rectangle { width: 10.0, height: 20.0 }),
];

// Iterate and call methods dynamically
for shape in shapes {
    shape.draw();  // Runtime dispatch
}
```

## GUI Component Example

### Screen and Component Traits
```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

### Different Component Types
```rust
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button: {} ({}x{})", self.label, self.width, self.height);
    }
}

pub struct TextField {
    pub width: u32,
    pub height: u32,
    pub placeholder: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("Drawing text field: {} ({}x{})", self.placeholder, self.width, self.height);
    }
}
```

### Using the GUI System
```rust
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 75,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(TextField {
                width: 200,
                height: 20,
                placeholder: String::from("Enter text here"),
            }),
        ],
    };
    
    screen.run();
}
```

## Trait Object Syntax Variations

### Different Ways to Create Trait Objects
```rust
// Box<dyn Trait>
let boxed: Box<dyn Draw> = Box::new(Circle { radius: 3.0 });

// &dyn Trait
fn draw_shape(shape: &dyn Draw) {
    shape.draw();
}

// Arc<dyn Trait> for shared ownership
use std::sync::Arc;
let shared: Arc<dyn Draw> = Arc::new(Rectangle { width: 5.0, height: 8.0 });

// Rc<dyn Trait> for single-threaded shared ownership
use std::rc::Rc;
let counted: Rc<dyn Draw> = Rc::new(Circle { radius: 2.0 });
```

## Object Safety Rules

### Object-Safe Traits
A trait is object-safe if:
1. The trait doesn't require `Self: Sized`
2. All methods are object-safe

### Object-Safe Methods
Methods are object-safe if:
1. The return type isn't `Self`
2. There are no generic type parameters

```rust
// Object-safe trait
pub trait Draw {
    fn draw(&self);                    // ✓ Object-safe
    fn area(&self) -> f64;            // ✓ Object-safe
}

// Not object-safe trait
pub trait Clone {
    fn clone(&self) -> Self;           // ✗ Returns Self
}

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;  // ✗ Associated types can be problematic
}
```

## Dynamic vs Static Dispatch

### Static Dispatch (Generics)
```rust
// Compile-time polymorphism
fn draw_shapes<T: Draw>(shapes: Vec<T>) {
    for shape in shapes {
        shape.draw();  // Method call resolved at compile time
    }
}

// Zero runtime cost, but creates separate code for each type
```

### Dynamic Dispatch (Trait Objects)
```rust
// Runtime polymorphism
fn draw_shapes(shapes: Vec<Box<dyn Draw>>) {
    for shape in shapes {
        shape.draw();  // Method call resolved at runtime via vtable
    }
}

// Slight runtime cost, but allows heterogeneous collections
```

## Advanced Trait Object Patterns

### Trait Objects with Multiple Traits
```rust
// Combining multiple traits
trait Draw {
    fn draw(&self);
}

trait Clickable {
    fn click(&self);
}

// Trait object with multiple bounds
fn handle_component(component: &(dyn Draw + Clickable)) {
    component.draw();
    component.click();
}

// Alternative syntax
fn handle_component_alt(component: &dyn Draw + &dyn Clickable) {
    component.draw();
    component.click();
}
```

### Trait Objects with Lifetimes
```rust
trait Draw {
    fn draw(&self);
}

// Trait object with lifetime parameter
fn process_drawable<'a>(drawable: &'a dyn Draw) {
    drawable.draw();
}

// Boxed trait object with lifetime
struct Container<'a> {
    drawable: Box<dyn Draw + 'a>,
}
```

## Performance Considerations

### Vtable Overhead
```rust
// Trait objects use virtual method tables (vtables)
// Small runtime cost for method dispatch
// Memory overhead for storing vtable pointer

pub trait Animal {
    fn make_sound(&self);
    fn move_around(&self);
}

// Each trait object stores:
// 1. Pointer to data
// 2. Pointer to vtable (method implementations)
```

### When to Use Trait Objects vs Generics
- **Use Trait Objects When**:
  - Need heterogeneous collections
  - Type is determined at runtime
  - Want smaller binary size
  
- **Use Generics When**:
  - Know types at compile time
  - Need maximum performance
  - Want monomorphization benefits

## Error Handling with Trait Objects

### Trait Objects with Results
```rust
trait Processor {
    fn process(&self, input: &str) -> Result<String, Box<dyn std::error::Error>>;
}

struct JsonProcessor;
struct XmlProcessor;

impl Processor for JsonProcessor {
    fn process(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        // JSON processing logic
        Ok(format!("JSON: {}", input))
    }
}

impl Processor for XmlProcessor {
    fn process(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        // XML processing logic
        Ok(format!("XML: {}", input))
    }
}

fn process_data(processors: Vec<Box<dyn Processor>>, data: &str) {
    for processor in processors {
        match processor.process(data) {
            Ok(result) => println!("Success: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
}
```

## Integration with Previous Concepts
- **Ownership**: Trait objects often use `Box<dyn Trait>` for owned values
- **Borrowing**: `&dyn Trait` for borrowed trait objects
- **Lifetimes**: Trait objects can have lifetime parameters
- **Error Handling**: Common to return `Box<dyn Error>` for dynamic error types

## Best Practices
1. **Use When Appropriate**: Choose trait objects when you need runtime polymorphism
2. **Consider Performance**: Understand the vtable overhead
3. **Design Object-Safe Traits**: Ensure traits can be made into objects
4. **Prefer `&dyn` for Borrowed**: Use references when possible
5. **Handle Errors Gracefully**: Use `Result` types with trait object methods

This pattern enables powerful runtime polymorphism while maintaining Rust's safety guarantees.