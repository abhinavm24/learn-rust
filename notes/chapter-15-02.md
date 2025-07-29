# Chapter 15.2: Treating Smart Pointers Like Regular References with the Deref Trait

## Key Takeaways

### Deref Trait Purpose
- **Dereference Operator**: Customize behavior of `*` operator
- **Smart Pointer Integration**: Make smart pointers act like references
- **Automatic Dereferencing**: Enable automatic conversion to references
- **Method Call Syntax**: Allow calling methods on pointed-to data

### Basic Deref Implementation
```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);  // Calls y.deref() then applies *
}
```

### Deref Coercion
```rust
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    
    // Deref coercion: &MyBox<String> -> &String -> &str
    hello(&m);
    
    // Without deref coercion, we'd need:
    // hello(&(*m)[..]);
}
```

### Deref Coercion Rules
- **&T to &U**: When T implements Deref<Target=U>
- **&mut T to &mut U**: When T implements DerefMut<Target=U>
- **&mut T to &U**: When T implements Deref<Target=U>

### Real-World Example: String Wrapper
```rust
use std::ops::Deref;

struct SafeString {
    content: String,
}

impl SafeString {
    fn new(s: &str) -> SafeString {
        SafeString {
            content: s.chars()
                .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                .collect(),
        }
    }
}

impl Deref for SafeString {
    type Target = String;
    
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

fn print_length(s: &str) {
    println!("Length: {}", s.len());
}

fn main() {
    let safe = SafeString::new("Hello, World! @#$");
    println!("Safe string: {}", &*safe);  // "Hello World "
    
    // Deref coercion allows this
    print_length(&safe);  // &SafeString -> &String -> &str
    
    // Can call String methods directly
    println!("Uppercase: {}", safe.to_uppercase());
}
```

### Complex Deref Chain
```rust
use std::ops::Deref;

struct Layer1<T>(T);
struct Layer2<T>(Layer1<T>);
struct Layer3<T>(Layer2<T>);

impl<T> Deref for Layer1<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> Deref for Layer2<T> {
    type Target = Layer1<T>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> Deref for Layer3<T> {
    type Target = Layer2<T>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

fn main() {
    let nested = Layer3(Layer2(Layer1(42)));
    
    // Deref chain: Layer3 -> Layer2 -> Layer1 -> i32
    println!("Value: {}", *nested);
    
    // Method call uses deref coercion
    let s = Layer3(Layer2(Layer1(String::from("Hello"))));
    println!("Length: {}", s.len());  // Calls String::len
}
```

Official Chapter: https://doc.rust-lang.org/book/ch15-02-deref.html

---
*Completed: ✓*