# Chapter 15.1: Using Box<T> to Point to Data on the Heap

## Key Takeaways

### Box<T> Fundamentals
- **Heap Allocation**: Store data on heap instead of stack
- **Owned Pointer**: Box owns the data it points to
- **Single Ownership**: Only one Box can own the data
- **Automatic Cleanup**: Memory freed when Box goes out of scope

### When to Use Box<T>
- **Large Data**: Avoid stack overflow with large data structures
- **Unknown Size**: Types whose size can't be known at compile time
- **Trait Objects**: Store values that implement a trait
- **Recursive Types**: Enable recursive data structures

### Basic Box Usage
```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    
    // Box is automatically dereferenced
    let x = *b;  // Dereference to get the value
    println!("x = {}", x);
}
```

### Box for Large Data
```rust
// Without Box - might cause stack overflow
struct LargeStruct {
    data: [u8; 1000000],  // Large array on stack
}

// With Box - data goes on heap
struct LargeStruct {
    data: Box<[u8; 1000000]>,  // Data on heap
}

fn create_large_data() -> Box<LargeStruct> {
    Box::new(LargeStruct {
        data: Box::new([0; 1000000]),
    })
}
```

### Recursive Data Structures
```rust
// This won't compile - infinite size
/*
enum List {
    Cons(i32, List),
    Nil,
}
*/

// Box enables recursive types
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}
```

### Linked List Implementation
```rust
#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node { value, next: None }
    }
    
    fn append(&mut self, value: i32) {
        match self.next {
            Some(ref mut next_node) => next_node.append(value),
            None => self.next = Some(Box::new(Node::new(value))),
        }
    }
    
    fn prepend(self, value: i32) -> Node {
        Node {
            value,
            next: Some(Box::new(self)),
        }
    }
    
    fn len(&self) -> usize {
        match self.next {
            Some(ref next_node) => 1 + next_node.len(),
            None => 1,
        }
    }
}

fn main() {
    let mut head = Node::new(1);
    head.append(2);
    head.append(3);
    
    println!("List: {:?}", head);
    println!("Length: {}", head.len());
    
    let new_head = head.prepend(0);
    println!("New list: {:?}", new_head);
}
```

### Binary Tree with Box
```rust
#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
    
    fn insert(&mut self, value: i32) {
        if value < self.value {
            match self.left {
                Some(ref mut left_node) => left_node.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            }
        } else {
            match self.right {
                Some(ref mut right_node) => right_node.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            }
        }
    }
    
    fn search(&self, value: i32) -> bool {
        if value == self.value {
            true
        } else if value < self.value {
            match self.left {
                Some(ref left_node) => left_node.search(value),
                None => false,
            }
        } else {
            match self.right {
                Some(ref right_node) => right_node.search(value),
                None => false,
            }
        }
    }
}

fn main() {
    let mut root = TreeNode::new(10);
    root.insert(5);
    root.insert(15);
    root.insert(3);
    root.insert(7);
    
    println!("Tree: {:?}", root);
    println!("Search 7: {}", root.search(7));
    println!("Search 12: {}", root.search(12));
}
```

### Box and Trait Objects
```rust
trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle {}x{}", self.width, self.height);
    }
}

fn main() {
    // Store different types that implement Draw trait
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 20.0 }),
        Box::new(Circle { radius: 3.0 }),
    ];
    
    for shape in shapes {
        shape.draw();
    }
}
```

### Memory Management with Box
```rust
fn create_box() -> Box<String> {
    let s = String::from("Hello, heap!");
    Box::new(s)  // Move s into Box
}

fn main() {
    let boxed_string = create_box();
    println!("Boxed string: {}", boxed_string);
    
    // Box and its contents are dropped here
}
```

### Box vs References
```rust
fn main() {
    let x = 5;
    
    // Reference - points to stack data
    let y = &x;
    
    // Box - owns heap data
    let z = Box::new(x);
    
    println!("x: {}, y: {}, z: {}", x, y, z);
    
    // y is just a reference, x is still owned by main
    // z owns its data on the heap
}
```

### Converting Between Box and Other Types
```rust
fn main() {
    // Create Box
    let boxed = Box::new(String::from("Hello"));
    
    // Convert Box to owned value
    let owned = *boxed;  // Moves out of Box
    // boxed is no longer valid here
    
    // Create new Box
    let boxed2 = Box::new(owned);
    
    // Leak Box to get raw pointer (advanced use case)
    let raw_ptr: *mut String = Box::into_raw(boxed2);
    
    // Convert back to Box (must be done to avoid memory leak)
    unsafe {
        let boxed3 = Box::from_raw(raw_ptr);
        println!("Recovered: {}", boxed3);
    }
}
```

### Box in Function Parameters
```rust
fn process_boxed_data(data: Box<String>) {
    println!("Processing: {}", data);
    // data is dropped at end of function
}

fn process_borrowed_data(data: &str) {
    println!("Processing: {}", data);
    // No ownership transfer
}

fn main() {
    let boxed = Box::new(String::from("Hello"));
    
    // Pass by value - transfers ownership
    process_boxed_data(boxed);
    // boxed is no longer valid here
    
    let boxed2 = Box::new(String::from("World"));
    
    // Borrow the boxed data
    process_borrowed_data(&boxed2);
    // boxed2 is still valid here
    
    println!("Still have: {}", boxed2);
}
```

### Performance Considerations
- **Heap Allocation**: Slower than stack allocation
- **Indirection**: One extra pointer dereference
- **Memory Locality**: May impact cache performance
- **Use When Necessary**: Only when stack allocation isn't suitable

### Common Patterns
- **Recursive Data Structures**: Lists, trees, graphs
- **Large Objects**: Avoid stack overflow
- **Trait Objects**: Dynamic dispatch
- **Optional Ownership**: Combined with Option<Box<T>>

Official Chapter: https://doc.rust-lang.org/book/ch15-01-box.html

---
*Completed: ✓*