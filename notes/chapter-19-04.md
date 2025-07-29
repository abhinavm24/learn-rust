# Chapter 19.4: Advanced Functions and Closures

## Key Takeaways
- **Function Pointers**: `fn` type for function pointers that implement closure traits
- **Returning Closures**: Use `Box<dyn Fn>` or `impl Fn` to return closures
- **Closure Traits**: `Fn`, `FnMut`, and `FnOnce` hierarchy and when to use each
- **Function Item Types**: Each function has its own unique type
- **Higher-Order Functions**: Functions that take or return other functions

## Function Pointers Deep Dive

### Basic Function Pointers

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer); // 12
    
    // Function pointers implement all three closure traits
    let f: fn(i32) -> i32 = add_one;
    let result1 = call_fn(f, 10);      // Fn trait
    let result2 = call_fn_mut(f, 10);  // FnMut trait  
    let result3 = call_fn_once(f, 10); // FnOnce trait
    
    println!("Results: {}, {}, {}", result1, result2, result3);
}

fn call_fn<F>(f: F, x: i32) -> i32 
where F: Fn(i32) -> i32 
{
    f(x)
}

fn call_fn_mut<F>(mut f: F, x: i32) -> i32 
where F: FnMut(i32) -> i32 
{
    f(x)
}

fn call_fn_once<F>(f: F, x: i32) -> i32 
where F: FnOnce(i32) -> i32 
{
    f(x)
}
```

### Function Pointers vs Closures

```rust
fn main() {
    let list_of_numbers = vec![1, 2, 3];
    
    // Method 1: Using closure
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();
    
    // Method 2: Using method as function pointer
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)  // Function pointer
        .collect();
        
    // Method 3: Using function name directly
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| ToString::to_string(i))
        .collect();
    
    println!("{:?}", list_of_strings);
}
```

### Function Pointer Arrays and Collections

```rust
type BinaryOp = fn(i32, i32) -> i32;

fn add(a: i32, b: i32) -> i32 { a + b }
fn subtract(a: i32, b: i32) -> i32 { a - b }
fn multiply(a: i32, b: i32) -> i32 { a * b }
fn divide(a: i32, b: i32) -> i32 { a / b }

fn main() {
    let operations: [BinaryOp; 4] = [add, subtract, multiply, divide];
    
    let a = 10;
    let b = 5;
    
    for (i, op) in operations.iter().enumerate() {
        let result = op(a, b);
        match i {
            0 => println!("{} + {} = {}", a, b, result),
            1 => println!("{} - {} = {}", a, b, result),
            2 => println!("{} * {} = {}", a, b, result),
            3 => println!("{} / {} = {}", a, b, result),
            _ => unreachable!(),
        }
    }
}
```

## Understanding Closure Traits

### Fn Trait - Immutable Borrowing

```rust
fn call_fn<F>(f: F) 
where
    F: Fn(),
{
    f();  // Can call multiple times
    f();
    f();
}

fn main() {
    let name = String::from("Alice");
    
    // Closure that borrows immutably
    let greet = || {
        println!("Hello, {}!", name);  // Immutable borrow
    };
    
    call_fn(greet);  // Can use multiple times
    println!("Original name: {}", name);  // name still accessible
}
```

### FnMut Trait - Mutable Borrowing

```rust
fn call_fn_mut<F>(mut f: F) 
where
    F: FnMut(),
{
    f();  // Can call multiple times
    f();
    f();
}

fn main() {
    let mut counter = 0;
    
    // Closure that mutably borrows
    let mut increment = || {
        counter += 1;  // Mutable borrow
        println!("Counter: {}", counter);
    };
    
    call_fn_mut(increment);
    println!("Final counter: {}", counter);
}
```

### FnOnce Trait - Taking Ownership

```rust
fn call_fn_once<F>(f: F) 
where
    F: FnOnce(),
{
    f();  // Can only call once
    // f();  // ❌ Would cause compile error
}

fn main() {
    let name = String::from("Bob");
    
    // Closure that takes ownership
    let consume = move || {
        println!("Consuming: {}", name);  // Takes ownership
        drop(name);  // Explicitly consume
    };
    
    call_fn_once(consume);
    // println!("{}", name);  // ❌ name has been moved
}
```

### Trait Hierarchy and Automatic Implementation

```rust
// Every Fn implements FnMut
// Every FnMut implements FnOnce
// But not vice versa

fn demonstrate_hierarchy() {
    let x = 5;
    
    // This closure implements Fn (and therefore FnMut and FnOnce)
    let immutable_closure = || println!("x is: {}", x);
    
    // Can be used where any of the three traits are expected
    takes_fn(&immutable_closure);      // Fn
    takes_fn_mut(&immutable_closure);  // FnMut (coerced from Fn)
    takes_fn_once(immutable_closure);  // FnOnce (coerced from Fn)
}

