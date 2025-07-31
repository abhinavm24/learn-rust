//! Chapter 19.1: Unsafe Rust
//! 
//! This example demonstrates:
//! - The five unsafe superpowers
//! - Dereferencing raw pointers
//! - Calling unsafe functions
//! - Accessing and modifying static variables
//! - Implementing unsafe traits
//! - Accessing union fields
//! - Foreign Function Interface (FFI)
//! - Safe abstractions over unsafe code

use rust_book_examples::print_chapter_header;
use std::slice;

fn main() {
    print_chapter_header("Chapter 19.1", "Unsafe Rust");
    
    println!("=== Dereferencing Raw Pointers ===");
    raw_pointers_example();
    
    println!("\n=== Calling Unsafe Functions ===");
    unsafe_functions_example();
    
    println!("\n=== Accessing Static Variables ===");
    static_variables_example();
    
    println!("\n=== Creating Safe Abstractions ===");
    safe_abstractions_example();
    
    println!("\n=== Implementing Unsafe Traits ===");
    unsafe_traits_example();
    
    println!("\n=== Using Unions ===");
    unions_example();
    
    println!("\n=== Foreign Function Interface ===");
    ffi_example();
    
    println!("\n=== Advanced Raw Pointer Usage ===");
    advanced_pointer_usage();
}

fn raw_pointers_example() {
    let mut num = 5;
    
    // Creating raw pointers is safe
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    println!("Raw pointer addresses:");
    println!("r1 (const): {:p}", r1);
    println!("r2 (mut): {:p}", r2);
    
    // Dereferencing raw pointers requires unsafe
    unsafe {
        println!("r1 points to: {}", *r1);
        println!("r2 points to: {}", *r2);
        
        // Modifying through mutable raw pointer
        *r2 = 10;
        println!("After modification through r2: {}", *r1);
    }
    
    println!("Original variable num: {}", num);
    
    // Creating raw pointers from arbitrary memory addresses
    let address = 0x012345usize;
    let r = address as *const i32;
    
    println!("Created pointer to arbitrary address: {:p}", r);
    // Note: We won't dereference this as it would likely crash
    
    // Raw pointers can be null
    let null_ptr: *const i32 = std::ptr::null();
    println!("Null pointer: {:p}", null_ptr);
    println!("Is null: {}", null_ptr.is_null());
    
    // Raw pointers don't follow borrowing rules
    let mut value = 42;
    let ptr1 = &mut value as *mut i32;
    let ptr2 = &mut value as *mut i32; // This is allowed!
    
    unsafe {
        println!("Both pointers point to same location:");
        println!("ptr1: {}", *ptr1);
        println!("ptr2: {}", *ptr2);
        
        *ptr1 = 100;
        println!("After modifying through ptr1, ptr2 sees: {}", *ptr2);
    }
}

unsafe fn dangerous() {
    println!("This is an unsafe function - it could do dangerous things!");
    
    // Example of why this might be unsafe: raw pointer operations
    let mut value = 42;
    let ptr = &mut value as *mut i32;
    *ptr = 999; // Direct memory manipulation
    
    println!("Modified value through raw pointer: {}", value);
}

fn unsafe_functions_example() {
    println!("Calling an unsafe function:");
    
    unsafe {
        dangerous();
    }
    
    // Demonstrating split_at_mut - a safe wrapper around unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];
    
    let (left, right) = v.split_at_mut(3);
    println!("Left half: {:?}", left);
    println!("Right half: {:?}", right);
    
    // Modifying both halves simultaneously (which is safe)
    left[0] = 10;
    right[0] = 40;
    println!("After modification: {:?}", v);
    
    // Example of an unsafe block within a safe function
    fn safe_function_with_unsafe_block() {
        let mut numbers = vec![1, 2, 3, 4, 5];
        let ptr = numbers.as_mut_ptr();
        
        unsafe {
            // This is unsafe but contained within a safe function
            *ptr.add(2) = 999; // Modify the third element
        }
        
        println!("Modified vector: {:?}", numbers);
    }
    
    safe_function_with_unsafe_block();
}

