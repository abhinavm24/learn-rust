use rust_book_examples::print_chapter_header;

fn main() {
    print_chapter_header("Chapter 3.3", "Functions");
    
    println!("Hello, world!");

    another_function(12);

    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {x}");

    let y = plus_two(x);
    println!("The value of y is: {y}");
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_two(x: i32) -> i32 {
    x + 2
}