//! Chapter 19.4: Advanced Functions and Closures
//! 
//! This example demonstrates advanced function and closure concepts in Rust:
//! - Function pointers and the `fn` type
//! - Returning closures with `Box<dyn Fn>` and `impl Fn`
//! - Closure traits: `Fn`, `FnMut`, and `FnOnce`
//! - Higher-order functions and function composition
//! - Currying and partial application
//! - Function item types and their unique nature

use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 19.4", "Advanced Functions and Closures");
    
    function_pointers_demo();
    closure_traits_demo();
    returning_closures_demo();
    higher_order_functions_demo();
    function_item_types_demo();
    advanced_patterns_demo();
}

/// Demonstrates function pointers and their usage
fn function_pointers_demo() {
    println!("\n=== Function Pointers ===");
    
    // Function pointers implement all three closure traits
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    
    let answer = do_twice(add_one, 5);
    println!("do_twice(add_one, 5) = {}", answer); // 12
    
    // Function pointers can be used where closures are expected
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Using closure
    let strings1: Vec<String> = numbers.iter().map(|i| i.to_string()).collect();
    
    // Using method as function pointer  
    let strings2: Vec<String> = numbers.iter().map(ToString::to_string).collect();
    
    println!("With closure: {:?}", strings1);
    println!("With function pointer: {:?}", strings2);
    
    // Function pointer arrays for operation dispatch
    demonstrate_function_arrays();
}

/// Demonstrates storing functions in arrays for dispatch
fn demonstrate_function_arrays() {
    println!("\n--- Function Arrays ---");
    
    type BinaryOp = fn(i32, i32) -> i32;
    
    fn add(a: i32, b: i32) -> i32 { a + b }
    fn subtract(a: i32, b: i32) -> i32 { a - b }
    fn multiply(a: i32, b: i32) -> i32 { a * b }
    fn divide(a: i32, b: i32) -> i32 { a / b }
    
    let operations: [BinaryOp; 4] = [add, subtract, multiply, divide];
    let operation_names = ["+", "-", "*", "/"];
    
    let a = 10;
    let b = 5;
    
    for (op, name) in operations.iter().zip(operation_names.iter()) {
        let result = op(a, b);
        println!("{} {} {} = {}", a, name, b, result);
    }
}

/// Demonstrates the closure trait hierarchy
fn closure_traits_demo() {
    println!("\n=== Closure Traits ===");
    
    // Fn trait - immutable borrowing
    println!("\n--- Fn Trait ---");
    let name = String::from("Alice");
    let greet = || {
        println!("Hello, {}!", name); // Immutable borrow
    };
    
    call_fn(&greet);
    call_fn(&greet); // Can call multiple times
    println!("Original name still accessible: {}\n", name);
    
    // FnMut trait - mutable borrowing
    println!("--- FnMut Trait ---");
    let mut counter = 0;
    let mut increment = || {
        counter += 1; // Mutable borrow
        println!("Counter: {}", counter);
    };
    
    call_fn_mut(&mut increment);
    call_fn_mut(&mut increment);
    println!("Final counter: {}\n", counter);
    
    // FnOnce trait - taking ownership
    println!("--- FnOnce Trait ---");
    let name = String::from("Bob");
    let consume = move || {
        println!("Consuming: {}", name); // Takes ownership
        drop(name); // Explicitly consume
    };
    
    call_fn_once(consume);
    // consume(); // ❌ Would cause compile error - already consumed
    
    demonstrate_trait_hierarchy();
}

fn call_fn<F>(f: F) 
where F: Fn()
{
    f();
}

fn call_fn_mut<F>(f: &mut F) 
where F: FnMut()
{
    f();
}

fn call_fn_once<F>(f: F) 
where F: FnOnce()
{
    f();
}

/// Shows how closure traits form a hierarchy
fn demonstrate_trait_hierarchy() {
    println!("--- Trait Hierarchy ---");
    let x = 5;
    
    // This closure implements Fn (and therefore FnMut and FnOnce)
    let immutable_closure = || println!("x is: {}", x);
    
    // Can be used where any of the three traits are expected
    takes_fn(&immutable_closure);      // Fn
    takes_fn_mut(&immutable_closure);  // FnMut (coerced from Fn)
    takes_fn_once(immutable_closure);  // FnOnce (coerced from Fn)
}

fn takes_fn<F: Fn()>(f: &F) { 
    f(); 
    println!("Called as Fn");
}

fn takes_fn_mut<F: Fn()>(f: &F) { 
    f(); 
    println!("Called as FnMut (coerced from Fn)");
}

