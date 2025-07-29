# Chapter 19.3: Advanced Types

## Key Takeaways
- **Type Aliases**: Create synonyms for existing types using `type`
- **Never Type**: `!` represents computations that never return normally
- **Dynamically Sized Types (DST)**: Types whose size isn't known at compile time
- **Function Pointers**: `fn` type for function pointers vs closures
- **Trait Objects**: Dynamic dispatch with `dyn` keyword

## Type Aliases

### Basic Type Aliases

```rust
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    
    println!("x + y = {}", x + y);  // Works because both are i32
}
```

### Reducing Repetition with Type Aliases

```rust
// Before: Verbose type annotations
let f1: Box<dyn Fn(i32) -> i32 + Send + 'static> = Box::new(|x| x + 1);
let f2: Box<dyn Fn(i32) -> i32 + Send + 'static> = Box::new(|x| x * 2);

// After: Using type alias
type Processor = Box<dyn Fn(i32) -> i32 + Send + 'static>;

let f1: Processor = Box::new(|x| x + 1);
let f2: Processor = Box::new(|x| x * 2);
```

### Generic Type Aliases

```rust
type Result<T> = std::result::Result<T, std::io::Error>;

// Now can use Result<T> instead of Result<T, std::io::Error>
fn read_file() -> Result<String> {
    std::fs::read_to_string("hello.txt")
}

fn write_file(contents: &str) -> Result<()> {
    std::fs::write("output.txt", contents)
}
```

### Complex Type Aliases

```rust
use std::collections::HashMap;

type UserDatabase = HashMap<String, User>;
type UserResult<T> = Result<T, UserError>;
type AsyncOperation<T> = Box<dyn Future<Output = T> + Send + Unpin>;

#[derive(Debug)]
struct User {
    id: String,
    name: String,
    email: String,
}

#[derive(Debug)]
enum UserError {
    NotFound,
    InvalidEmail,
    DatabaseError,
}

use std::future::Future;

fn get_user_async(id: String) -> AsyncOperation<UserResult<User>> {
    Box::new(async move {
        // Simulate async database lookup
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        
        if id == "1" {
            Ok(User {
                id,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            })
        } else {
            Err(UserError::NotFound)
        }
    })
}
```

## The Never Type (!)

### Understanding Never Type

```rust
fn bar() -> ! {
    panic!("This function never returns!");
}

fn main() {
    let x: i32 = match some_condition() {
        true => 42,
        false => panic!("This never returns!"),  // ! can coerce to any type
    };
    
    println!("x is: {}", x);
}

fn some_condition() -> bool {
    true
}
```

### Never Type in Practice

```rust
fn process_number(n: i32) -> String {
    match n {
        1..=10 => format!("Small number: {}", n),
        11..=100 => format!("Medium number: {}", n),
        _ => {
            println!("Number too large!");
            std::process::exit(1);  // Returns !, coerces to String
        }
    }
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("Division by zero!");  // Returns !, coerces to f64
    } else {
        a / b
    }
}
```

### Never Type with Loop

```rust
fn server_loop() -> ! {
    loop {
        println!("Server is running...");
        std::thread::sleep(std::time::Duration::from_secs(1));
        // This loop never ends, so function returns !
    }
}

fn main() {
    let result: i32 = match std::env::args().len() {
        1 => 42,
        2 => 84,
        _ => server_loop(),  // ! coerces to i32
    };
    
    println!("Result: {}", result);
}
```

### Continue and Break with Never Type

```rust
fn main() {
    let mut i = 0;
    let x: i32 = loop {
        i += 1;
        if i < 5 {
            continue;  // ! type, but loop continues
        } else if i == 5 {
            break 42;  // Break with value of type i32
        }
    };
    
    println!("x: {}", x);
}
```

## Dynamically Sized Types (DST)

### Understanding DST

```rust
// These are DSTs - size not known at compile time
let s1: str = "Hello world";  // ❌ Can't do this directly
let s2: [i32] = [1, 2, 3, 4]; // ❌ Can't do this directly

// Must use them behind pointers
let s1: &str = "Hello world";     // ✅ Fat pointer (ptr + len)
let s2: &[i32] = &[1, 2, 3, 4];  // ✅ Fat pointer (ptr + len)
let s3: Box<str> = "Hello".into(); // ✅ Boxed DST
```

### Working with Slices (DST)

```rust
fn print_slice(s: &[i32]) {  // Takes DST behind reference
    println!("Slice length: {}", s.len());
    for item in s {
        println!("{}", item);
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let vec = vec![6, 7, 8, 9, 10];
    
    print_slice(&arr[..]);     // Array slice
    print_slice(&vec[..]);     // Vector slice
    print_slice(&arr[1..3]);   // Partial slice
}
```