// Static variables
static HELLO_WORLD: &str = "Hello, world!"; // Immutable static
static mut COUNTER: usize = 0; // Mutable static

fn add_to_count(inc: usize) {
    unsafe {
        COUNTER += inc;
    }
}

fn get_count() -> usize {
    unsafe { COUNTER }
}

fn static_variables_example() {
    println!("Immutable static: {}", HELLO_WORLD);
    
    // Accessing mutable static requires unsafe
    println!("Initial counter: {}", get_count());
    
    add_to_count(3);
    add_to_count(7);
    
    unsafe {
        println!("Counter after additions: {}", COUNTER);
        
        // Direct modification
        COUNTER *= 2;
        println!("Counter after doubling: {}", COUNTER);
    }
    
    // Demonstrating the difference between static and const
    const MAX_POINTS: u32 = 100_000; // This is inlined everywhere it's used
    println!("Const value: {}", MAX_POINTS);
    
    // Static has a specific memory location
    println!("Static memory address: {:p}", &HELLO_WORLD);
    println!("Static counter address: {:p}", unsafe { &COUNTER });
}

// Creating a safe abstraction over unsafe code
fn split_at_mut<T>(values: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
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

fn safe_abstractions_example() {
    println!("Using our safe split_at_mut function:");
    
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let (left, right) = split_at_mut(&mut numbers, 4);
    
    println!("Left: {:?}", left);
    println!("Right: {:?}", right);
    
    // We can safely modify both parts
    left[0] = 100;
    right[0] = 500;
    
    println!("After modification: {:?}", numbers);
    
    // Another example: safe wrapper for raw pointer operations
    struct SafePointer<T> {
        ptr: *mut T,
        len: usize,
    }
    
    impl<T> SafePointer<T> {
        fn new(data: Vec<T>) -> Self {
            let mut data = data.into_boxed_slice();
            let ptr = data.as_mut_ptr();
            let len = data.len();
            std::mem::forget(data); // Prevent automatic cleanup
            
            SafePointer { ptr, len }
        }
        
        fn get(&self, index: usize) -> Option<&T> {
            if index >= self.len {
                None
            } else {
                unsafe { Some(&*self.ptr.add(index)) }
            }
        }
        
        fn get_mut(&mut self, index: usize) -> Option<&mut T> {
            if index >= self.len {
                None
            } else {
                unsafe { Some(&mut *self.ptr.add(index)) }
            }
        }
        
        fn len(&self) -> usize {
            self.len
        }
    }
    
    impl<T> Drop for SafePointer<T> {
        fn drop(&mut self) {
            unsafe {
                let data = Box::from_raw(slice::from_raw_parts_mut(self.ptr, self.len));
                drop(data);
            }
        }
    }
    
    let mut safe_ptr = SafePointer::new(vec![10, 20, 30, 40, 50]);
    
    println!("Safe pointer length: {}", safe_ptr.len());
    println!("Element at index 2: {:?}", safe_ptr.get(2));
    
    if let Some(elem) = safe_ptr.get_mut(1) {
        *elem = 999;
    }
    
    println!("After modification, element at index 1: {:?}", safe_ptr.get(1));
}

// Unsafe traits
unsafe trait Foo {
    fn foo_method(&self);
}

struct MyStruct;

unsafe impl Foo for MyStruct {
    fn foo_method(&self) {
        println!("Implementing unsafe trait Foo");
    }
}

// Example with Send and Sync
use std::rc::Rc;

struct MyBox<T>(*mut T);

unsafe impl<T> Send for MyBox<T> where T: Send {}
unsafe impl<T> Sync for MyBox<T> where T: Sync {}

fn unsafe_traits_example() {
    println!("Implementing unsafe traits:");
    
    let my_struct = MyStruct;
    my_struct.foo_method();
    
    // Demonstrating Send and Sync
    println!("Send and Sync traits:");
    
    // Rc is not Send or Sync by default
    let rc_value = Rc::new(42);
    println!("Rc value: {}", rc_value);
    
    // But we can create our own types that are Send/Sync
    let my_box = MyBox(Box::into_raw(Box::new(100)));
    
    // This demonstrates that we've implemented Send/Sync
    // (In a real scenario, you'd use this across threads)
    unsafe {
        println!("MyBox contains: {}", *my_box.0);
        
        // Clean up
        let _ = Box::from_raw(my_box.0);
    }
    
    println!("Send and Sync traits allow types to be transferred between threads safely");
}

// Union example
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}

