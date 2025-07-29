# Chapter 13.1: Closures - Anonymous Functions that Capture Their Environment

## Key Takeaways

### Closure Fundamentals
- **Anonymous Functions**: Functions without names
- **Environment Capture**: Can capture variables from enclosing scope
- **Type Inference**: Compiler infers parameter and return types
- **Flexible Syntax**: Multiple syntax forms for different use cases

### Closure Syntax
```rust
// Full syntax
let closure = |param1: i32, param2: i32| -> i32 {
    param1 + param2
};

// Type inference
let closure = |x, y| x + y;

// Single expression
let closure = |x| x + 1;

// No parameters
let closure = || println!("Hello!");
```

### Capturing Environment
```rust
fn main() {
    let x = 4;
    
    // Closure captures x from environment
    let equal_to_x = |z| z == x;
    
    let y = 4;
    assert!(equal_to_x(y));
}
```

### Closure Traits
- **FnOnce**: Takes ownership of captured variables
- **FnMut**: Mutably borrows captured variables
- **Fn**: Immutably borrows captured variables

### Capturing Modes
```rust
fn main() {
    let mut x = vec![1, 2, 3];
    
    // Immutable borrow
    let print = || println!("{:?}", x);
    print();
    
    // Mutable borrow
    let mut modify = || x.push(4);
    modify();
    
    // Move ownership
    let consume = move || x;
    let owned = consume();
}
```

### Storing Closures
```rust
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

### Function vs Closure
```rust
// Function
fn add_one_fn(x: usize) -> usize {
    x + 1
}

// Closure
let add_one_closure = |x: usize| x + 1;

// Both implement Fn trait
fn do_twice<F>(f: F, arg: usize) -> usize
where
    F: Fn(usize) -> usize,
{
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one_fn, 5);      // Functions work
    let answer = do_twice(add_one_closure, 5); // Closures work
}
```

### Move Closures
```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    
    // Move ownership to closure
    let handle = thread::spawn(move || {
        println!("Vector: {:?}", v);
    });
    
    handle.join().unwrap();
    
    // v is no longer accessible here
}
```

### Real-World Example: Event Handlers
```rust
struct Button {
    label: String,
    on_click: Box<dyn Fn()>,
}

impl Button {
    fn new<F>(label: String, on_click: F) -> Button
    where
        F: Fn() + 'static,
    {
        Button {
            label,
            on_click: Box::new(on_click),
        }
    }
    
    fn click(&self) {
        (self.on_click)();
    }
}

fn main() {
    let name = "World".to_string();
    
    let button = Button::new(
        "Say Hello".to_string(),
        move || println!("Hello, {}!", name),
    );
    
    button.click();
}
```

Official Chapter: https://doc.rust-lang.org/book/ch13-01-closures.html

---
*Completed: ✓*