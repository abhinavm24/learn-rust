fn main() {
    let number = 6;

    if number < 10 {
        println!("Smaller than 10");
    } else {
        println!("Bigger than or equal to 10");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let number = if true { 5 } else { 6 };

    println!("The value of number is: {number}");


    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter < 10 {
            continue;
        } else if counter == 10 {
            break counter * 2;
        } else {
            break counter + 1;
        }
    };

    println!("The result is: {result}");


    // Loop with labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }


    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}
