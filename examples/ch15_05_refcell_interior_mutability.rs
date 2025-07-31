use rust_book_examples::print_chapter_header;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    print_chapter_header("Chapter 15.5", "RefCell<T> and the Interior Mutability Pattern");
    
    println!("🔄 Interior Mutability with RefCell<T>");
    println!();
    
    demonstrate_interior_mutability_concept();
    demonstrate_refcell_basics();
    demonstrate_rc_refcell_combination();
    demonstrate_borrowing_rules_at_runtime();
    demonstrate_mock_objects();
}

fn demonstrate_interior_mutability_concept() {
    println!("🔒 Interior Mutability Pattern:");
    println!();
    
    println!("Interior mutability allows you to mutate data even when");
    println!("there are immutable references to that data.");
    println!();
    
    println!("📋 Key Concepts:");
    println!("• RefCell<T> enforces borrowing rules at runtime");
    println!("• Can have multiple immutable borrows OR one mutable borrow");
    println!("• Panics if borrowing rules are violated at runtime");
    println!("• Useful when you know code is correct but compiler can't prove it");
    println!();
}

fn demonstrate_refcell_basics() {
    println!("🧪 RefCell<T> Basic Operations:");
    println!();
    
    let data = RefCell::new(vec![1, 2, 3]);
    println!("Created RefCell with data: {:?}", data);
    
    // Immutable borrow
    {
        let borrowed = data.borrow();
        println!("Immutable borrow: {:?}", *borrowed);
        println!("Length: {}", borrowed.len());
        
        // Can have multiple immutable borrows
        let borrowed2 = data.borrow();
        println!("Second immutable borrow: {:?}", *borrowed2);
    } // borrows dropped here
    
    // Mutable borrow
    {
        let mut borrowed_mut = data.borrow_mut();
        borrowed_mut.push(4);
        borrowed_mut.push(5);
        println!("After mutable operations: {:?}", *borrowed_mut);
    } // mutable borrow dropped here
    
    println!("Final state: {:?}", data);
    println!();
}

#[derive(Debug)]
struct Counter {
    value: RefCell<i32>,
}

impl Counter {
    fn new() -> Counter {
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
}

fn demonstrate_rc_refcell_combination() {
    println!("🤝 Combining Rc<T> and RefCell<T>:");
    println!();
    
    println!("Rc<RefCell<T>> pattern provides:");
    println!("• Multiple ownership (Rc)");
    println!("• Interior mutability (RefCell)");
    println!();
    
    let counter = Rc::new(Counter::new());
    
    let counter1 = Rc::clone(&counter);
    let counter2 = Rc::clone(&counter);
    let counter3 = Rc::clone(&counter);
    
    println!("Initial value: {}", counter.get());
    println!("Reference count: {}", Rc::strong_count(&counter));
    
    counter1.increment();
    println!("After counter1 increment: {}", counter.get());
    
    counter2.increment();
    counter2.increment();
    println!("After counter2 increments: {}", counter.get());
    
    counter3.increment();
    println!("After counter3 increment: {}", counter.get());
    
    println!("All counters share the same data!");
    println!();
    
    // Demonstrate shared mutable data
    let shared_data = Rc::new(RefCell::new(vec!["initial".to_string()]));
    
    let data1 = Rc::clone(&shared_data);
    let data2 = Rc::clone(&shared_data);
    
    data1.borrow_mut().push("from data1".to_string());
    data2.borrow_mut().push("from data2".to_string());
    
    println!("Shared vector: {:?}", shared_data.borrow());
    println!();
}

fn demonstrate_borrowing_rules_at_runtime() {
    println!("⚠️ Runtime Borrowing Rule Enforcement:");
    println!();
    
    let data = RefCell::new(String::from("Hello"));
    
    println!("✅ Valid: Multiple immutable borrows");
    {
        let borrow1 = data.borrow();
        let borrow2 = data.borrow();
        println!("Borrow1: {}, Borrow2: {}", *borrow1, *borrow2);
    }
    
    println!("✅ Valid: Single mutable borrow");
    {
        let mut borrow_mut = data.borrow_mut();
        borrow_mut.push_str(", World!");
        println!("Mutable borrow result: {}", *borrow_mut);
    }
    
    println!("⚠️ This would panic at runtime:");
    println!("// let borrow = data.borrow();");
    println!("// let borrow_mut = data.borrow_mut(); // PANIC!");
    println!();
    
    // Demonstrate safe usage with try_borrow
    println!("🛡️ Safe borrowing with try_borrow:");
    let borrow_result = data.try_borrow();
    match borrow_result {
        Ok(borrow) => println!("Successfully borrowed: {}", *borrow),
        Err(e) => println!("Borrow failed: {}", e),
    }
    
    let mut_borrow_result = data.try_borrow_mut();
    match mut_borrow_result {
        Ok(mut borrow) => {
            borrow.push_str(" (modified safely)");
            println!("Successfully mutably borrowed and modified");
        },
        Err(e) => println!("Mutable borrow failed: {}", e),
    }
    
    println!();
}

// Mock object example for testing
trait Messenger {
    fn send(&self, msg: &str);
}

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
    
    fn messages_sent(&self) -> Vec<String> {
        self.sent_messages.borrow().clone()
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        // Even though send takes &self (immutable), we can mutate internal state
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    
    fn set_value(&mut self, value: usize) {
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

fn demonstrate_mock_objects() {
    println!("🧪 Mock Objects with RefCell<T>:");
    println!();
    
    println!("RefCell enables interior mutability for testing scenarios");
    println!("where we need to track calls to immutable methods.");
    println!();
    
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    
    println!("Testing limit tracker with various values:");
    
    limit_tracker.set_value(50);
    println!("Set value to 50 (50% of limit)");
    
    limit_tracker.set_value(80);
    println!("Set value to 80 (80% of limit)");
    
    limit_tracker.set_value(95);
    println!("Set value to 95 (95% of limit)");
    
    limit_tracker.set_value(110);
    println!("Set value to 110 (110% of limit)");
    
    println!();
    println!("📬 Messages sent by mock messenger:");
    let messages = mock_messenger.messages_sent();
    for (i, message) in messages.iter().enumerate() {
        println!("{}. {}", i + 1, message);
    }
    
    println!();
    println!("💡 Key Benefits of RefCell in Testing:");
    println!("• Allows mutation through immutable references");
    println!("• Perfect for mock objects that need to track state");
    println!("• Enables testing of objects with immutable interfaces");
    println!("• Runtime borrowing rule checking catches bugs");
    
    println!();
    println!("🔍 When to Use RefCell<T>:");
    println!("• You're sure your code is correct but compiler disagrees");
    println!("• Need interior mutability in single-threaded code");
    println!("• Creating mock objects for testing");
    println!("• Working with immutable data structures that need occasional updates");
    
    println!();
    println!("⚠️ Cautions:");
    println!("• Runtime panics instead of compile-time errors");
    println!("• Small runtime performance cost");
    println!("• Not thread-safe (use Mutex<T> for multi-threading)");
    println!("• Can create reference cycles with Rc<RefCell<T>>");
}