# Chapter 15.4: Rc<T>, the Reference Counted Smart Pointer

## Key Takeaways

### Rc<T> Purpose
- **Multiple Ownership**: Allow multiple owners of same data
- **Reference Counting**: Track number of references to data
- **Single-Threaded**: Only for single-threaded scenarios
- **Immutable References**: All references are immutable

### Basic Rc Usage
```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(5);
    let b = Rc::clone(&a);  // Increment reference count
    let c = Rc::clone(&a);  // Increment reference count
    
    println!("a: {}, b: {}, c: {}", a, b, c);
    println!("Reference count: {}", Rc::strong_count(&a));
}
```

### Rc with Data Structures
```rust
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

### Shared Data Example
```rust
use std::rc::Rc;

struct SharedData {
    id: u32,
    name: String,
}

struct Node {
    data: Rc<SharedData>,
    children: Vec<Node>,
}

fn main() {
    let shared = Rc::new(SharedData {
        id: 1,
        name: "Shared Resource".to_string(),
    });
    
    let node1 = Node {
        data: Rc::clone(&shared),
        children: vec![],
    };
    
    let node2 = Node {
        data: Rc::clone(&shared),
        children: vec![],
    };
    
    println!("Shared data reference count: {}", Rc::strong_count(&shared));
    println!("Data: {} - {}", shared.id, shared.name);
}
```

Official Chapter: https://doc.rust-lang.org/book/ch15-04-rc.html

---
*Completed: ✓*