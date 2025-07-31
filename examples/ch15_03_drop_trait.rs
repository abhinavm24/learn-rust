//! # Chapter 15.3: Running Code on Cleanup with the Drop Trait
//! 
//! This example demonstrates:
//! - Implementing the Drop trait for custom cleanup code
//! - RAII (Resource Acquisition Is Initialization) pattern
//! - Automatic resource management and cleanup
//! - Manual dropping with std::mem::drop
//! - Order of drop execution
//! 
//! Run this example with: `cargo run --example ch15_03_drop_trait`

use rust_book_examples::print_chapter_header;
use std::fs::File;
use std::io::Write;

fn main() {
    print_chapter_header("Chapter 15.3", "Running Code on Cleanup with the Drop Trait");

    println!("The Drop trait allows us to run cleanup code when values go out of scope!");
    println!();

    demonstrate_basic_drop();
    demonstrate_drop_order();
    demonstrate_manual_drop();
    demonstrate_resource_management();
    demonstrate_drop_with_smart_pointers();
}

/// Basic smart pointer with Drop implementation
struct CustomSmartPointer {
    data: String,
}

impl CustomSmartPointer {
    fn new(data: &str) -> Self {
        println!("📦 Creating CustomSmartPointer with data: '{}'", data);
        CustomSmartPointer {
            data: data.to_string(),
        }
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("🗑️  Dropping CustomSmartPointer with data: '{}'", self.data);
    }
}

/// Demonstrates basic Drop trait functionality
fn demonstrate_basic_drop() {
    println!("=== Basic Drop Trait ===");
    
    {
        let _c = CustomSmartPointer::new("my stuff");
        let _d = CustomSmartPointer::new("other stuff");
        println!("📝 CustomSmartPointers created, still in scope");
    } // Drop gets called here automatically for both c and d
    
    println!("✅ Scope ended - Drop was called automatically");
    println!();
}

/// Demonstrates the order in which Drop is called
fn demonstrate_drop_order() {
    println!("=== Drop Order (LIFO - Last In, First Out) ===");
    
    {
        let _first = CustomSmartPointer::new("first");
        let _second = CustomSmartPointer::new("second");
        let _third = CustomSmartPointer::new("third");
        
        println!("📝 All three pointers created");
        // They will be dropped in reverse order: third, second, first
    }
    
    println!("✅ Notice they dropped in reverse order (LIFO)");
    println!();
}

/// Demonstrates manual dropping with std::mem::drop
fn demonstrate_manual_drop() {
    println!("=== Manual Drop with std::mem::drop ===");
    
    let c = CustomSmartPointer::new("manual drop example");
    println!("📝 About to manually drop the pointer");
    
    // Manually call drop before the end of scope
    drop(c);
    
    println!("✅ Manually dropped - happened immediately");
    
    // This would cause a compile error because c has been moved:
    // println!("Data: {}", c.data); // Error: borrow of moved value
    
    println!();
}

/// File management example demonstrating RAII
struct ManagedFile {
    filename: String,
    file: Option<File>,
}

impl ManagedFile {
    fn new(filename: &str) -> std::io::Result<Self> {
        println!("📁 Opening file: {}", filename);
        let file = File::create(filename)?;
        
        Ok(ManagedFile {
            filename: filename.to_string(),
            file: Some(file),
        })
    }
    
    fn write_data(&mut self, data: &str) -> std::io::Result<()> {
        if let Some(ref mut file) = self.file {
            writeln!(file, "{}", data)?;
            println!("✏️  Written data to {}: '{}'", self.filename, data);
        }
        Ok(())
    }
}

impl Drop for ManagedFile {
    fn drop(&mut self) {
        println!("🗑️  Closing file: {}", self.filename);
        // File is automatically closed when it goes out of scope
        // We could add additional cleanup here if needed
        
        // Clean up the test file
        if let Err(e) = std::fs::remove_file(&self.filename) {
            println!("⚠️  Warning: Could not remove test file: {}", e);
        } else {
            println!("🧹 Test file {} removed", self.filename);
        }
    }
}

