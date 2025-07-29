# Chapter 18.3: Pattern Syntax

## Key Takeaways
- **Rich Pattern Syntax**: Rust provides extensive pattern matching capabilities
- **Literal Matching**: Match exact values directly
- **Variable Binding**: Capture values with named variables
- **Wildcards**: Use `_` to ignore values
- **Multiple Patterns**: Use `|` for OR logic
- **Range Patterns**: Match ranges of values
- **Destructuring**: Extract values from complex types
- **Pattern Guards**: Add conditions with `if`
- **@ Bindings**: Capture values while testing patterns

## Matching Literals

### Basic Literal Matching
```rust
fn main() {
    let x = 1;
    
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
```

### String and Character Literals
```rust
fn main() {
    let message = "hello";
    
    match message {
        "hello" => println!("Hello there!"),
        "goodbye" => println!("See you later!"),
        _ => println!("Unknown message"),
    }
    
    let character = 'x';
    match character {
        'a'..='z' => println!("lowercase letter"),
        'A'..='Z' => println!("uppercase letter"),
        '0'..='9' => println!("digit"),
        _ => println!("other character"),
    }
}
```

### Boolean Literals
```rust
fn main() {
    let is_active = true;
    
    match is_active {
        true => println!("System is active"),
        false => println!("System is inactive"),
    }
}
```

## Matching Named Variables

### Variable Binding in Match
```rust
fn main() {
    let x = Some(5);
    let y = 10;
    
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),  // This y shadows outer y
        _ => println!("Default case, x = {:?}", x),
    }
    
    println!("at the end: x = {:?}, y = {y}", x);  // outer y is still 10
}
```

### Shadowing in Patterns
```rust
fn main() {
    let x = Some(5);
    let y = 10;
    
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),  // Use guard to access outer y
        Some(n) => println!("Matched, n = {n}"),
        _ => println!("Default case"),
    }
}
```

## Multiple Patterns

### OR Patterns with |
```rust
fn main() {
    let x = 1;
    
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
```

### Complex OR Patterns
```rust
enum Message {
    Hello { id: i32 },
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Hello { id: 5 };
    
    match msg {
        Message::Hello { id: 5 | 6 | 7 } => {
            println!("Found an id in range 5, 6, or 7");
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id);
        }
        _ => (),
    }
}
```

## Matching Ranges with ..=

### Numeric Ranges
```rust
fn main() {
    let x = 5;
    
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}
```

### Character Ranges
```rust
fn main() {
    let x = 'c';
    
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
```

### Multiple Range Patterns
```rust
fn main() {
    let temperature = 75;
    
    match temperature {
        0..=32 => println!("Freezing"),
        33..=70 => println!("Cool"),
        71..=85 => println!("Warm"),
        86..=100 => println!("Hot"),
        _ => println!("Extreme temperature"),
    }
}
```

## Destructuring to Break Apart Values

### Destructuring Structs
```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    
    // Shorthand when variable names match field names
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```

### Struct Pattern Matching
```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
```

### Partial Struct Destructuring
```rust
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let point = Point3D { x: 1, y: 2, z: 3 };
    
    match point {
        Point3D { x, .. } => println!("x is {}", x),  // Ignore y and z
    }
    
    let Point3D { x, .. } = point;  // In let binding too
    println!("x: {}", x);
}
```

## Destructuring Enums

### Basic Enum Destructuring
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);
    
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
    }
}
```

### Nested Enum Destructuring
```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }
}
```

## Destructuring Nested Structs and Enums

### Complex Nested Destructuring
```rust
struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }
}
```

### Deeply Nested Patterns
```rust
struct Address {
    street: String,
    city: String,
    country: String,
}

struct Person {
    name: String,
    age: u8,
    address: Address,
}

enum Contact {
    Person(Person),
    Company { name: String, address: Address },
}