fn takes_fn<F: Fn()>(f: &F) { f(); }
fn takes_fn_mut<F: FnMut()>(f: &F) { }  // Note: taking &F, not mut F
fn takes_fn_once<F: FnOnce()>(f: F) { f(); }
```

## Returning Closures

### Using Box<dyn Fn>

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_conditional_closure(increment: bool) -> Box<dyn Fn(i32) -> i32> {
    if increment {
        Box::new(|x| x + 1)
    } else {
        Box::new(|x| x - 1)
    }
}

fn main() {
    let add_one = returns_closure();
    println!("5 + 1 = {}", add_one(5));
    
    let op = returns_conditional_closure(true);
    println!("10 + 1 = {}", op(10));
    
    let op = returns_conditional_closure(false);
    println!("10 - 1 = {}", op(10));
}
```

### Using impl Fn

```rust
fn returns_closure_impl() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn create_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

// ❌ Can't return different types with impl Fn
// fn returns_conditional_impl(increment: bool) -> impl Fn(i32) -> i32 {
//     if increment {
//         |x| x + 1  // Type A
//     } else {
//         |x| x - 1  // Type B - different type!
//     }
// }

fn main() {
    let add_one = returns_closure_impl();
    println!("5 + 1 = {}", add_one(5));
    
    let times_three = create_multiplier(3);
    println!("4 * 3 = {}", times_three(4));
}
```

### Returning Closures with Captured Variables

```rust
fn create_counter() -> Box<dyn FnMut() -> i32> {
    let mut count = 0;
    Box::new(move || {
        count += 1;
        count
    })
}

fn create_accumulator(initial: i32) -> Box<dyn FnMut(i32) -> i32> {
    let mut total = initial;
    Box::new(move |x| {
        total += x;
        total
    })
}

fn main() {
    let mut counter = create_counter();
    println!("Count: {}", counter()); // 1
    println!("Count: {}", counter()); // 2
    println!("Count: {}", counter()); // 3
    
    let mut acc = create_accumulator(10);
    println!("Accumulator: {}", acc(5));  // 15
    println!("Accumulator: {}", acc(3));  // 18
    println!("Accumulator: {}", acc(7));  // 25
}
```

## Higher-Order Functions

### Functions That Take Functions

```rust
fn apply_operation<F>(vec: Vec<i32>, op: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(op).collect()
}

fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn double(x: i32) -> i32 { x * 2 }
fn add_ten(x: i32) -> i32 { x + 10 }

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    let doubled = apply_operation(numbers.clone(), double);
    println!("Doubled: {:?}", doubled);
    
    let incremented = apply_operation(numbers.clone(), |x| x + 1);
    println!("Incremented: {:?}", incremented);
    
    // Function composition
    let double_then_add_ten = compose(double, add_ten);
    println!("5 doubled then +10: {}", double_then_add_ten(5)); // 20
}
```

### Currying and Partial Application

```rust
fn curry_add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn curry_multiply(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x * y
}

fn partial_apply<F, A, B, C>(f: F, a: A) -> impl Fn(B) -> C
where
    F: Fn(A, B) -> C,
{
    move |b| f(a, b)
}

fn add(x: i32, y: i32) -> i32 { x + y }
fn multiply(x: i32, y: i32) -> i32 { x * y }

fn main() {
    let add_five = curry_add(5);
    println!("10 + 5 = {}", add_five(10));
    
    let times_three = curry_multiply(3);
    println!("7 * 3 = {}", times_three(7));
    
    // Partial application
    let add_ten = partial_apply(add, 10);
    println!("15 + 10 = {}", add_ten(15));
    
    let multiply_by_four = partial_apply(multiply, 4);
    println!("6 * 4 = {}", multiply_by_four(6));
}
```

## Function Item Types

### Understanding Function Item Types

```rust
fn foo() {}
fn bar() {}

fn main() {
    // Each function has its own unique type
    let f1 = foo;  // Type: fn() {foo}
    let f2 = bar;  // Type: fn() {bar}
    
    // These are different types even though they have the same signature!
    // let array = [f1, f2];  // ❌ Won't compile - different types
    
    // Need to coerce to function pointer type
    let f1: fn() = foo;
    let f2: fn() = bar;
    let array = [f1, f2];  // ✅ Now they're the same type
    
    for func in &array {
        func();
    }
}
```

### Generic Functions with Function Items

```rust
fn call_function<F>(f: F) 
where 
    F: Fn(),
{
    f();
}

fn greet() {
    println!("Hello!");
}

fn farewell() {
    println!("Goodbye!");
}

fn main() {
    // Each function has a unique zero-sized type
    call_function(greet);     // F = fn() {greet}
    call_function(farewell);  // F = fn() {farewell}
    
    // These create different monomorphized versions of call_function
}
```

## Advanced Closure Patterns

### Closure as State Machine

