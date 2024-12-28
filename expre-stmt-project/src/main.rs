/*
 * Function, Expressions and Statement
 * expression perform some action and returns a value.
 * statement perform some action but do not return a value.
 */

fn a() {
    // expression
    let number = {
        let x = 3;
        x + 1
    };
    println!("number: {}", number);
}

// expression
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

fn b() {
    let result = add_numbers(20, 30); // expression
    println!("result: {}", result);
}

// The trait `std::fmt::Display` is not implemented for `()`
// () does not have the power of Display
// Trait = powers for types
fn c() {
    // statement
    let my_num = {
        let _second_num = 9;
    };

    // Debug printing
    println!("my_num: {:?}", my_num);
    // println!("my_num: {:#?}", my_num); // pretty printing
}

fn main() {
    a();
    b();
    c();
}
