# Chapter 8.1: Storing Lists of Values with Vectors

## Key Takeaways

### Vector Fundamentals
- **Vector (Vec<T>)**: Dynamically-sized arrays that store multiple values of the same type
- **Homogeneous Storage**: All elements must be the same type
- **Dynamic Growth**: Can add and remove elements at runtime
- **Memory Layout**: Elements stored contiguously in memory

### Vector Characteristics
- **Heap Allocated**: Data stored on the heap, can grow and shrink
- **Owned Data**: Vectors own their elements
- **Generic Type**: `Vec<T>` where T is the element type
- **Zero-indexed**: Elements accessed starting from index 0

### Creation and Modification
- Create empty vectors with `Vec::new()`
- Create with initial values using `vec![]` macro
- Use `push()` to add elements
- Must be mutable to modify after creation

### Important Syntax and Operators

#### Vector Creation
```rust
let v: Vec<i32> = Vec::new();           // Empty vector
let v = vec![1, 2, 3];                  // Vector with initial values
```

#### Element Access
```rust
let v = vec![1, 2, 3, 4, 5];
let third: &i32 = &v[2];                // Direct indexing (panics if out of bounds)
let third: Option<&i32> = v.get(2);     // Safe access with Option
```

#### Modification
```rust
let mut v = Vec::new();
v.push(5);                              // Add element
```

### Programming Concepts Introduced
- **Dynamic Data Structures**: Collections that can grow and shrink
- **Homogeneous Collections**: All elements have the same type
- **Memory Management**: Automatic cleanup when vectors go out of scope
- **Safe Access Patterns**: Using Option to handle out-of-bounds access

### Code Examples and Patterns

#### Basic Vector Creation and Usage
```rust
fn main() {
    // Creating empty vector with type annotation
    let v: Vec<i32> = Vec::new();
    
    // Creating vector with initial values
    let v = vec![1, 2, 3];
    
    // Creating and modifying mutable vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    println!("{:?}", v); // [5, 6, 7, 8]
}
```

#### Element Access Methods
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // Method 1: Direct indexing (can panic)
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    
    // Method 2: Using get method (returns Option)
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    
    // Handling out-of-bounds access
    // let does_not_exist = &v[100];        // This would panic!
    let does_not_exist = v.get(100);        // This returns None
    
    match does_not_exist {
        Some(value) => println!("Value: {}", value),
        None => println!("Index out of bounds"),
    }
}
```

#### Borrowing Rules with Vectors
```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    
    let first = &v[0];  // Immutable borrow
    
    // v.push(6);       // Error! Cannot borrow as mutable while immutable borrow exists
    
    println!("The first element is: {first}");
    
    // After first is no longer used, we can mutate again
    v.push(6);
    println!("{:?}", v);
}

// Correct approach
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    
    {
        let first = &v[0];
        println!("The first element is: {first}");
    } // first goes out of scope here
    
    v.push(6); // Now we can mutate
    println!("{:?}", v);
}
```

#### Iterating Over Vectors
```rust
fn main() {
    let v = vec![100, 32, 57];
    
    // Immutable iteration
    for i in &v {
        println!("{i}");
    }
    
    // Mutable iteration
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;  // Dereference to modify the value
    }
    
    println!("{:?}", v); // [150, 82, 107]
}
```

#### Using Enums to Store Multiple Types
```rust
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Integer: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(s) => println!("Text: {}", s),
        }
    }
}
```

#### Real-World Example: Shopping Cart
```rust
#[derive(Debug)]
struct Item {
    name: String,
    price: f64,
    quantity: u32,
}

impl Item {
    fn new(name: String, price: f64, quantity: u32) -> Self {
        Item { name, price, quantity }
    }
    
    fn total_price(&self) -> f64 {
        self.price * self.quantity as f64
    }
}