### String Slices (DST)

```rust
fn print_str(s: &str) {  // str is a DST
    println!("String: '{}', length: {}", s, s.len());
    println!("Bytes: {:?}", s.as_bytes());
}

fn take_ownership(s: Box<str>) {  // Owned DST
    println!("Owned string: {}", s);
}

fn main() {
    let literal = "Hello, world!";       // &str
    let string = String::from("Hello");  // String
    
    print_str(literal);
    print_str(&string);
    print_str(&string[..3]);
    
    let boxed: Box<str> = string.into_boxed_str();
    take_ownership(boxed);
}
```

### Custom DST with Trait Objects

```rust
trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle {}x{}", self.width, self.height);
    }
}

fn draw_shape(shape: &dyn Draw) {  // dyn Draw is a DST
    shape.draw();
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 10.0, height: 20.0 };
    
    draw_shape(&circle);     // &dyn Draw
    draw_shape(&rectangle);  // &dyn Draw
    
    // Can also use Box<dyn Draw>
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle { width: 5.0, height: 8.0 }),
    ];
    
    for shape in &shapes {
        shape.draw();
    }
}
```

### Sized Trait

```rust
fn generic_function<T>(t: T) {
    // T is implicitly bounded by Sized: T: Sized
    println!("Function called");
}

fn explicit_sized<T: Sized>(t: T) {
    // Explicitly require T to be Sized
    println!("Function called");
}

fn relaxed_sizing<T: ?Sized>(t: &T) {
    // T may or may not be Sized
    // Must use T behind a pointer since it might be unsized
    println!("Function called");
}

fn main() {
    let x = 42;
    let s = "hello";
    
    generic_function(x);        // i32 is Sized
    explicit_sized(x);          // i32 is Sized
    relaxed_sizing(&x);         // &i32
    relaxed_sizing(s);          // &str (str is not Sized)
}
```

## Function Pointers

### Function Pointers vs Closures

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);  // 12
}
```

### Function Pointers with Closures

```rust
fn main() {
    let list_of_numbers = vec![1, 2, 3];
    
    // Using function pointer
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())  // Closure
        .collect();
    
    println!("{:?}", list_of_strings);
    
    // Using function name directly (function pointer)
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)  // Function pointer
        .collect();
    
    println!("{:?}", list_of_strings);
}
```

### Function Pointers vs Trait Objects

```rust
// Function pointer - specific function type
fn process_with_fn_ptr(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

// Trait object - any type implementing the trait
fn process_with_trait_object<F>(f: F, x: i32) -> i32 
where
    F: Fn(i32) -> i32,
{
    f(x)
}

fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    let closure = |x| x * 3;
    
    // Function pointer only accepts functions, not closures
    let result1 = process_with_fn_ptr(double, 5);
    // let result2 = process_with_fn_ptr(closure, 5);  // ❌ Won't compile
    
    // Trait object accepts both functions and closures
    let result3 = process_with_trait_object(double, 5);
    let result4 = process_with_trait_object(closure, 5);
    
    println!("Results: {}, {}, {}", result1, result3, result4);
}
```

### Storing Function Pointers

```rust
type Operation = fn(i32, i32) -> i32;

fn add(a: i32, b: i32) -> i32 { a + b }
fn multiply(a: i32, b: i32) -> i32 { a * b }
fn subtract(a: i32, b: i32) -> i32 { a - b }

struct Calculator {
    operations: std::collections::HashMap<String, Operation>,
}

impl Calculator {
    fn new() -> Self {
        let mut operations = std::collections::HashMap::new();
        operations.insert("add".to_string(), add as Operation);
        operations.insert("multiply".to_string(), multiply as Operation);
        operations.insert("subtract".to_string(), subtract as Operation);
        
        Calculator { operations }
    }
    
    fn calculate(&self, op: &str, a: i32, b: i32) -> Option<i32> {
        self.operations.get(op).map(|f| f(a, b))
    }
}

fn main() {
    let calc = Calculator::new();
    
    println!("3 + 4 = {:?}", calc.calculate("add", 3, 4));
    println!("3 * 4 = {:?}", calc.calculate("multiply", 3, 4));
    println!("3 - 4 = {:?}", calc.calculate("subtract", 3, 4));
}
```

## Returning Closures

### Problem with Returning Closures

```rust
// ❌ This won't compile
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}

// ❌ This also won't compile - size not known
fn returns_closure_v2() -> Box<dyn Fn(i32) -> i32> {
    |x| x + 1
}
```

### Solutions for Returning Closures

```rust
// ✅ Using Box with trait object
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// ✅ Using impl Trait (Rust 2018+)
fn returns_closure_impl() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