fn main() {
    let contact = Contact::Person(Person {
        name: String::from("Alice"),
        age: 25,
        address: Address {
            street: String::from("123 Main St"),
            city: String::from("Springfield"),
            country: String::from("USA"),
        },
    });
    
    match contact {
        Contact::Person(Person {
            name,
            address: Address { city, country, .. },
            ..
        }) => {
            println!("{} lives in {}, {}", name, city, country);
        }
        Contact::Company { name, .. } => {
            println!("Company: {}", name);
        }
    }
}
```

## Destructuring Tuples

### Basic Tuple Destructuring
```rust
fn main() {
    let tuple = (1, 2, 3);
    let (x, y, z) = tuple;
    println!("x: {}, y: {}, z: {}", x, y, z);
    
    match tuple {
        (0, y, z) => println!("First is 0, y: {}, z: {}", y, z),
        (1, ..) => println!("First is 1 and we don't care about the rest"),
        _ => println!("It doesn't matter what they are"),
    }
}
```

### Nested Tuple Destructuring
```rust
fn main() {
    let nested = ((1, 2), (3, 4));
    let ((a, b), (c, d)) = nested;
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
    
    match nested {
        ((0, b), (c, d)) => println!("First tuple starts with 0: b={}, c={}, d={}", b, c, d),
        ((a, 0), (c, d)) => println!("First tuple ends with 0: a={}, c={}, d={}", a, c, d),
        ((a, b), (0, d)) => println!("Second tuple starts with 0: a={}, b={}, d={}", a, b, d),
        ((a, b), (c, 0)) => println!("Second tuple ends with 0: a={}, b={}, c={}", a, b, c),
        ((a, b), (c, d)) => println!("No zeros: a={}, b={}, c={}, d={}", a, b, c, d),
    }
}
```

## Destructuring Arrays and Slices

### Array Destructuring
```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    
    match arr {
        [first, second, ..] => {
            println!("First: {}, Second: {}", first, second);
        }
    }
    
    match arr {
        [.., fourth, fifth] => {
            println!("Fourth: {}, Fifth: {}", fourth, fifth);
        }
    }
    
    match arr {
        [first, .., last] => {
            println!("First: {}, Last: {}", first, last);
        }
    }
    
    match arr {
        [1, 2, middle, 4, 5] => {
            println!("Middle element: {}", middle);
        }
        _ => println!("Pattern didn't match"),
    }
}
```

### Slice Pattern Matching
```rust
fn analyze_slice(slice: &[i32]) {
    match slice {
        [] => println!("Empty slice"),
        [only] => println!("One element: {}", only),
        [first, second] => println!("Two elements: {} and {}", first, second),
        [first, .., last] => println!("First: {}, Last: {}, {} elements total", 
                                    first, last, slice.len()),
    }
}

fn main() {
    analyze_slice(&[]);
    analyze_slice(&[1]);
    analyze_slice(&[1, 2]);
    analyze_slice(&[1, 2, 3, 4, 5]);
}
```

## Ignoring Values in a Pattern

### Using _ to Ignore Values
```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);
    
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }
}
```

### Ignoring Function Parameters
```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
```

### Ignoring Parts of a Value with Nested _
```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin = Point { x: 0, y: 0, z: 0 };
    
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}
```

### Using _ vs Variable Names Starting with _
```rust
fn main() {
    let _x = 5;  // Warning suppressed, but variable is still bound
    let y = 10;
    let _ = y;   // Completely ignored, no binding
    
    // println!("{}", _x);  // This works
    // println!("{}", _);   // This would cause a compile error
}
```

## Ignoring Remaining Parts with ..

### Using .. in Structs
```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin = Point { x: 0, y: 0, z: 0 };
    
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}
```

### Using .. in Tuples
```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);
    
    match numbers {
        (first, ..) => {
            println!("The first number is {}", first);
        }
    }
    
    match numbers {
        (.., last) => {
            println!("The last number is {}", last);
        }
    }
    
    match numbers {
        (first, .., last) => {
            println!("The first is {} and the last is {}", first, last);
        }
    }
}
```

### Invalid .. Usage
```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);
    
    // ❌ This is ambiguous and won't compile
    // match numbers {
    //     (.., second, ..) => {
    //         println!("The second number is {}", second);
    //     }
    // }
}
```

## Extra Conditionals with Match Guards

### Basic Match Guards
```rust
fn main() {
    let num = Some(4);
    
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}
```

### Match Guards with Multiple Patterns
```rust
fn main() {
    let x = 4;
    let y = false;
    
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
```

### Complex Match Guards
```rust
fn main() {
    let pair = (2, -2);
    
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 != 0 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}
```

### Match Guards with Outer Variables
```rust
fn main() {
    let x = Some(5);
    let y = 10;
    
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    
    println!("at the end: x = {:?}, y = {}", x, y);
}
```

## @ Bindings

### Basic @ Bindings
```rust
enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };
    
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range");
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
```

### @ Bindings with Complex Patterns
```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 5 };
    
    match point {
        p @ Point { x: 1..=5, y: 1..=10 } => {
            println!("Point {:?} is in the valid range", p);
        }
        Point { x, y } => {
            println!("Point ({}, {}) is outside valid range", x, y);
        }
    }
}
```

### @ Bindings in Nested Patterns
```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    ChangeColor(Color),
    Move { x: i32, y: i32 },
}

