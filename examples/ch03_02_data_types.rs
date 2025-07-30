use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 3.2", "Data Types");
    
    println!("Hello, world!");

    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("guess: {}, x: {}, y: {}", guess, x, y);



    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;



    let _t = true;

    let _f: bool = false; // with explicit type annotation



    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';


    let tup: (i32, f64, u8) = (500, 6.4, 1);


    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");


    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;



    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];



}
