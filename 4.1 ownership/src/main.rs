fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let s1 = s; //s out of scope with this operation and can't be used further

    println!("{}", s1);

    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let y = String::from("hello");  // y comes into scope

    takes_ownership(y); // y moves to funciton and is invalid after this

    let x = 5; // x comes into scope

    makes_copy(x); // x is i32 and has copy trait, so remains valid

    println!("{}", x);




    let s1 = gives_ownership();        // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3

    println!()

} // Here, x goes out of scope, then y. But because y's value was moved, nothing
// special happens.
// Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // some_integer goes out of scope, nothing special



fn gives_ownership() -> String {       // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}