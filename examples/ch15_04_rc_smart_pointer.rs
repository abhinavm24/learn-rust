use rust_book_examples::print_chapter_header;
use std::rc::Rc;

fn main() {
    print_chapter_header("Chapter 15.4", "Rc<T>, the Reference Counted Smart Pointer");
    
    println!("📊 Reference Counting with Rc<T>");
    println!();
    
    demonstrate_rc_basics();
    demonstrate_multiple_ownership();
    demonstrate_rc_with_lists();
    demonstrate_rc_limitations();
}

fn demonstrate_rc_basics() {
    println!("🔢 Rc<T> Basics - Reference Counting:");
    println!();
    
    println!("Rc<T> enables multiple ownership of the same data");
    println!("It keeps track of the number of references to determine");
    println!("when the data should be cleaned up.");
    println!();
    
    let data = Rc::new(String::from("Hello, Rc!"));
    println!("Created Rc with data: {}", data);
    println!("Reference count: {}", Rc::strong_count(&data));
    
    {
        let data_clone1 = Rc::clone(&data);
        println!("After first clone: {}", Rc::strong_count(&data));
        
        {
            let data_clone2 = Rc::clone(&data);
            println!("After second clone: {}", Rc::strong_count(&data));
            println!("All three references point to: {}", data_clone2);
        }
        
        println!("After inner scope ends: {}", Rc::strong_count(&data));
    }
    
    println!("After outer scope ends: {}", Rc::strong_count(&data));
    println!();
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn demonstrate_multiple_ownership() {
    println!("🌳 Multiple Ownership with Rc<T>:");
    println!();
    
    // Without Rc, this wouldn't work because we can't have multiple owners
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Created list a: {:?}", a);
    println!("Reference count for a: {}", Rc::strong_count(&a));
    
    let b = Cons(3, Rc::clone(&a));
    println!("Created list b that shares a: {:?}", b);
    println!("Reference count for a after creating b: {}", Rc::strong_count(&a));
    
    let c = Cons(4, Rc::clone(&a));
    println!("Created list c that shares a: {:?}", c);
    println!("Reference count for a after creating c: {}", Rc::strong_count(&a));
    
    println!();
    println!("📊 Memory Layout:");
    println!("a: [5] -> [10] -> Nil");
    println!("b: [3] -> ^");
    println!("c: [4] -> ^");
    println!("Both b and c share the same tail through Rc!");
    println!();
}

fn demonstrate_rc_with_lists() {
    println!("📝 Practical Example - Shared Configuration:");
    println!();
    
    // Simulate a shared configuration that multiple components use
    #[derive(Debug)]
    struct Config {
        database_url: String,
        api_key: String,
        debug_mode: bool,
    }
    
    let shared_config = Rc::new(Config {
        database_url: "postgresql://localhost/mydb".to_string(),
        api_key: "secret-key-123".to_string(),
        debug_mode: true,
    });
    
    println!("Created shared config:");
    println!("{:#?}", shared_config);
    println!("Reference count: {}", Rc::strong_count(&shared_config));
    
    // Multiple components can now share this configuration
    let web_server_config = Rc::clone(&shared_config);
    let database_manager_config = Rc::clone(&shared_config);
    let logger_config = Rc::clone(&shared_config);
    
    println!("After creating component configs:");
    println!("Reference count: {}", Rc::strong_count(&shared_config));
    
    // Simulate using the configurations
    println!();
    println!("Web server using config: debug_mode = {}", web_server_config.debug_mode);
    println!("Database manager using config: database_url = {}", database_manager_config.database_url);
    println!("Logger using config: debug_mode = {}", logger_config.debug_mode);
    
    drop(web_server_config);
    println!("After dropping web server config: {}", Rc::strong_count(&shared_config));
    
    drop(database_manager_config);
    println!("After dropping database manager config: {}", Rc::strong_count(&shared_config));
    
    drop(logger_config);
    println!("After dropping logger config: {}", Rc::strong_count(&shared_config));
    println!();
}

fn demonstrate_rc_limitations() {
    println!("⚠️ Rc<T> Limitations and Considerations:");
    println!();
    
    println!("1. 🚫 Single-threaded only:");
    println!("   • Rc<T> is not thread-safe");
    println!("   • Use Arc<T> for multi-threaded scenarios");
    println!("   • Rc<T> has less overhead than Arc<T>");
    println!();
    
    println!("2. 📖 Immutable references only:");
    println!("   • Rc<T> only gives immutable references");
    println!("   • Cannot modify data through Rc<T>");
    println!("   • Combine with RefCell<T> for interior mutability");
    println!();
    
    println!("3. 🔄 Reference cycles can cause memory leaks:");
    println!("   • If A references B and B references A, neither will be dropped");
    println!("   • Use Weak<T> to break cycles");
    println!("   • Be careful with circular data structures");
    println!();
    
    println!("4. 📊 Runtime overhead:");
    println!("   • Reference counting has runtime cost");
    println!("   • Each clone/drop updates the count");
    println!("   • Consider if single ownership would work instead");
    println!();
    
    // Demonstrate attempting to mutate through Rc (this won't compile)
    let data = Rc::new(vec![1, 2, 3]);
    let data_clone = Rc::clone(&data);
    
    // This would not compile:
    // data.push(4); // Error: cannot borrow as mutable
    
    println!("✅ When to use Rc<T>:");
    println!("• Need multiple owners of the same data");
    println!("• Data is read-only or rarely modified");
    println!("• Single-threaded environment");
    println!("• Tree-like or graph-like data structures");
    println!("• Shared configuration or resources");
    
    println!();
    println!("🔧 Common patterns:");
    println!("• Rc<RefCell<T>> for shared mutable data");
    println!("• Rc<Vec<T>> for shared collections");
    println!("• Rc<dyn Trait> for shared trait objects");
    
    println!();
    println!("💡 Performance tip:");
    println!("Rc::clone() is cheap - it only increments a counter");
    println!("It doesn't deep-clone the data like .clone() would");
    
    println!("Current data reference count: {}", Rc::strong_count(&data));
    drop(data_clone);
    println!("After dropping clone: {}", Rc::strong_count(&data));
}