/// Demonstrates resource management with Drop
fn demonstrate_resource_management() {
    println!("=== Resource Management with Drop (RAII) ===");
    
    {
        let mut managed_file = match ManagedFile::new("test_drop.txt") {
            Ok(file) => file,
            Err(e) => {
                println!("❌ Failed to create file: {}", e);
                return;
            }
        };
        
        // Use the file
        if let Err(e) = managed_file.write_data("Hello from Drop trait!") {
            println!("❌ Failed to write to file: {}", e);
        }
        
        if let Err(e) = managed_file.write_data("This will be automatically saved and closed.") {
            println!("❌ Failed to write to file: {}", e);
        }
        
        println!("📝 File operations complete, about to go out of scope");
    } // ManagedFile::drop() called here - file is closed and cleaned up
    
    println!("✅ File was automatically closed and cleaned up");
    println!();
}

/// Network connection simulation
struct NetworkConnection {
    address: String,
    connected: bool,
}

impl NetworkConnection {
    fn new(address: &str) -> Self {
        println!("🌐 Connecting to: {}", address);
        NetworkConnection {
            address: address.to_string(),
            connected: true,
        }
    }
    
    fn send_data(&self, data: &str) {
        if self.connected {
            println!("📡 Sending data to {}: '{}'", self.address, data);
        } else {
            println!("❌ Cannot send data - not connected");
        }
    }
}

impl Drop for NetworkConnection {
    fn drop(&mut self) {
        if self.connected {
            println!("🔌 Disconnecting from: {}", self.address);
            self.connected = false;
        }
    }
}

/// Database connection simulation
struct DatabaseConnection {
    db_name: String,
    transaction_active: bool,
}

impl DatabaseConnection {
    fn new(db_name: &str) -> Self {
        println!("🗄️  Connecting to database: {}", db_name);
        DatabaseConnection {
            db_name: db_name.to_string(),
            transaction_active: false,
        }
    }
    
    fn begin_transaction(&mut self) {
        println!("🔄 Beginning transaction on {}", self.db_name);
        self.transaction_active = true;
    }
    
    fn execute_query(&self, query: &str) {
        println!("🔍 Executing query on {}: {}", self.db_name, query);
    }
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        if self.transaction_active {
            println!("🔄 Rolling back active transaction on {}", self.db_name);
        }
        println!("🗄️  Closing database connection to: {}", self.db_name);
    }
}

/// Demonstrates Drop with smart pointers and complex scenarios
fn demonstrate_drop_with_smart_pointers() {
    println!("=== Drop with Smart Pointers and Complex Scenarios ===");
    
    // Scenario 1: Network connections
    {
        let conn1 = NetworkConnection::new("192.168.1.1:8080");
        let conn2 = NetworkConnection::new("api.example.com:443");
        
        conn1.send_data("Hello Server 1");
        conn2.send_data("Hello Server 2");
        
        println!("📝 About to close network connections");
    } // Both connections automatically closed
    
    println!();
    
    // Scenario 2: Database with transaction
    {
        let mut db = DatabaseConnection::new("user_database");
        db.begin_transaction();
        db.execute_query("INSERT INTO users (name) VALUES ('Alice')");
        
        // Transaction will be rolled back automatically if we don't commit
        println!("📝 About to go out of scope with active transaction");
    } // Transaction rolled back and connection closed
    
    println!();
    
    // Scenario 3: Nested smart pointers
    {
        let boxed_pointer = Box::new(CustomSmartPointer::new("boxed data"));
        let _another_box = Box::new(NetworkConnection::new("nested.example.com:80"));
        
        println!("📝 Nested smart pointers created");
    } // All nested structures properly cleaned up
    
    println!("✅ All resources cleaned up automatically");
    println!();
}

// === ADVANCED DROP EXAMPLES ===

/// Example of a reference counting smart pointer with Drop
struct SimpleRc<T> {
    data: Box<T>,
    ref_count: *mut usize,
}

impl<T> SimpleRc<T> {
    fn new(data: T) -> Self {
        println!("📊 Creating SimpleRc with ref count 1");
        SimpleRc {
            data: Box::new(data),
            ref_count: Box::into_raw(Box::new(1)),
        }
    }
}

