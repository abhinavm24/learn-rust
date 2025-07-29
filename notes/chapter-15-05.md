# Chapter 15.5: RefCell<T> and the Interior Mutability Pattern

## Key Takeaways

### Interior Mutability
- **Runtime Borrowing**: Borrow checking at runtime instead of compile time
- **Immutable Outside**: Mutate data through immutable reference
- **Single-Threaded**: RefCell<T> is not thread-safe
- **Runtime Panics**: Borrowing rule violations cause runtime panics

### RefCell<T> vs Box<T>
- **Box<T>**: Compile-time borrow checking, single owner
- **RefCell<T>**: Runtime borrow checking, interior mutability
- **Rc<RefCell<T>>**: Multiple owners with mutable data

### Basic RefCell Usage
```rust
use std::cell::RefCell;

fn main() {
    let value = RefCell::new(5);
    
    // Borrow immutably
    {
        let borrowed = value.borrow();
        println!("Value: {}", *borrowed);
    } // Borrow released
    
    // Borrow mutably
    {
        let mut borrowed = value.borrow_mut();
        *borrowed += 10;
    } // Mutable borrow released
    
    println!("New value: {}", *value.borrow());
}
```

### Mock Object Pattern
```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        
        let percentage_of_max = self.value as f64 / self.max as f64;
        
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }
    
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        
        limit_tracker.set_value(80);
        
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

### Combining Rc<T> and RefCell<T>
```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(5));
    
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    
    *value.borrow_mut() += 10;
    
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

Official Chapter: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html

---
*Completed: ✓*