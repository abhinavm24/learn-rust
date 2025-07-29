# Chapter 15: Smart Pointers

## Key Takeaways

### Smart Pointer Fundamentals
- **Smart Pointers**: Data structures that act like pointers with additional metadata
- **Ownership**: Smart pointers own the data they point to
- **Automatic Management**: Handle memory allocation and deallocation automatically
- **Reference Counting**: Some smart pointers enable multiple ownership

### Common Smart Pointers
- **Box<T>**: Heap allocation for single ownership
- **Rc<T>**: Reference counting for multiple ownership (single-threaded)
- **RefCell<T>**: Interior mutability with runtime borrow checking
- **Arc<T>**: Atomic reference counting for multiple ownership (multi-threaded)

### Box<T> - Heap Allocation
```rust
let b = Box::new(5);  // Allocate integer on heap
let list = Box::new(Node { value: 1, next: None });
```

### Use Cases for Box<T>
- **Large Data**: Move large data to heap to avoid stack overflow
- **Trait Objects**: Store types whose size is unknown at compile time
- **Recursive Types**: Enable recursive data structures like linked lists
- **Ownership Transfer**: Transfer ownership of heap data

### Rc<T> - Reference Counting
```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a);  // Increment reference count
let c = Rc::clone(&a);  // Multiple owners of same data
```

### RefCell<T> - Interior Mutability
```rust
use std::cell::RefCell;

let data = RefCell::new(5);
*data.borrow_mut() += 1;  // Runtime borrow checking
let value = *data.borrow();  // Immutable borrow
```

### Combining Smart Pointers
```rust
use std::rc::Rc;
use std::cell::RefCell;

let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));
let a = Rc::clone(&shared_data);
let b = Rc::clone(&shared_data);
```

### Key Concepts
- **Deref Trait**: Allows smart pointers to be treated like references
- **Drop Trait**: Customizes cleanup behavior when values go out of scope
- **Interior Mutability**: Mutate data even when there are immutable references
- **Reference Cycles**: Potential memory leaks with circular references

### When to Use Each Type
- **Box<T>**: Single ownership, heap allocation
- **Rc<T>**: Multiple ownership in single-threaded contexts
- **RefCell<T>**: Need mutability with immutable references
- **Arc<T>**: Multiple ownership in multi-threaded contexts

### Integration with Previous Chapters
- Builds on ownership concepts from Chapter 4
- Uses traits from Chapter 10
- Enables complex data structures
- Foundation for concurrent programming

### Practical Applications
- Tree and graph data structures
- Shared state in single-threaded applications
- Implementing observer patterns
- Complex object relationships

Official Chapter: https://doc.rust-lang.org/book/ch15-00-smart-pointers.html

---
*Completed: ✓*