fn unions_example() {
    println!("Working with unions:");
    
    let u = MyUnion { f1: 1 };
    
    unsafe {
        // Accessing the field we set
        println!("u.f1 = {}", u.f1);
        
        // Accessing the same memory as a different type
        println!("u.f2 = {} (same memory interpreted as f32)", u.f2);
    }
    
    // More practical union example: type punning
    #[repr(C)]
    union FloatOrBytes {
        float_val: f32,
        bytes: [u8; 4],
    }
    
    let mut converter = FloatOrBytes { float_val: 3.14159 };
    
    unsafe {
        println!("Float value: {}", converter.float_val);
        println!("As bytes: {:?}", converter.bytes);
        
        // Modify bytes directly
        converter.bytes[0] = 0;
        println!("After modifying first byte, float value: {}", converter.float_val);
    }
}

// Foreign Function Interface
extern "C" {
    fn abs(input: i32) -> i32;
}

// Function callable from C
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn ffi_example() {
    println!("Foreign Function Interface:");
    
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
        println!("Absolute value of 42 according to C: {}", abs(42));
    }
    
    // Calling our exported function
    call_from_c();
    
    // Working with C strings
    use std::ffi::{CStr, CString};
    use std::os::raw::c_char;
    
    // Simulating a C function that works with strings
    unsafe fn simulate_c_strlen(s: *const c_char) -> usize {
        let c_str = CStr::from_ptr(s);
        c_str.to_str().unwrap().len()
    }
    
    let c_string = CString::new("Hello from Rust!").expect("CString::new failed");
    
    unsafe {
        let length = simulate_c_strlen(c_string.as_ptr());
        println!("C string length: {}", length);
        
        // Converting back to Rust string
        let back_to_rust = CStr::from_ptr(c_string.as_ptr())
            .to_str()
            .expect("Invalid UTF-8");
        println!("Back to Rust string: {}", back_to_rust);
    }
}

fn advanced_pointer_usage() {
    println!("Advanced raw pointer operations:");
    
    // Pointer arithmetic
    let mut numbers = [1, 2, 3, 4, 5];
    let ptr = numbers.as_mut_ptr();
    
    unsafe {
        println!("Original array: {:?}", numbers);
        
        // Modify elements using pointer arithmetic
        for i in 0..numbers.len() {
            let element_ptr = ptr.add(i);
            *element_ptr *= 10;
        }
        
        println!("After pointer arithmetic modification: {:?}", numbers);
    }
    
    // Working with uninitialized memory
    use std::mem::MaybeUninit;
    
    let mut data: [MaybeUninit<i32>; 5] = MaybeUninit::uninit_array();
    
    // Initialize the array
    for (i, elem) in data.iter_mut().enumerate() {
        elem.write(i as i32 * i as i32);
    }
    
    // Convert to initialized array
    let initialized_data: [i32; 5] = unsafe {
        MaybeUninit::array_assume_init(data)
    };
    
    println!("Initialized array: {:?}", initialized_data);
    
    // Manual memory allocation
    use std::alloc::{alloc, dealloc, Layout};
    
    unsafe {
        let layout = Layout::new::<i32>();
        let ptr = alloc(layout) as *mut i32;
        
        if ptr.is_null() {
            panic!("Failed to allocate memory");
        }
        
        // Use the allocated memory
        *ptr = 42;
        println!("Allocated and set value: {}", *ptr);
        
        // Don't forget to deallocate
        dealloc(ptr as *mut u8, layout);
        println!("Memory deallocated");
    }
    
    // Non-null pointers
    use std::ptr::NonNull;
    
    let mut value = 100;
    let non_null_ptr = NonNull::new(&mut value as *mut i32).expect("Pointer is null");
    
    unsafe {
        println!("NonNull pointer value: {}", *non_null_ptr.as_ptr());
        *non_null_ptr.as_ptr() = 200;
        println!("Modified through NonNull: {}", value);
    }
}

