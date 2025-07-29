# Chapter 7.2: Defining Modules to Control Scope and Privacy

## Key Takeaways

### Module Fundamentals
- **Modules**: Organize code into logical units and control privacy
- **Privacy**: Items are private by default, public access must be explicit
- **Module Tree**: Hierarchical structure with crate as root
- **Scope Control**: Modules define boundaries for name resolution

### Privacy Rules
- All items (functions, methods, structs, enums, modules, constants) are **private by default**
- Use `pub` keyword to make items public
- Child modules can access parent items, but not vice versa
- Sibling modules cannot access each other's private items

### Module Declaration
- Use `mod` keyword to declare modules
- Modules can be nested inside other modules
- Can be defined inline or in separate files
- Module names follow snake_case convention

### Important Syntax and Operators

#### Module Declaration
```rust
mod module_name {
    // module contents
}
```

#### Public Items
```rust
pub mod module_name {
    pub fn function_name() {}
    pub struct StructName {}
}
```

#### Nested Modules
```rust
mod parent {
    mod child {
        fn private_function() {}
    }
}
```

### Programming Concepts Introduced
- **Encapsulation**: Hiding implementation details behind public interfaces
- **Namespace Management**: Organizing code to avoid naming conflicts
- **Access Control**: Determining what code can use what functionality
- **Code Organization**: Grouping related functionality together

### Code Examples and Patterns

#### Basic Module Structure
```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // This won't work - hosting module is private!
    // front_of_house::hosting::add_to_waitlist();
}
```

#### Making Modules and Functions Public
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {} // Still private
    }

    mod serving { // Still private module
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Now this works!
    front_of_house::hosting::add_to_waitlist();
    
    // This still won't work - serving module is private
    // front_of_house::serving::take_order();
}
```

#### Module Tree Structure
```rust
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

#### Nested Module Example
```rust
mod performance {
    pub mod audio {
        pub fn play_sound() {
            println!("Playing audio");
        }
        
        fn adjust_volume() {
            println!("Adjusting volume");
        }
        
        pub mod effects {
            pub fn add_reverb() {
                println!("Adding reverb effect");
            }
            
            pub fn add_echo() {
                // Can access parent module's public items
                super::play_sound();
                println!("Adding echo effect");
            }
        }
    }
    
    mod video {
        fn render_frame() {
            println!("Rendering video frame");
        }
    }
}

fn main() {
    performance::audio::play_sound();
    performance::audio::effects::add_reverb();
    performance::audio::effects::add_echo();
    
    // These won't work:
    // performance::audio::adjust_volume();  // Private function
    // performance::video::render_frame();   // Private module
}
```

#### Real-World Library Example
```rust
// Library structure for a web server
pub mod server {
    pub mod http {
        pub struct Request {
            method: String,
            path: String,
            headers: std::collections::HashMap<String, String>,
        }
        
        impl Request {
            pub fn new(method: String, path: String) -> Self {
                Request {
                    method,
                    path,
                    headers: std::collections::HashMap::new(),
                }
            }
            
            pub fn add_header(&mut self, key: String, value: String) {
                self.headers.insert(key, value);
            }
            
            // Private method - implementation detail
            fn validate(&self) -> bool {
                !self.method.is_empty() && !self.path.is_empty()
            }
        }
        
        pub struct Response {
            pub status_code: u16,
            pub body: String,
        }
        
        impl Response {
            pub fn new(status_code: u16, body: String) -> Self {
                Response { status_code, body }
            }
        }
    }
    
    pub mod routing {
        use super::http::{Request, Response};
        
        pub fn handle_request(request: Request) -> Response {
            match request.path.as_str() {
                "/" => Response::new(200, "Home page".to_string()),
                "/about" => Response::new(200, "About page".to_string()),
                _ => Response::new(404, "Not found".to_string()),
            }
        }
        
        // Private helper function
        fn parse_route(path: &str) -> Vec<&str> {
            path.split('/').collect()
        }
    }
    
    // Private internal module
    mod config {
        pub struct ServerConfig {
            pub port: u16,
            pub host: String,
        }
        
        impl ServerConfig {
            pub fn default() -> Self {
                ServerConfig {
                    port: 8080,
                    host: "localhost".to_string(),
                }
            }
        }
    }
}

// Usage from main.rs
use server::http::{Request, Response};
use server::routing::handle_request;

fn main() {
    let mut request = Request::new("GET".to_string(), "/".to_string());
    request.add_header("User-Agent".to_string(), "My Browser".to_string());
    
    let response = handle_request(request);
    println!("Status: {}, Body: {}", response.status_code, response.body);
}
```

