# Chapter 19.1: Unsafe Rust

## Key Takeaways
- **Unsafe Rust**: Allows bypassing compiler's memory safety guarantees
- **Five Unsafe Superpowers**: Dereference raw pointers, call unsafe functions, access/modify statics, implement unsafe traits, access union fields
- **Memory Safety**: Developer responsibility in unsafe blocks
- **FFI**: Foreign Function Interface requires unsafe
- **Performance**: Sometimes needed for zero-cost abstractions

## Understanding Unsafe Rust

### Why Unsafe Exists
Rust's borrow checker is conservative - it rejects some programs that are actually safe but difficult to prove safe at compile time.

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    
    // This would be rejected by the borrow checker
    // even though it's actually safe
    let (a, b) = r.split_at_mut(3);
    
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
```

### The Five Unsafe Superpowers

#### 1. Dereference Raw Pointers

```rust
fn main() {
    let mut num = 5;
    
    // Creating raw pointers is safe
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    // Dereferencing raw pointers requires unsafe
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
```

**Raw Pointer Characteristics:**
- Can be null or point to invalid memory
- Don't implement automatic cleanup
- Can create multiple mutable pointers to same location
- No borrowing rules enforcement

```rust
fn main() {
    let address = 0x012345usize;
    let r = address as *const i32;
    
    // This might crash or access invalid memory!
    unsafe {
        // println!("Value at address: {}", *r);  // Dangerous!
    }
}
```

#### 2. Call Unsafe Functions

```rust
unsafe fn dangerous() {
    println!("This function is marked unsafe");
}

fn main() {
    unsafe {
        dangerous();
    }
}
```

**Creating Safe Abstractions:**
```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    
    assert!(mid <= len);
    
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (a, b) = split_at_mut(&mut v, 3);
    
    println!("a: {:?}", a);  // [1, 2, 3]
    println!("b: {:?}", b);  // [4, 5, 6]
}
```

#### 3. Access or Modify Static Variables

```rust
static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: usize = 0;

fn add_to_count(inc: usize) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    println!("name is: {}", HELLO_WORLD);
    
    add_to_count(3);
    
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

**Static vs Const:**
- `static`: Single instance with fixed memory address
- `const`: Inlined at each usage site
- Mutable statics are inherently unsafe due to data races

#### 4. Implement Unsafe Traits

```rust
unsafe trait Foo {
    // Methods can go here
}

unsafe impl Foo for i32 {
    // Implementation details
}

fn main() {
    // Using the unsafe trait
}
```

**Common Unsafe Traits:**
- `Send`: Types safe to transfer between threads
- `Sync`: Types safe to access from multiple threads

```rust
use std::marker::{Send, Sync};

struct MyBox<T>(*mut T);

unsafe impl<T: Send> Send for MyBox<T> {}
unsafe impl<T: Sync> Sync for MyBox<T> {}
```

#### 5. Access Fields of Unions

```rust
union MyUnion {
    f1: u32,
    f2: f32,
}

fn main() {
    let u = MyUnion { f1: 1 };
    
    unsafe {
        let f1 = u.f1;
        println!("f1: {}", f1);
        
        // This interprets the same memory as f32
        let f2 = u.f2;
        println!("f2: {}", f2);
    }
}
```

## Using extern Functions for FFI

### Calling C Functions from Rust

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

### Calling Rust Functions from Other Languages

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

**Example with C Integration:**
```rust
use std::os::raw::c_char;
use std::ffi::CString;

extern "C" {
    fn puts(s: *const c_char) -> i32;
}

fn main() {
    let c_string = CString::new("Hello from Rust!").expect("CString::new failed");
    unsafe {
        puts(c_string.as_ptr());
    }
}
```

## Advanced Raw Pointer Usage

### Pointer Arithmetic

```rust
fn main() {
    let mut nums = [1, 2, 3, 4, 5];
    let ptr = nums.as_mut_ptr();
    
    unsafe {
        // Access elements using pointer arithmetic
        for i in 0..5 {
            let element_ptr = ptr.add(i);
            *element_ptr *= 2;
        }
    }
    
    println!("{:?}", nums);  // [2, 4, 6, 8, 10]
}
```

### Creating Slices from Raw Pointers

```rust
use std::slice;

fn main() {
    let data = [1, 2, 3, 4, 5];
    let ptr = data.as_ptr();
    let len = data.len();
    
    unsafe {
        let slice = slice::from_raw_parts(ptr, len);
        println!("Slice: {:?}", slice);
    }
}
```

## Memory Management with Unsafe

### Manual Memory Allocation

```rust
use std::alloc::{alloc, dealloc, Layout};

fn main() {
    unsafe {
        let layout = Layout::new::<i32>();
        let ptr = alloc(layout) as *mut i32;
        
        if ptr.is_null() {
            panic!("Failed to allocate memory");
        }
        
        // Write to allocated memory
        *ptr = 42;
        println!("Value: {}", *ptr);
        
        // Don't forget to deallocate!
        dealloc(ptr as *mut u8, layout);
    }
}
```

### Working with Uninitialized Memory

