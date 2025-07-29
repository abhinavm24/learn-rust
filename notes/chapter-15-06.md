# Chapter 15.6: Reference Cycles Can Leak Memory

## Key Takeaways

### Reference Cycles Problem
- **Memory Leaks**: Circular references prevent automatic cleanup
- **Strong References**: Rc<T> creates strong references that keep data alive
- **Weak References**: Weak<T> breaks cycles by not affecting reference count
- **Manual Breaking**: Sometimes cycles must be broken manually

### Creating a Reference Cycle
```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);  // Create cycle: a -> b -> a
    }
    
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
```

### Using Weak<T> to Break Cycles
```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,      // Weak reference to parent
    children: RefCell<Vec<Rc<Node>>>, // Strong references to children
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    
    println!("leaf strong = {}, weak = {}", 
             Rc::strong_count(&leaf), 
             Rc::weak_count(&leaf));
    
    println!("branch strong = {}, weak = {}", 
             Rc::strong_count(&branch), 
             Rc::weak_count(&branch));
}
```

### Tree Structure with Weak References  
```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct TreeNode {
    value: String,
    parent: RefCell<Weak<TreeNode>>,
    children: RefCell<Vec<Rc<TreeNode>>>,
}

impl TreeNode {
    fn new(value: String) -> Rc<Self> {
        Rc::new(TreeNode {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }
    
    fn add_child(parent: &Rc<TreeNode>, child: Rc<TreeNode>) {
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        parent.children.borrow_mut().push(child);
    }
}

fn main() {
    let root = TreeNode::new("root".to_string());
    let child1 = TreeNode::new("child1".to_string());
    let child2 = TreeNode::new("child2".to_string());
    
    TreeNode::add_child(&root, child1.clone());
    TreeNode::add_child(&root, child2.clone());
    
    // No reference cycle - tree can be properly cleaned up
    println!("Root has {} children", root.children.borrow().len());
    
    if let Some(parent) = child1.parent.borrow().upgrade() {
        println!("Child1's parent is: {}", parent.value);
    }
}
```

Official Chapter: https://doc.rust-lang.org/book/ch15-06-reference-cycles.html

---
*Completed: ✓*