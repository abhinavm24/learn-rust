use rust_book_examples::print_chapter_header;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    print_chapter_header("Chapter 15.6", "Reference Cycles Can Leak Memory");
    
    println!("🔄 Reference Cycles and Memory Leaks");
    println!();
    
    demonstrate_reference_cycle_problem();
    demonstrate_weak_references();
    demonstrate_tree_structure();
    demonstrate_preventing_cycles();
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }
    
    fn add_child(parent: &Rc<Node>, child: Rc<Node>) {
        child.parent.borrow_mut().clone_from(&Rc::downgrade(parent));
        parent.children.borrow_mut().push(child);
    }
}

fn demonstrate_reference_cycle_problem() {
    println!("⚠️ The Reference Cycle Problem:");
    println!();
    
    println!("When two or more Rc<T> instances reference each other,");
    println!("they create a cycle that prevents memory cleanup.");
    println!();
    
    // Example of a problematic cycle (simplified)
    #[derive(Debug)]
    struct CyclicNode {
        value: i32,
        next: RefCell<Option<Rc<CyclicNode>>>,
    }
    
    impl CyclicNode {
        fn new(value: i32) -> Rc<CyclicNode> {
            Rc::new(CyclicNode {
                value,
                next: RefCell::new(None),
            })
        }
    }
    
    println!("Creating a potential cycle:");
    let node_a = CyclicNode::new(1);
    let node_b = CyclicNode::new(2);
    
    println!("Node A reference count: {}", Rc::strong_count(&node_a));
    println!("Node B reference count: {}", Rc::strong_count(&node_b));
    
    // Create references between nodes
    *node_a.next.borrow_mut() = Some(Rc::clone(&node_b));
    *node_b.next.borrow_mut() = Some(Rc::clone(&node_a));
    
    println!("After creating cycle:");
    println!("Node A reference count: {}", Rc::strong_count(&node_a));
    println!("Node B reference count: {}", Rc::strong_count(&node_b));
    
    println!();
    println!("🔄 The cycle: A -> B -> A");
    println!("Neither can be dropped because each has a reference count > 0");
    println!("This is a memory leak!");
    println!();
}

fn demonstrate_weak_references() {
    println!("💪 Weak References - Breaking Cycles:");
    println!();
    
    println!("Weak<T> provides a non-owning reference that:");
    println!("• Doesn't affect reference count");
    println!("• Can become invalid if the data is dropped");
    println!("• Must be upgraded to Rc<T> to access data");
    println!("• Helps prevent reference cycles");
    println!();
    
    let data = Rc::new(String::from("Important data"));
    println!("Created Rc with data: {}", data);
    println!("Strong count: {}", Rc::strong_count(&data));
    println!("Weak count: {}", Rc::weak_count(&data));
    
    // Create weak reference
    let weak_ref = Rc::downgrade(&data);
    println!("Created weak reference");
    println!("Strong count: {}", Rc::strong_count(&data));
    println!("Weak count: {}", Rc::weak_count(&data));
    
    // Try to upgrade weak reference
    match weak_ref.upgrade() {
        Some(strong_ref) => {
            println!("Successfully upgraded weak reference: {}", strong_ref);
            println!("Strong count during upgrade: {}", Rc::strong_count(&strong_ref));
        },
        None => println!("Weak reference points to dropped data"),
    }
    
    // Drop the original strong reference
    drop(data);
    println!("Dropped original strong reference");
    
    // Try to upgrade again
    match weak_ref.upgrade() {
        Some(strong_ref) => println!("Upgraded: {}", strong_ref),
        None => println!("❌ Weak reference is now invalid - data was dropped"),
    }
    
    println!();
}

