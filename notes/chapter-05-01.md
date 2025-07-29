# Chapter 5.1: Defining and Instantiating Structs

## Key Takeaways

### Struct Fundamentals
- Custom data types that group related values together
- Similar to tuples but with named fields for clarity
- Each field can have a different type
- Create custom types that represent concepts in your domain

### Struct Definition Syntax
- Use `struct` keyword followed by name and field definitions
- Field names must be unique within the struct
- Fields are private by default (within the same module)
- Naming convention: PascalCase for struct names, snake_case for fields

### Struct Instantiation
- Create instances by specifying values for all fields
- Fields can be specified in any order
- Must provide values for all fields (no partial initialization)
- Can use field init shorthand when variable names match field names

### Important Syntax and Operators

#### Struct Definition
```rust
struct StructName {
    field1: Type1,
    field2: Type2,
    field3: Type3,
}
```

#### Struct Instantiation
```rust
let instance = StructName {
    field1: value1,
    field2: value2,
    field3: value3,
};
```

#### Field Access
- `instance.field_name` - Access field value
- `instance.field_name = new_value` - Modify field (if instance is mutable)

### Programming Concepts Introduced
- **Data Aggregation**: Grouping related data into single type
- **Named Fields**: Self-documenting field access vs tuple indices
- **Custom Types**: Creating domain-specific data structures
- **Field Init Shorthand**: Convenient syntax for common patterns

### Code Examples and Patterns

#### Basic Struct Definition and Usage
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    println!("User email: {}", user1.email);
}
```

#### Mutable Structs
```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    user1.email = String::from("anotheremail@example.com");
    user1.sign_in_count += 1;
}
```

#### Constructor Function Pattern
```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// With field init shorthand
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username,  // Same as username: username
        email,     // Same as email: email
        sign_in_count: 1,
    }
}
```

#### Struct Update Syntax
```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    // Create new instance using most fields from user1
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1  // Use remaining fields from user1
    };
    
    // Note: user1 is no longer valid if it contains non-Copy types
    // that were moved to user2 (like String fields)
}
```

#### Working with References in Structs
```rust
// This won't compile without lifetimes (covered later)
// struct User {
//     username: &str,  // Error: missing lifetime specifier
//     email: &str,
// }

// For now, use owned String types
struct User {
    username: String,
    email: String,
}
```

#### Multiple Struct Types
```rust
struct Color(i32, i32, i32);  // Tuple struct
struct Point(i32, i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}

struct Circle {
    radius: f64,
}

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    let circle = Circle {
        radius: 5.0,
    };
}
```

#### Struct with Different Field Types
```rust
struct Employee {
    id: u32,
    name: String,
    salary: f64,
    active: bool,
    department: String,
    start_date: String,  // In real code, use a proper date type
}

fn main() {
    let employee = Employee {
        id: 1001,
        name: String::from("Alice Johnson"),
        salary: 75000.0,
        active: true,
        department: String::from("Engineering"),
        start_date: String::from("2023-01-15"),
    };
    
    println!("Employee {}: ${}", employee.name, employee.salary);
}
```

### Practical Applications
- Modeling real-world entities (User, Product, Order)
- Configuration objects with named settings
- API response structures
- Database record representations
- Grouping related function parameters

### Memory Layout
- Structs are stored contiguously in memory
- Field order in memory matches definition order
- Compiler may add padding for alignment
- Owned fields are stored directly in the struct
- Reference fields store pointers (with lifetime requirements)

### Integration with Previous Chapters
- Uses ownership concepts for field values
- String fields demonstrate ownership transfer
- Builds on variable binding and mutability
- Functions can return struct instances

### Community Conventions and Idioms
- Use PascalCase for struct names: `UserAccount`, `HttpRequest`
- Use snake_case for field names: `user_id`, `created_at`
- Create constructor functions for complex initialization
- Use builder pattern for structs with many optional fields
- Group related structs in the same module

### Personal Notes
- Structs make code much more readable than tuples for complex data
- Field names are self-documenting and prevent field order mistakes
- The struct update syntax is very convenient for creating variations
- Understanding struct ownership is crucial for avoiding borrow checker issues
- Structs are the foundation for object-oriented-like patterns in Rust

Official Chapter: https://doc.rust-lang.org/book/ch05-01-defining-structs.html

---
*Completed: ✓*