# Chapter 15.3: Running Code on Cleanup with the Drop Trait

## Key Takeaways

### Drop Trait Purpose
- **Cleanup Code**: Run code when value goes out of scope
- **Resource Management**: Free resources like files, network connections
- **RAII Pattern**: Resource Acquisition Is Initialization
- **Automatic**: Called automatically, no manual intervention needed

### Basic Drop Implementation
```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // Variables dropped in reverse order: d, then c
}
```

### Early Drop with std::mem::drop
```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    
    drop(c);  // Explicitly drop c early
    println!("CustomSmartPointer dropped before the end of main.");
}
```

### Real-World Example: File Handler
```rust
use std::fs::File;
use std::io::Write;

struct FileManager {
    file: Option<File>,
    filename: String,
}

impl FileManager {
    fn new(filename: &str) -> std::io::Result<Self> {
        let file = File::create(filename)?;
        Ok(FileManager {
            file: Some(file),
            filename: filename.to_string(),
        })
    }
    
    fn write_data(&mut self, data: &str) -> std::io::Result<()> {
        if let Some(ref mut file) = self.file {
            writeln!(file, "{}", data)?;
        }
        Ok(())
    }
}

impl Drop for FileManager {
    fn drop(&mut self) {
        if let Some(file) = self.file.take() {
            drop(file);  // Ensure file is closed
            println!("File '{}' has been closed and cleaned up", self.filename);
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut manager = FileManager::new("output.txt")?;
    manager.write_data("Hello, world!")?;
    manager.write_data("Goodbye, world!")?;
    
    // File automatically closed when manager goes out of scope
    Ok(())
}
```

### Drop Order and Ownership
```rust
struct HasDrop1;
struct HasDrop2;
struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}

impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("Dropping HasDrop1!");
    }
}

impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("Dropping HasDrop2!");
    }
}

impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops!");
    }
}

fn main() {
    let _x = HasTwoDrops {
        two: HasDrop2,
        one: HasDrop1,
    };
    println!("Running!");
    // Drop order: HasTwoDrops, then its fields in reverse declaration order
    // Output: Dropping HasTwoDrops!, Dropping HasDrop1!, Dropping HasDrop2!
}
```

Official Chapter: https://doc.rust-lang.org/book/ch15-03-drop.html

---
*Completed: ✓*