```rust
use std::mem::MaybeUninit;

fn main() {
    let mut data: [MaybeUninit<i32>; 10] = MaybeUninit::uninit_array();
    
    // Initialize the array
    for (i, elem) in data.iter_mut().enumerate() {
        elem.write(i as i32);
    }
    
    // Convert to initialized array
    let data: [i32; 10] = unsafe {
        MaybeUninit::array_assume_init(data)
    };
    
    println!("{:?}", data);
}
```

## Thread Safety and Unsafe

### Implementing Send and Sync

```rust
use std::ptr::NonNull;
use std::marker::PhantomData;

struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for MyVec<T> {}
unsafe impl<T: Sync> Sync for MyVec<T> {}

impl<T> MyVec<T> {
    fn new() -> Self {
        MyVec {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
            _marker: PhantomData,
        }
    }
}
```

### Atomic Operations

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", counter.load(Ordering::SeqCst));
}
```

## Best Practices for Unsafe Code

### 1. Minimize Unsafe Blocks

```rust
// ❌ Bad: Large unsafe block
unsafe {
    let ptr = get_pointer();
    do_safe_operation();
    let value = *ptr;
    another_safe_operation();
    return value;
}

// ✅ Good: Minimal unsafe block
let ptr = get_pointer();
do_safe_operation();
let value = unsafe { *ptr };
another_safe_operation();
return value;
```

### 2. Document Safety Requirements

```rust
/// # Safety
/// 
/// The caller must ensure that:
/// - `ptr` points to valid memory
/// - `ptr` is properly aligned for type `T`
/// - The memory pointed to by `ptr` is not accessed by other threads
unsafe fn read_raw<T>(ptr: *const T) -> T {
    std::ptr::read(ptr)
}
```

### 3. Create Safe Abstractions

```rust
pub struct SafeWrapper {
    inner: *mut i32,
    len: usize,
}

impl SafeWrapper {
    pub fn new(data: Vec<i32>) -> Self {
        let mut data = data.into_boxed_slice();
        let ptr = data.as_mut_ptr();
        let len = data.len();
        std::mem::forget(data);  // Prevent deallocation
        
        SafeWrapper { inner: ptr, len }
    }
    
    pub fn get(&self, index: usize) -> Option<i32> {
        if index >= self.len {
            return None;
        }
        
        unsafe {
            Some(*self.inner.add(index))
        }
    }
}

impl Drop for SafeWrapper {
    fn drop(&mut self) {
        unsafe {
            let data = Box::from_raw(
                std::slice::from_raw_parts_mut(self.inner, self.len)
            );
            drop(data);
        }
    }
}
```

### 4. Test Thoroughly

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_safe_wrapper() {
        let wrapper = SafeWrapper::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(wrapper.get(0), Some(1));
        assert_eq!(wrapper.get(10), None);
    }
    
    #[test]
    fn test_memory_safety() {
        // Test for memory leaks, double frees, etc.
    }
}
```

## Common Unsafe Patterns

### Option-like Enums with Raw Pointers

```rust
enum RawOption<T> {
    Some(*mut T),
    None,
}

impl<T> RawOption<T> {
    fn new(value: T) -> Self {
        let boxed = Box::new(value);
        RawOption::Some(Box::into_raw(boxed))
    }
    
    fn take(&mut self) -> Option<T> {
        match *self {
            RawOption::Some(ptr) => {
                *self = RawOption::None;
                unsafe {
                    Some(*Box::from_raw(ptr))
                }
            }
            RawOption::None => None,
        }
    }
}
```

### Interfacing with C Libraries

```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
}

fn safe_strlen(s: &str) -> usize {
    let c_string = CString::new(s).expect("CString::new failed");
    unsafe {
        strlen(c_string.as_ptr())
    }
}

fn main() {
    let length = safe_strlen("Hello, world!");
    println!("Length: {}", length);
}
```

## When to Use Unsafe

### ✅ Good Reasons to Use Unsafe
- Implementing fundamental data structures
- Interfacing with C libraries (FFI)
- Optimizing performance-critical code
- Implementing abstractions that safe Rust can't express

### ❌ Bad Reasons to Use Unsafe
- Working around borrow checker for convenience
- "I know what I'm doing" without proper understanding
- Premature optimization
- Avoiding proper error handling

## Integration with Safe Rust

### Unsafe Functions with Safe Interfaces

```rust
pub fn safe_split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    assert!(mid <= slice.len());
    
    unsafe {
        let ptr = slice.as_mut_ptr();
        let len = slice.len();
        
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

### Maintaining Invariants

```rust
struct SortedVec<T: Ord> {
    inner: Vec<T>,
}

impl<T: Ord> SortedVec<T> {
    pub fn new() -> Self {
        SortedVec { inner: Vec::new() }
    }
    
    pub fn insert(&mut self, item: T) {
        let pos = self.inner.binary_search(&item).unwrap_or_else(|e| e);
        self.inner.insert(pos, item);
        // Invariant: inner is always sorted
    }
    
    // Safe because we maintain the sorted invariant
    pub fn get_sorted_slice(&self) -> &[T] {
        &self.inner
    }
}
```

Unsafe Rust provides the flexibility needed for systems programming while maintaining most of Rust's safety guarantees through careful encapsulation and documentation.