fn demonstrate_tree_structure() {
    println!("🌳 Tree Structure with Parent-Child Relationships:");
    println!();
    
    let root = Node::new(1);
    println!("Created root node with value 1");
    println!("Root strong count: {}", Rc::strong_count(&root));
    println!("Root weak count: {}", Rc::weak_count(&root));
    
    let child1 = Node::new(2);
    let child2 = Node::new(3);
    
    println!("Created child nodes with values 2 and 3");
    println!("Child1 strong count: {}", Rc::strong_count(&child1));
    println!("Child2 strong count: {}", Rc::strong_count(&child2));
    
    Node::add_child(&root, child1.clone());
    Node::add_child(&root, child2.clone());
    
    println!("Added children to root");
    println!("Root strong count: {}", Rc::strong_count(&root));
    println!("Root weak count: {}", Rc::weak_count(&root)); // Should be 2 (one from each child)
    println!("Child1 strong count: {}", Rc::strong_count(&child1));
    println!("Child2 strong count: {}", Rc::strong_count(&child2));
    
    // Access parent from child
    println!();
    println!("🔍 Accessing parent from child:");
    if let Some(parent) = child1.parent.borrow().upgrade() {
        println!("Child1's parent has value: {}", parent.value);
    } else {
        println!("Child1's parent is no longer available");
    }
    
    // Print tree structure
    println!();
    println!("📊 Tree structure:");
    print_tree(&root, 0);
    
    println!();
    println!("🔄 Reference pattern:");
    println!("• Parent -> Child: Strong reference (Rc)");
    println!("• Child -> Parent: Weak reference (Weak)");
    println!("• This prevents cycles while maintaining relationships");
    
    // Demonstrate cleanup
    println!();
    println!("🧹 Cleanup demonstration:");
    drop(child1);
    println!("Dropped child1 reference");
    println!("Root weak count: {}", Rc::weak_count(&root));
    
    drop(child2);
    println!("Dropped child2 reference");
    println!("Root weak count: {}", Rc::weak_count(&root));
    
    println!("Root strong count: {}", Rc::strong_count(&root));
    println!();
}

fn print_tree(node: &Rc<Node>, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{}Node({})", indent, node.value);
    
    for child in node.children.borrow().iter() {
        print_tree(child, depth + 1);
    }
}

fn demonstrate_preventing_cycles() {
    println!("🛡️ Strategies for Preventing Reference Cycles:");
    println!();
    
    println!("1. 🎯 Design Patterns:");
    println!("   • Parent-Child: Parent owns children (Rc), children reference parent (Weak)");
    println!("   • Owner-Observer: Owner holds observers (Rc), observers reference owner (Weak)");
    println!("   • Graph Traversal: Use indices instead of references");
    println!();
    
    println!("2. 🔧 Architectural Solutions:");
    println!("   • Use unique ownership when possible (Box<T>)");
    println!("   • Consider using indices into a central collection");
    println!("   • Use lifetimes to express borrowing relationships");
    println!("   • Break cycles manually when objects are no longer needed");
    println!();
    
    println!("3. 🧪 Testing for Cycles:");
    println!("   • Monitor memory usage in long-running applications");
    println!("   • Use debugging tools to detect memory leaks");
    println!("   • Create unit tests that verify reference counts");
    println!("   • Use weak references for back-references");
    println!();
    
    // Example of manual cycle breaking
    #[derive(Debug)]
    struct Connection {
        id: u32,
        peer: RefCell<Option<Weak<Connection>>>,
    }
    
    impl Connection {
        fn new(id: u32) -> Rc<Connection> {
            Rc::new(Connection {
                id,
                peer: RefCell::new(None),
            })
        }
        
        fn connect(conn1: &Rc<Connection>, conn2: &Rc<Connection>) {
            *conn1.peer.borrow_mut() = Some(Rc::downgrade(conn2));
            *conn2.peer.borrow_mut() = Some(Rc::downgrade(conn1));
        }
        
        fn disconnect(&self) {
            *self.peer.borrow_mut() = None;
        }
    }
    
    println!("🔌 Connection Example (avoiding cycles):");
    let conn1 = Connection::new(1);
    let conn2 = Connection::new(2);
    
    println!("Connection 1 strong count: {}", Rc::strong_count(&conn1));
    println!("Connection 2 strong count: {}", Rc::strong_count(&conn2));
    
    Connection::connect(&conn1, &conn2);
    
    println!("After connecting:");
    println!("Connection 1 strong count: {}", Rc::strong_count(&conn1));
    println!("Connection 2 strong count: {}", Rc::strong_count(&conn2));
    println!("Connection 1 weak count: {}", Rc::weak_count(&conn1));
    println!("Connection 2 weak count: {}", Rc::weak_count(&conn2));
    
    // Manual cleanup to break potential cycles
    conn1.disconnect();
    conn2.disconnect();
    
    println!("After disconnecting:");
    println!("Connection 1 weak count: {}", Rc::weak_count(&conn1));
    println!("Connection 2 weak count: {}", Rc::weak_count(&conn2));
    
    println!();
    println!("💡 Key Takeaways:");
    println!("• Reference cycles can cause memory leaks");
    println!("• Use Weak<T> for non-owning references");
    println!("• Design data structures to avoid cycles when possible");
    println!("• Consider alternative patterns like indices or unique ownership");
    println!("• Test for memory leaks in long-running applications");
}