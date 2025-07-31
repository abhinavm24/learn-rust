//! # Chapter 15.1: Using Box<T> to Point to Data on the Heap
//! 
//! This example demonstrates:
//! - What smart pointers are and how they differ from references
//! - Using Box<T> to store data on the heap
//! - Enabling recursive types with Box<T>
//! - The Deref trait and how it works
//! - Drop trait for cleanup
//! - Reference counting with Rc<T>
//! - Interior mutability with RefCell<T>

use rust_book_examples::print_chapter_header;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    print_chapter_header("Chapter 15.1", "Smart Pointers: Box, Rc, RefCell");

    println!("=== Box<T> Basics ===");
    box_basics();
    
    println!("\n=== Recursive Types with Box ===");
    recursive_types();
    
    println!("\n=== Custom Smart Pointer ===");
    custom_smart_pointer();
    
    println!("\n=== Reference Counting with Rc<T> ===");
    reference_counting();
    
    println!("\n=== Interior Mutability with RefCell<T> ===");
    interior_mutability();
    
    println!("\n=== Combining Rc<T> and RefCell<T> ===");
    combining_rc_refcell();
}

fn box_basics() {
    // Box<T> stores data on the heap instead of the stack
    let b = Box::new(5);
    println!("b = {}", b);
    
    // Box can store large amounts of data
    let large_array = Box::new([0; 1000]);
    println!("Large array length: {}", large_array.len());
    
    // Box with different types
    let boxed_string = Box::new(String::from("Hello, Box!"));
    println!("Boxed string: {}", boxed_string);
    
    let boxed_vector = Box::new(vec![1, 2, 3, 4, 5]);
    println!("Boxed vector: {:?}", boxed_vector);
    
    // Box automatically dereferences
    let x = 5;
    let y = Box::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y); // Dereference the Box
    
    println!("x = {}, y = {}", x, *y);
    
    // Box can be moved
    let original_box = Box::new(42);
    let moved_box = original_box;
    // println!("original_box: {}", original_box); // Would cause compile error
    println!("moved_box: {}", moved_box);
}

fn recursive_types() {
    // Without Box, this would be impossible due to infinite size
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    impl List {
        fn new() -> List {
            List::Nil
        }
        
        fn prepend(self, elem: i32) -> List {
            List::Cons(elem, Box::new(self))
        }
        
        fn len(&self) -> usize {
            match self {
                List::Cons(_, tail) => 1 + tail.len(),
                List::Nil => 0,
            }
        }
        
        fn stringify(&self) -> String {
            match self {
                List::Cons(head, tail) => {
                    format!("{}, {}", head, tail.stringify())
                }
                List::Nil => {
                    format!("Nil")
                }
            }
        }
    }
    
    // Create a list: 1 -> 2 -> 3 -> Nil
    let list = List::new()
        .prepend(3)
        .prepend(2)
        .prepend(1);
    
    println!("List: {}", list.stringify());
    println!("List length: {}", list.len());
    
    // Binary tree example
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
            if value <= self.value {
                match self.left {
                    Some(ref mut left) => left.insert(value),
                    None => self.left = Some(Box::new(TreeNode::new(value))),
                }
            } else {
                match self.right {
                    Some(ref mut right) => right.insert(value),
                    None => self.right = Some(Box::new(TreeNode::new(value))),
                }
            }
        }
        
        fn search(&self, value: i32) -> bool {
            if value == self.value {
                true
            } else if value < self.value {
                match self.left {
                    Some(ref left) => left.search(value),
                    None => false,
                }
            } else {
                match self.right {
                    Some(ref right) => right.search(value),
                    None => false,
                }
            }
        }
    }
    
    let mut tree = TreeNode::new(10);
    tree.insert(5);
    tree.insert(15);
    tree.insert(3);
    tree.insert(7);
    tree.insert(12);
    tree.insert(18);
    
    println!("Tree: {:?}", tree);
    println!("Search for 7: {}", tree.search(7));
    println!("Search for 20: {}", tree.search(20));
}

fn custom_smart_pointer() {
    // Implementing Deref trait for a custom smart pointer
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
    
    // Implementing Drop trait for cleanup
    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            println!("Dropping MyBox!");
        }
    }
    
    let x = 5;
    let y = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y); // Uses our Deref implementation
    
    println!("MyBox contains: {}", *y);
    
    // Deref coercion example
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }
    
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // &MyBox<String> -> &String -> &str through deref coercion
    
    // Drop trait in action
    {
        let _temp = MyBox::new("temporary");
        println!("Created temporary MyBox");
    } // Drop is called here automatically
    println!("Temporary MyBox was dropped");
    
    // Manual drop
    let manual_drop = MyBox::new("manual");
    println!("Before manual drop");
    drop(manual_drop); // Explicitly call drop
    println!("After manual drop");
}