fn main() {
    let mut cart: Vec<Item> = Vec::new();
    
    // Add items to cart
    cart.push(Item::new("Apple".to_string(), 0.50, 6));
    cart.push(Item::new("Bread".to_string(), 2.00, 1));
    cart.push(Item::new("Milk".to_string(), 3.50, 2));
    
    // Calculate total
    let mut total = 0.0;
    for item in &cart {
        println!("{}: ${:.2} x {} = ${:.2}", 
                 item.name, item.price, item.quantity, item.total_price());
        total += item.total_price();
    }
    
    println!("Total: ${:.2}", total);
    
    // Remove an item (by index)
    if cart.len() > 1 {
        let removed = cart.remove(1);
        println!("Removed: {}", removed.name);
    }
    
    println!("Final cart: {:?}", cart);
}
```

#### Vector Methods and Operations
```rust
fn main() {
    let mut numbers = vec![1, 2, 3];
    
    // Adding elements
    numbers.push(4);
    numbers.push(5);
    
    // Removing elements
    let last = numbers.pop();           // Returns Option<T>
    println!("Popped: {:?}", last);     // Some(5)
    
    // Inserting at specific position
    numbers.insert(1, 10);              // Insert 10 at index 1
    
    // Removing at specific position
    let removed = numbers.remove(1);    // Remove element at index 1
    println!("Removed: {}", removed);   // 10
    
    // Length and capacity
    println!("Length: {}", numbers.len());
    println!("Capacity: {}", numbers.capacity());
    
    // Check if empty
    if !numbers.is_empty() {
        println!("Vector is not empty");
    }
    
    // Clear all elements
    numbers.clear();
    println!("After clear - Length: {}", numbers.len());
}
```

#### Working with Vector Slices
```rust
fn print_slice(slice: &[i32]) {
    for item in slice {
        println!("{}", item);
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // Pass entire vector as slice
    print_slice(&v);
    
    // Pass part of vector as slice
    print_slice(&v[1..3]);  // Elements at index 1 and 2
    
    // Convert vector to slice
    let slice: &[i32] = &v;
    println!("Slice length: {}", slice.len());
}
```

#### Vector with Complex Types
```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    let mut people = Vec::new();
    
    people.push(Person::new("Alice", 30));
    people.push(Person::new("Bob", 25));
    people.push(Person::new("Charlie", 35));
    
    // Find person by name
    for person in &people {
        if person.name == "Bob" {
            println!("Found {}, age {}", person.name, person.age);
        }
    }
    
    // Filter people by age
    let adults: Vec<&Person> = people.iter()
        .filter(|person| person.age >= 30)
        .collect();
    
    println!("Adults: {:?}", adults);
}
```

#### Memory Management and Dropping
```rust
fn main() {
    {
        let v = vec![1, 2, 3, 4];
        // v is valid here
        println!("{:?}", v);
    } // v goes out of scope and is dropped here, along with its elements
    
    // v is no longer accessible here
}

fn demonstrate_ownership() {
    let v1 = vec![1, 2, 3];
    let v2 = v1;  // v1 is moved to v2
    
    // println!("{:?}", v1);  // Error! v1 no longer owns the data
    println!("{:?}", v2);     // OK
}
```

#### Vector Capacity Management
```rust
fn main() {
    let mut v = Vec::new();
    
    // Reserve capacity to avoid reallocations
    v.reserve(100);
    println!("Capacity after reserve: {}", v.capacity());
    
    // Add elements
    for i in 0..10 {
        v.push(i);
        println!("Length: {}, Capacity: {}", v.len(), v.capacity());
    }
    
    // Shrink to fit actual size
    v.shrink_to_fit();
    println!("After shrink - Length: {}, Capacity: {}", v.len(), v.capacity());
}
```

### Practical Applications
- Storing lists of user input or data from files
- Building dynamic data structures like stacks and queues
- Collecting results from computations
- Managing collections of objects in applications
- Implementing algorithms that need resizable arrays

### Common Vector Operations
- `push(item)` - Add element to end
- `pop()` - Remove and return last element
- `insert(index, item)` - Insert at specific position
- `remove(index)` - Remove element at position
- `len()` - Get number of elements
- `is_empty()` - Check if vector has no elements
- `clear()` - Remove all elements
- `get(index)` - Safe element access

### Integration with Previous Chapters
- Uses ownership rules for element management
- Elements can be structs and enums from previous chapters
- Iteration patterns apply match expressions
- Generic type parameter `T` allows storing any type

### Community Conventions and Idioms
- Prefer `vec![]` macro for vectors with initial values
- Use `get()` method for safe access when index might be invalid
- Use slices (`&[T]`) for function parameters when you don't need ownership
- Reserve capacity when you know approximate size
- Use `collect()` with iterators to build vectors from other collections

### Performance Considerations
- Vectors are efficient for sequential access
- Inserting/removing at end is O(1)
- Inserting/removing in middle is O(n)
- Accessing by index is O(1)
- Reallocations can be expensive, so reserve capacity when possible

### Personal Notes
- Vectors are one of the most commonly used collections in Rust
- Understanding borrowing rules is crucial when working with vector elements
- The enum pattern for storing different types is very powerful
- Vectors automatically manage memory, making them much safer than raw arrays
- The choice between indexing and `get()` depends on whether you want to panic or handle errors

Official Chapter: https://doc.rust-lang.org/book/ch08-01-vectors.html

---
*Completed: ✓*