// Best practices demonstration
mod best_practices {
    use super::*;
    
    /// # Safety
    /// 
    /// The caller must ensure that:
    /// - `ptr` points to valid, properly aligned memory
    /// - The memory pointed to by `ptr` is not accessed by other code
    /// - The memory will remain valid for the duration of the returned slice
    pub unsafe fn slice_from_raw_parts_documented<'a, T>(
        ptr: *const T, 
        len: usize
    ) -> &'a [T] {
        slice::from_raw_parts(ptr, len)
    }
    
    pub fn safe_wrapper_example() {
        println!("\nBest practices for unsafe code:");
        
        let data = vec![1, 2, 3, 4, 5];
        let ptr = data.as_ptr();
        let len = data.len();
        
        // Using our documented unsafe function
        let slice = unsafe {
            slice_from_raw_parts_documented(ptr, len)
        };
        
        println!("Safe slice from unsafe function: {:?}", slice);
        
        // Always minimize unsafe blocks
        let value = 42;
        let ptr = &value as *const i32;
        
        // ✅ Good: minimal unsafe block
        let result = unsafe { *ptr };
        println!("Value from minimal unsafe block: {}", result);
        
        // Document why unsafe is needed
        /// This function demonstrates proper unsafe code documentation
        /// 
        /// # Safety
        /// This function is safe to call as long as the input vector
        /// has at least one element.
        fn get_first_element_unsafe<T>(vec: &Vec<T>) -> &T {
            unsafe {
                // SAFETY: We know the vector has at least one element
                // because we check the length before calling this function
                &*vec.as_ptr()
            }
        }
        
        let numbers = vec![10, 20, 30];
        if !numbers.is_empty() {
            let first = get_first_element_unsafe(&numbers);
            println!("First element (unsafe): {}", first);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_pointers() {
        let mut num = 5;
        let ptr = &mut num as *mut i32;
        
        unsafe {
            *ptr = 10;
        }
        
        assert_eq!(num, 10);
    }

    #[test]
    fn test_safe_split_at_mut() {
        let mut data = vec![1, 2, 3, 4, 5];
        let (left, right) = split_at_mut(&mut data, 2);
        
        assert_eq!(left, &[1, 2]);
        assert_eq!(right, &[3, 4, 5]);
        
        left[0] = 10;
        right[0] = 30;
        
        assert_eq!(data, vec![10, 2, 30, 4, 5]);
    }

    #[test]
    fn test_union_access() {
        let u = MyUnion { f1: 0x3f800000 }; // 1.0 in f32 representation
        
        unsafe {
            assert_eq!(u.f1, 0x3f800000);
            assert!((u.f2 - 1.0).abs() < f32::EPSILON);
        }
    }

    #[test]
    fn test_static_counter() {
        // Reset counter for test
        unsafe {
            COUNTER = 0;
        }
        
        add_to_count(5);
        assert_eq!(get_count(), 5);
        
        add_to_count(3);
        assert_eq!(get_count(), 8);
    }
}

// This function demonstrates calling all the examples
pub fn run_all_examples() {
    // This would be called from main() to run everything
    best_practices::safe_wrapper_example();
}