fn reference_counting() {
    // Rc<T> allows multiple owners of the same data
    let a = Rc::new(String::from("hello"));
    println!("Reference count after creating a: {}", Rc::strong_count(&a));
    
    let b = Rc::clone(&a);
    println!("Reference count after cloning to b: {}", Rc::strong_count(&a));
    
    {
        let c = Rc::clone(&a);
        println!("Reference count after cloning to c: {}", Rc::strong_count(&a));
        println!("a: {}, b: {}, c: {}", a, b, c);
    } // c goes out of scope here
    
    println!("Reference count after c goes out of scope: {}", Rc::strong_count(&a));
    
    // Rc with custom types
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: Vec<Rc<Node>>,
    }
    
    impl Node {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(Node {
                value,
                children: Vec::new(),
            })
        }
        
        fn add_child(self: &Rc<Self>, child: Rc<Node>) -> Rc<Self> {
            let mut new_children = self.children.clone();
            new_children.push(child);
            Rc::new(Node {
                value: self.value,
                children: new_children,
            })
        }
    }
    
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);
    
    let root_with_children = root.add_child(child1).add_child(child2);
    println!("Tree with shared ownership: {:?}", root_with_children);
    
    // Multiple parents sharing the same child
    let shared_child = Node::new(10);
    let parent1 = Node::new(20);
    let parent2 = Node::new(30);
    
    let parent1_with_child = parent1.add_child(Rc::clone(&shared_child));
    let parent2_with_child = parent2.add_child(Rc::clone(&shared_child));
    
    println!("Parent 1: {:?}", parent1_with_child);
    println!("Parent 2: {:?}", parent2_with_child);
    println!("Shared child reference count: {}", Rc::strong_count(&shared_child));
}

fn interior_mutability() {
    // RefCell<T> allows mutable borrows checked at runtime
    let data = RefCell::new(5);
    
    println!("Initial value: {:?}", data);
    
    // Mutable borrow
    {
        let mut borrowed = data.borrow_mut();
        *borrowed += 10;
        println!("Modified value: {}", *borrowed);
    } // Mutable borrow ends here
    
    // Immutable borrow
    let borrowed = data.borrow();
    println!("Final value: {}", *borrowed);
    
    // RefCell with a more complex example
    #[derive(Debug)]
    struct Counter {
        value: RefCell<i32>,
    }
    
    impl Counter {
        fn new() -> Self {
            Counter {
                value: RefCell::new(0),
            }
        }
        
        fn increment(&self) {
            let mut val = self.value.borrow_mut();
            *val += 1;
        }
        
        fn get(&self) -> i32 {
            *self.value.borrow()
        }
        
        fn add(&self, amount: i32) {
            *self.value.borrow_mut() += amount;
        }
    }
    
    let counter = Counter::new();
    println!("Counter initial: {}", counter.get());
    
    counter.increment();
    counter.increment();
    counter.add(5);
    
    println!("Counter after operations: {}", counter.get());
    println!("Counter debug: {:?}", counter);
    
    // Mock object pattern with RefCell
    struct MockDatabase {
        queries: RefCell<Vec<String>>,
    }
    
    impl MockDatabase {
        fn new() -> Self {
            MockDatabase {
                queries: RefCell::new(Vec::new()),
            }
        }
        
        fn execute_query(&self, query: &str) -> String {
            self.queries.borrow_mut().push(query.to_string());
            format!("Executed: {}", query)
        }
        
        fn get_query_count(&self) -> usize {
            self.queries.borrow().len()
        }
        
        fn get_queries(&self) -> Vec<String> {
            self.queries.borrow().clone()
        }
    }
    
    let mock_db = MockDatabase::new();
    
    println!("Mock DB query count: {}", mock_db.get_query_count());
    
    mock_db.execute_query("SELECT * FROM users");
    mock_db.execute_query("INSERT INTO users VALUES (1, 'Alice')");
    mock_db.execute_query("UPDATE users SET name = 'Bob' WHERE id = 1");
    
    println!("Mock DB query count: {}", mock_db.get_query_count());
    println!("Mock DB queries: {:?}", mock_db.get_queries());
}