#### Module with Constants and Types
```rust
pub mod graphics {
    pub const MAX_VERTICES: usize = 1000;
    pub const DEFAULT_COLOR: (u8, u8, u8) = (255, 255, 255);
    
    pub struct Point {
        pub x: f32,
        pub y: f32,
    }
    
    impl Point {
        pub fn new(x: f32, y: f32) -> Self {
            Point { x, y }
        }
        
        pub fn distance_from_origin(&self) -> f32 {
            (self.x * self.x + self.y * self.y).sqrt()
        }
    }
    
    pub enum Shape {
        Circle { center: Point, radius: f32 },
        Rectangle { top_left: Point, width: f32, height: f32 },
    }
    
    impl Shape {
        pub fn area(&self) -> f32 {
            match self {
                Shape::Circle { radius, .. } => std::f32::consts::PI * radius * radius,
                Shape::Rectangle { width, height, .. } => width * height,
            }
        }
    }
    
    // Private helper functions
    fn clamp(value: f32, min: f32, max: f32) -> f32 {
        if value < min { min } else if value > max { max } else { value }
    }
}

fn main() {
    use graphics::{Point, Shape, MAX_VERTICES};
    
    let point = Point::new(3.0, 4.0);
    println!("Distance from origin: {}", point.distance_from_origin());
    
    let circle = Shape::Circle {
        center: Point::new(0.0, 0.0),
        radius: 5.0,
    };
    
    println!("Circle area: {}", circle.area());
    println!("Max vertices: {}", MAX_VERTICES);
}
```

#### Privacy with Structs and Enums
```rust
mod shapes {
    pub struct Rectangle {
        pub width: f64,
        height: f64,  // Private field
    }
    
    impl Rectangle {
        pub fn new(width: f64, height: f64) -> Rectangle {
            Rectangle { width, height }
        }
        
        pub fn area(&self) -> f64 {
            self.width * self.height
        }
        
        pub fn get_height(&self) -> f64 {
            self.height
        }
        
        pub fn set_height(&mut self, height: f64) {
            self.height = height;
        }
    }
    
    pub enum Color {
        Red,
        Green,
        Blue,
        Custom { r: u8, g: u8, b: u8 },
    }
}

fn main() {
    let mut rect = shapes::Rectangle::new(10.0, 5.0);
    
    // Can access public field directly
    rect.width = 15.0;
    
    // Cannot access private field directly
    // rect.height = 8.0;  // Error!
    
    // Must use public methods for private fields
    rect.set_height(8.0);
    println!("Height: {}", rect.get_height());
    
    let color = shapes::Color::Custom { r: 255, g: 0, b: 0 };
}
```

#### Module Organization Patterns
```rust
// Game engine module structure
pub mod engine {
    pub mod graphics {
        pub mod renderer {
            pub fn render_frame() {}
            fn setup_buffers() {}  // Private implementation
        }
        
        pub mod shaders {
            pub fn load_shader(name: &str) -> Result<Shader, String> {
                // Implementation
                Ok(Shader { name: name.to_string() })
            }
            
            pub struct Shader {
                name: String,
            }
        }
    }
    
    pub mod audio {
        pub fn play_sound(sound_id: u32) {}
        pub fn set_volume(volume: f32) {}
        
        // Private audio implementation details
        mod mixer {
            pub fn mix_channels() {}
        }
    }
    
    pub mod input {
        pub enum Key {
            A, B, C, Space, Enter,
        }
        
        pub fn is_key_pressed(key: Key) -> bool {
            // Implementation
            false
        }
    }
}

// Clean public API
pub use engine::graphics::renderer::render_frame;
pub use engine::audio::{play_sound, set_volume};
pub use engine::input::{Key, is_key_pressed};
```

### Practical Applications
- Creating clean public APIs for libraries
- Organizing large applications into logical components
- Hiding implementation details from users
- Preventing accidental access to internal functionality
- Building maintainable and modular codebases

### Privacy Benefits
- **Encapsulation**: Hide complexity behind simple interfaces
- **Flexibility**: Change private implementation without breaking users
- **Safety**: Prevent misuse of internal functions
- **Documentation**: Public items clearly show intended API
- **Maintenance**: Smaller public surface area is easier to maintain

### Integration with Previous Chapters
- Uses structs and enums as module building blocks
- Applies functions and methods within module boundaries
- Extends package and crate organization from Chapter 7.1
- Provides foundation for path resolution in upcoming chapters

### Community Conventions and Idioms
- Keep public APIs small and focused
- Use descriptive module names that explain purpose
- Group related functionality together
- Make implementation details private by default
- Document public APIs comprehensively
- Use `pub(crate)` for items that should be public within the crate only

### Personal Notes
- Privacy by default encourages thoughtful API design
- Module structure should reflect the problem domain
- Public/private boundaries are enforced at compile time
- Good module organization makes code self-documenting
- Understanding privacy is crucial for library design

Official Chapter: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html

---
*Completed: ✓*