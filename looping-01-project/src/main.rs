use std::ops::{Range, RangeInclusive};

fn check_equal() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
    println!("success!");
}

fn print_numbers() {
    for i in 0..10 {
        println!("{}", i);
    }

    // print -3 to 2 where 2 is exclusive
    let mut sum: i32 = 0;
    for i in -3..2 {
        sum += i
    }
    assert!(sum == -5);
    println!("sum is: {}", sum);
}

// print char in decimal format
fn print_char() {
    // print a to z where z is inclusive
    for c in 'a'..='z' {
        println!("{}", c as u8);
    }
}

// Control Flow
fn looping() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("The result is {}", result);
}

fn conditional_loop() {
    let mut number: u8 = 3;

    while number != 0 {
        println!("number is : {}", number);

        number -= 1;
    }

    println!("LIFTOFF!!!")
}

fn array_looping() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}

fn main() {
    check_equal();
    print_numbers();
    print_char();
    looping();
    conditional_loop();
    array_looping();
}