```rust
enum State {
    Start,
    Middle(i32),
    End,
}

fn create_state_machine() -> Box<dyn FnMut(i32) -> String> {
    let mut state = State::Start;
    
    Box::new(move |input| {
        match state {
            State::Start => {
                state = State::Middle(input);
                format!("Started with {}", input)
            }
            State::Middle(ref mut value) => {
                *value += input;
                if *value > 10 {
                    let result = format!("Finished with {}", *value);
                    state = State::End;
                    result
                } else {
                    format!("Continue with {}", *value)
                }
            }
            State::End => {
                "Already finished".to_string()
            }
        }
    })
}

fn main() {
    let mut machine = create_state_machine();
    
    println!("{}", machine(3));  // Started with 3
    println!("{}", machine(4));  // Continue with 7
    println!("{}", machine(5));  // Finished with 12
    println!("{}", machine(1));  // Already finished
}
```

### Closure for Event Handling

```rust
struct EventHandler {
    handlers: Vec<Box<dyn FnMut(&str)>>,
}

impl EventHandler {
    fn new() -> Self {
        EventHandler {
            handlers: Vec::new(),
        }
    }
    
    fn add_handler<F>(&mut self, handler: F)
    where
        F: FnMut(&str) + 'static,
    {
        self.handlers.push(Box::new(handler));
    }
    
    fn trigger_event(&mut self, event: &str) {
        for handler in &mut self.handlers {
            handler(event);
        }
    }
}

fn main() {
    let mut event_handler = EventHandler::new();
    
    // Add different types of handlers
    event_handler.add_handler(|event| {
        println!("Logger: {}", event);
    });
    
    let mut counter = 0;
    event_handler.add_handler(move |_event| {
        counter += 1;
        println!("Event count: {}", counter);
    });
    
    event_handler.add_handler(|event| {
        if event.contains("error") {
            println!("ERROR ALERT: {}", event);
        }
    });
    
    // Trigger events
    event_handler.trigger_event("user login");
    event_handler.trigger_event("data processed");
    event_handler.trigger_event("error occurred");
}
```

### Memoization with Closures

```rust
use std::collections::HashMap;

fn memoize<F, Arg, Ret>(mut f: F) -> impl FnMut(Arg) -> Ret
where
    F: FnMut(Arg) -> Ret,
    Arg: Clone + std::hash::Hash + Eq,
    Ret: Clone,
{
    let mut cache = HashMap::new();
    
    move |arg| {
        if let Some(result) = cache.get(&arg) {
            result.clone()
        } else {
            let result = f(arg.clone());
            cache.insert(arg, result.clone());
            result
        }
    }
}

fn expensive_fibonacci(n: u64) -> u64 {
    println!("Computing fibonacci({})", n);  // To show when it's actually computed
    match n {
        0 => 0,
        1 => 1,
        _ => expensive_fibonacci(n - 1) + expensive_fibonacci(n - 2),
    }
}

fn main() {
    let mut memoized_fib = memoize(expensive_fibonacci);
    
    println!("Result: {}", memoized_fib(10));  // Computes and caches
    println!("Result: {}", memoized_fib(10));  // Returns cached result
    println!("Result: {}", memoized_fib(11));  // Uses cached 10, computes 9 and down
}
```

## Performance Considerations

### Zero-Cost Abstractions

```rust
// These are equivalent in performance
fn manual_loop(vec: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 0..vec.len() {
        sum += vec[i] * 2;
    }
    sum
}

fn iterator_closure(vec: Vec<i32>) -> i32 {
    vec.into_iter()
        .map(|x| x * 2)  // Closure gets inlined
        .sum()
}

fn iterator_function_pointer(vec: Vec<i32>) -> i32 {
    fn double(x: i32) -> i32 { x * 2 }
    
    vec.into_iter()
        .map(double)  // Function pointer, no overhead
        .sum()
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    println!("Manual: {}", manual_loop(data.clone()));
    println!("Closure: {}", iterator_closure(data.clone()));
    println!("Function: {}", iterator_function_pointer(data));
}
```

### Avoiding Allocations

```rust
// ❌ Allocates on heap
fn returns_boxed_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// ✅ No allocation, but can't return different closure types
fn returns_impl_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

// ✅ Use function pointers when possible - zero overhead
fn add_one(x: i32) -> i32 { x + 1 }

fn returns_function_pointer() -> fn(i32) -> i32 {
    add_one
}

fn main() {
    let f1 = returns_boxed_closure();    // Heap allocation
    let f2 = returns_impl_closure();     // Zero-sized type
    let f3 = returns_function_pointer(); // Function pointer
    
    println!("{}, {}, {}", f1(5), f2(5), f3(5));
}
```

Advanced functions and closures provide powerful abstractions for functional programming patterns while maintaining Rust's performance guarantees through zero-cost abstractions and compile-time optimizations.