fn combining_rc_refcell() {
    // Combining Rc<T> and RefCell<T> for shared mutable data
    
    #[derive(Debug)]
    struct SharedCounter {
        value: Rc<RefCell<i32>>,
    }
    
    impl SharedCounter {
        fn new(initial: i32) -> Self {
            SharedCounter {
                value: Rc::new(RefCell::new(initial)),
            }
        }
        
        fn clone_counter(&self) -> Self {
            SharedCounter {
                value: Rc::clone(&self.value),
            }
        }
        
        fn increment(&self) {
            *self.value.borrow_mut() += 1;
        }
        
        fn get(&self) -> i32 {
            *self.value.borrow()
        }
        
        fn add(&self, amount: i32) {
            *self.value.borrow_mut() += amount;
        }
    }
    
    let counter1 = SharedCounter::new(0);
    let counter2 = counter1.clone_counter();
    let counter3 = counter1.clone_counter();
    
    println!("Initial counters: c1={}, c2={}, c3={}", 
             counter1.get(), counter2.get(), counter3.get());
    
    counter1.increment();
    counter2.add(5);
    counter3.increment();
    
    println!("After operations: c1={}, c2={}, c3={}", 
             counter1.get(), counter2.get(), counter3.get());
    
    // Tree with shared mutable nodes
    #[derive(Debug)]
    struct MutableNode {
        value: i32,
        children: RefCell<Vec<Rc<MutableNode>>>,
    }
    
    impl MutableNode {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(MutableNode {
                value,
                children: RefCell::new(Vec::new()),
            })
        }
        
        fn add_child(self: &Rc<Self>, child: Rc<MutableNode>) {
            self.children.borrow_mut().push(child);
        }
        
        fn child_count(&self) -> usize {
            self.children.borrow().len()
        }
        
        fn get_children(&self) -> Vec<Rc<MutableNode>> {
            self.children.borrow().clone()
        }
    }
    
    let root = MutableNode::new(1);
    let child1 = MutableNode::new(2);
    let child2 = MutableNode::new(3);
    let child3 = MutableNode::new(4);
    
    root.add_child(Rc::clone(&child1));
    root.add_child(Rc::clone(&child2));
    
    // child1 can also have its own children
    child1.add_child(child3);
    
    println!("Root children count: {}", root.child_count());
    println!("Child1 children count: {}", child1.child_count());
    
    // Shared ownership example
    let shared_node = MutableNode::new(100);
    let parent1 = MutableNode::new(200);
    let parent2 = MutableNode::new(300);
    
    parent1.add_child(Rc::clone(&shared_node));
    parent2.add_child(Rc::clone(&shared_node));
    
    println!("Parent1 children: {}", parent1.child_count());
    println!("Parent2 children: {}", parent2.child_count());
    println!("Shared node reference count: {}", Rc::strong_count(&shared_node));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_basic() {
        let b = Box::new(5);
        assert_eq!(*b, 5);
    }

    #[test]
    fn test_recursive_list() {
        #[derive(Debug)]
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
        
        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
        
        match list {
            List::Cons(val, _) => assert_eq!(val, 1),
            List::Nil => panic!("Expected Cons, got Nil"),
        }
    }

    #[test]
    fn test_rc_reference_counting() {
        let a = Rc::new(String::from("test"));
        assert_eq!(Rc::strong_count(&a), 1);
        
        let b = Rc::clone(&a);
        assert_eq!(Rc::strong_count(&a), 2);
        assert_eq!(Rc::strong_count(&b), 2);
        
        drop(b);
        assert_eq!(Rc::strong_count(&a), 1);
    }

    #[test]
    fn test_refcell_interior_mutability() {
        let data = RefCell::new(5);
        
        {
            let mut borrowed = data.borrow_mut();
            *borrowed = 10;
        }
        
        assert_eq!(*data.borrow(), 10);
    }

    #[test]
    fn test_rc_refcell_combination() {
        let shared_data = Rc::new(RefCell::new(0));
        let data1 = Rc::clone(&shared_data);
        let data2 = Rc::clone(&shared_data);
        
        *data1.borrow_mut() += 1;
        *data2.borrow_mut() += 2;
        
        assert_eq!(*shared_data.borrow(), 3);
    }

    #[test]
    #[should_panic]
    fn test_refcell_runtime_panic() {
        let data = RefCell::new(5);
        
        let _borrow1 = data.borrow_mut();
        let _borrow2 = data.borrow_mut(); // This should panic at runtime
    }
}