// ✅ Conditional return with Box
fn returns_conditional_closure(condition: bool) -> Box<dyn Fn(i32) -> i32> {
    if condition {
        Box::new(|x| x + 1)
    } else {
        Box::new(|x| x * 2)
    }
}

fn main() {
    let f1 = returns_closure();
    let f2 = returns_closure_impl();
    let f3 = returns_conditional_closure(true);
    let f4 = returns_conditional_closure(false);
    
    println!("f1(5) = {}", f1(5));  // 6
    println!("f2(5) = {}", f2(5));  // 6
    println!("f3(5) = {}", f3(5));  // 6
    println!("f4(5) = {}", f4(5));  // 10
}
```

### Closure Factory Pattern

```rust
fn create_multiplier(multiplier: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x * multiplier)
}

fn create_adder(addend: i32) -> impl Fn(i32) -> i32 {
    move |x| x + addend
}

fn main() {
    let times_two = create_multiplier(2);
    let times_three = create_multiplier(3);
    let add_ten = create_adder(10);
    
    println!("times_two(5) = {}", times_two(5));    // 10
    println!("times_three(4) = {}", times_three(4)); // 12
    println!("add_ten(5) = {}", add_ten(5));        // 15
}
```

## Advanced Type Patterns

### Type-Level Programming

```rust
// Phantom types for type safety
use std::marker::PhantomData;

struct Celsius;
struct Fahrenheit;

#[derive(Debug)]
struct Temperature<Scale> {
    degrees: f64,
    _phantom: PhantomData<Scale>,
}

impl Temperature<Celsius> {
    fn new_celsius(degrees: f64) -> Self {
        Temperature {
            degrees,
            _phantom: PhantomData,
        }
    }
    
    fn to_fahrenheit(self) -> Temperature<Fahrenheit> {
        Temperature {
            degrees: self.degrees * 9.0 / 5.0 + 32.0,
            _phantom: PhantomData,
        }
    }
}

impl Temperature<Fahrenheit> {
    fn new_fahrenheit(degrees: f64) -> Self {
        Temperature {
            degrees,
            _phantom: PhantomData,
        }
    }
    
    fn to_celsius(self) -> Temperature<Celsius> {
        Temperature {
            degrees: (self.degrees - 32.0) * 5.0 / 9.0,
            _phantom: PhantomData,
        }
    }
}

fn main() {
    let temp_c = Temperature::<Celsius>::new_celsius(25.0);
    let temp_f = temp_c.to_fahrenheit();
    
    println!("Temperature: {:?}", temp_f);
    
    let temp_c2 = temp_f.to_celsius();
    println!("Back to Celsius: {:?}", temp_c2);
}
```

### Zero-Sized Types

```rust
use std::marker::PhantomData;

// Zero-sized type for compile-time checks
struct Locked;
struct Unlocked;

struct Door<State> {
    _state: PhantomData<State>,
}

impl Door<Locked> {
    fn new() -> Door<Locked> {
        Door { _state: PhantomData }
    }
    
    fn unlock(self) -> Door<Unlocked> {
        println!("Door unlocked");
        Door { _state: PhantomData }
    }
}

impl Door<Unlocked> {
    fn lock(self) -> Door<Locked> {
        println!("Door locked");
        Door { _state: PhantomData }
    }
    
    fn open(&self) {
        println!("Door opened");
    }
}

fn main() {
    let door = Door::new();          // Door<Locked>
    // door.open();                  // ❌ Can't open locked door
    
    let door = door.unlock();        // Door<Unlocked>
    door.open();                     // ✅ Can open unlocked door
    
    let door = door.lock();          // Door<Locked>
    // door.open();                  // ❌ Can't open locked door again
}
```

### Associated Type Projections

```rust
trait Container {
    type Item;
    type Iter: Iterator<Item = Self::Item>;
    
    fn iter(&self) -> Self::Iter;
}

struct MyVec<T> {
    items: Vec<T>,
}

impl<T> Container for MyVec<T> 
where
    T: Clone,
{
    type Item = T;
    type Iter = std::vec::IntoIter<T>;
    
    fn iter(&self) -> Self::Iter {
        self.items.clone().into_iter()
    }
}

fn process_container<C>(container: &C) 
where
    C: Container,
    C::Item: std::fmt::Debug,  // Associated type projection
{
    for item in container.iter() {
        println!("Item: {:?}", item);
    }
}

fn main() {
    let my_vec = MyVec {
        items: vec![1, 2, 3, 4, 5],
    };
    
    process_container(&my_vec);
}
```

Advanced types in Rust provide powerful abstractions while maintaining zero-cost principles and compile-time safety guarantees.