impl<T: Clone> Clone for SimpleRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            *self.ref_count += 1;
            println!("📊 Cloning SimpleRc, ref count now: {}", *self.ref_count);
        }
        
        SimpleRc {
            data: self.data.clone(),
            ref_count: self.ref_count,
        }
    }
}

impl<T> Drop for SimpleRc<T> {
    fn drop(&mut self) {
        unsafe {
            *self.ref_count -= 1;
            println!("📊 Dropping SimpleRc, ref count now: {}", *self.ref_count);
            
            if *self.ref_count == 0 {
                println!("📊 Last reference dropped, cleaning up data");
                // Convert raw pointer back to Box to properly drop it
                let _ = Box::from_raw(self.ref_count);
            }
        }
    }
}

/// Demonstrates custom reference counting
#[allow(dead_code)]
fn demonstrate_reference_counting() {
    println!("=== Custom Reference Counting with Drop ===");
    
    {
        let rc1 = SimpleRc::new(String::from("reference counted data"));
        {
            let rc2 = rc1.clone();
            {
                let rc3 = rc2.clone();
                println!("📝 Three references exist");
            } // rc3 dropped
            println!("📝 Two references exist");
        } // rc2 dropped
        println!("📝 One reference exists");
    } // rc1 dropped - data finally cleaned up
    
    println!("✅ Reference counting cleanup complete");
}

/// Example showing Drop is not called on panic
fn demonstrate_drop_and_panic() {
    println!("=== Drop Behavior During Panic ===");
    
    struct PanicTracker {
        name: String,
    }
    
    impl Drop for PanicTracker {
        fn drop(&mut self) {
            println!("🗑️  Dropping PanicTracker: {}", self.name);
        }
    }
    
    let _guard = PanicTracker {
        name: "panic guard".to_string(),
    };
    
    println!("📝 PanicTracker created");
    
    // Even if we panic, Drop will still be called during stack unwinding
    // (unless we use panic=abort)
    
    // Uncomment to test panic behavior:
    // panic!("Test panic!");
    
    println!("✅ Normal execution - Drop will be called at end of scope");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_drop_called() {
        static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);
        
        struct DropCounter;
        
        impl Drop for DropCounter {
            fn drop(&mut self) {
                DROP_COUNT.fetch_add(1, Ordering::SeqCst);
            }
        }
        
        {
            let _counter = DropCounter;
        } // Drop should be called here
        
        assert_eq!(1, DROP_COUNT.load(Ordering::SeqCst));
    }

    #[test]
    fn test_drop_order() {
        static DROP_ORDER: std::sync::Mutex<Vec<usize>> = std::sync::Mutex::new(Vec::new());
        
        struct OrderedDrop {
            id: usize,
        }
        
        impl Drop for OrderedDrop {
            fn drop(&mut self) {
                DROP_ORDER.lock().unwrap().push(self.id);
            }
        }
        
        {
            let _first = OrderedDrop { id: 1 };
            let _second = OrderedDrop { id: 2 };
            let _third = OrderedDrop { id: 3 };
        } // Should drop in order: 3, 2, 1 (LIFO)
        
        let order = DROP_ORDER.lock().unwrap();
        assert_eq!(*order, vec![3, 2, 1]);
    }

    #[test]
    fn test_manual_drop() {
        static DROPPED: AtomicUsize = AtomicUsize::new(0);
        
        struct ManualDropTest;
        
        impl Drop for ManualDropTest {
            fn drop(&mut self) {
                DROPPED.store(1, Ordering::SeqCst);
            }
        }
        
        let test_obj = ManualDropTest;
        assert_eq!(0, DROPPED.load(Ordering::SeqCst));
        
        drop(test_obj);
        assert_eq!(1, DROPPED.load(Ordering::SeqCst));
    }

    #[test]
    fn test_network_connection_drop() {
        // Test that Drop is called properly for our NetworkConnection
        let conn = NetworkConnection::new("test.com:80");
        assert!(conn.connected);
        
        // Drop should be called automatically at end of scope
        drop(conn);
        // We can't test the connected state after drop since the value is moved
        // but we can verify this manually by running the example
    }
}