fn main() {
    let msg = Message::ChangeColor(Color::Rgb(255, 0, 0));
    
    match msg {
        Message::ChangeColor(color @ Color::Rgb(r, g, b)) if r > 200 => {
            println!("Bright red color: {:?} with r={}", color, r);
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("RGB color: ({}, {}, {})", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("HSV color: ({}, {}, {})", h, s, v);
        }
        Message::Move { x, y } => {
            println!("Move to ({}, {})", x, y);
        }
    }
}
```

## Advanced Pattern Combinations

### Combining Multiple Pattern Features
```rust
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

fn analyze_shape(shape: Shape) {
    match shape {
        // @ binding with guard
        s @ Shape::Circle { radius } if radius > 10.0 => {
            println!("Large circle: {:?}", s);
        }
        
        // Multiple patterns with guard
        Shape::Rectangle { width, height } | Shape::Triangle { base: width, height } 
            if width == height => {
            println!("Square-like shape with side {}", width);
        }
        
        // Destructuring with ranges
        Shape::Rectangle { width: 1.0..=5.0, height } => {
            println!("Narrow rectangle with height {}", height);
        }
        
        // Default cases
        Shape::Circle { radius } => {
            println!("Circle with radius {}", radius);
        }
        Shape::Rectangle { width, height } => {
            println!("Rectangle {}x{}", width, height);
        }
        Shape::Triangle { base, height } => {
            println!("Triangle with base {} and height {}", base, height);
        }
    }
}

fn main() {
    analyze_shape(Shape::Circle { radius: 15.0 });
    analyze_shape(Shape::Rectangle { width: 3.0, height: 3.0 });
    analyze_shape(Shape::Rectangle { width: 2.0, height: 8.0 });
    analyze_shape(Shape::Triangle { base: 4.0, height: 6.0 });
}
```

## Pattern Syntax Summary

### Pattern Types
- **Literals**: `1`, `"hello"`, `true`
- **Variables**: `x`, `name`
- **Wildcards**: `_`
- **Ranges**: `1..=5`, `'a'..='z'`
- **Multiple**: `1 | 2 | 3`
- **Tuples**: `(x, y, z)`
- **Arrays**: `[first, second, ..]`
- **Structs**: `Point { x, y }`
- **Enums**: `Some(x)`, `Message::Quit`
- **References**: `&x`, `&mut y`
- **Guards**: `x if x > 5`
- **@ Bindings**: `x @ 1..=5`

### Pattern Locations
- `match` expressions
- `if let` expressions  
- `while let` loops
- `for` loop variables
- `let` statements
- Function parameters
- Closure parameters

## Best Practices
1. **Be Exhaustive**: Handle all possible cases in match expressions
2. **Use Appropriate Patterns**: Choose the most specific pattern for clarity
3. **Avoid Complex Guards**: Keep match guards simple and readable
4. **Use @ Bindings Sparingly**: Only when you need both the value and the pattern test
5. **Prefer Destructuring**: Extract only the data you need
6. **Use .. Judiciously**: Don't ignore too many fields without good reason

Pattern syntax provides powerful and expressive ways to work with data in Rust, enabling concise and safe code.