fn takes_fn_once<F: FnOnce()>(f: F) { 
    f(); 
    println!("Called as FnOnce (coerced from Fn)");
}

/// Demonstrates returning closures from functions
fn returning_closures_demo() {
    println!("\n=== Returning Closures ===");
    
    // Using Box<dyn Fn>
    println!("--- Using Box<dyn Fn> ---");
    let add_one = returns_boxed_closure();
    println!("5 + 1 = {}", add_one(5));
    
    let op = returns_conditional_closure(true);
    println!("10 + 1 = {}", op(10));
    
    let op = returns_conditional_closure(false);
    println!("10 - 1 = {}", op(10));
    
    // Using impl Fn
    println!("\n--- Using impl Fn ---");
    let add_one = returns_impl_closure();
    println!("5 + 1 = {}", add_one(5));
    
    let times_three = create_multiplier(3);
    println!("4 * 3 = {}", times_three(4));
    
    // Returning closures with captured state
    println!("\n--- Closures with State ---");
    let mut counter = create_counter();
    println!("Count: {}", counter()); // 1
    println!("Count: {}", counter()); // 2
    println!("Count: {}", counter()); // 3
    
    let mut acc = create_accumulator(10);
    println!("Accumulator: {}", acc(5));  // 15
    println!("Accumulator: {}", acc(3));  // 18
    println!("Accumulator: {}", acc(7));  // 25
}

fn returns_boxed_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_conditional_closure(increment: bool) -> Box<dyn Fn(i32) -> i32> {
    if increment {
        Box::new(|x| x + 1)
    } else {
        Box::new(|x| x - 1)
    }
}

fn returns_impl_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn create_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

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

/// Demonstrates higher-order functions and composition
fn higher_order_functions_demo() {
    println!("\n=== Higher-Order Functions ===");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    fn double(x: i32) -> i32 { x * 2 }
    fn add_ten(x: i32) -> i32 { x + 10 }
    
    let doubled = apply_operation(numbers.clone(), double);
    println!("Doubled: {:?}", doubled);
    
    let incremented = apply_operation(numbers.clone(), |x| x + 1);
    println!("Incremented: {:?}", incremented);
    
    // Function composition
    println!("\n--- Function Composition ---");
    let double_then_add_ten = compose(double, add_ten);
    println!("5 doubled then +10: {}", double_then_add_ten(5)); // 20
    
    // Currying and partial application
    println!("\n--- Currying and Partial Application ---");
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

fn curry_add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn curry_multiply(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x * y
}

fn partial_apply<F, A, B, C>(f: F, a: A) -> impl Fn(B) -> C
where
    F: Fn(A, B) -> C,
    A: Clone,
{
    move |b| f(a.clone(), b)
}

fn add(x: i32, y: i32) -> i32 { x + y }
fn multiply(x: i32, y: i32) -> i32 { x * y }

/// Demonstrates function item types and their unique nature
fn function_item_types_demo() {
    println!("\n=== Function Item Types ===");
    
    fn foo() { println!("foo called"); }
    fn bar() { println!("bar called"); }
    
    // Each function has its own unique type
    let f1 = foo;  // Type: fn() {foo}
    let f2 = bar;  // Type: fn() {bar}
    
    println!("Calling functions through variables:");
    f1();
    f2();
    
    // These are different types even though they have the same signature!
    // let array = [f1, f2];  // ❌ Won't compile - different types
    
    // Need to coerce to function pointer type
    let f1: fn() = foo;
    let f2: fn() = bar;
    let array = [f1, f2];  // ✅ Now they're the same type
    
    println!("Calling from array:");
    for func in &array {
        func();
    }
    
    println!("Each function creates different monomorphizations:");
    call_function(foo);     // F = fn() {foo}
    call_function(bar);     // F = fn() {bar}
}

fn call_function<F>(f: F) 
where 
    F: Fn(),
{
    f();
    println!("Function called via generic parameter");
}

/// Demonstrates advanced closure patterns
fn advanced_patterns_demo() {
    println!("\n=== Advanced Patterns ===");
    
    // Closure as state machine
    println!("--- State Machine Pattern ---");
    let mut machine = create_state_machine();
    
    println!("{}", machine(3));  // Started with 3
    println!("{}", machine(4));  // Continue with 7
    println!("{}", machine(5));  // Finished with 12
    println!("{}", machine(1));  // Already finished
    
    // Event handling with closures
    println!("\n--- Event Handling Pattern ---");
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

#[derive(Debug)]
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