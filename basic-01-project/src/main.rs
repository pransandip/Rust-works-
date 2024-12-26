/*
* Primitive dataTypes: Basic/Fundamental data type used to declare a value
* Primitive data types has two category: 1.) Scalar type 2.) Compound type
* Scalar: A scalar type represents a single value. Rust has four primary
* scalar types: integers, floating-point numbers, Booleans, and characters.
* Compound: compound types can group multiple values into one type.
* Rust has two primitive compound types: tuples and arrays.
*/

// 1. Tuple is fixed length sequence of elements that is immutable
// 2. arrays have to have same element inside

// 1. You have four spaces before println!()
// 2. You have explicit and implicit type
// 3. variables are immutable
// 4. const can't be redefine

/* Shadowing: when you shadow a variable, you don't destroy it. you block it. */

/* only u8 as a cast as a 'char', not i32 */
/* usize = 64 bits *if possible* - if not, 32 bit */

// const == is a value that doesn't change.
// static == is a value that does not change and has a fixed memory location.

// Rust won't use type inference; you need to write type for them
// You write them with ALL CAPITAL LETTERS, and usually outside of
// main so that they can live for the whole program

fn mutable() {
    let mut x = 4;
    println!("x is: {}", x);
    x = 5;
    println!("x is: {}", x as u8 as char); // double cast
}

fn redeclare() {
    let y = 222;
    println!("y is: {}", y);
    {
        // scope changed
        let y = y - 2;
        println!("y is: {}", y);
    }
    let y = y + 2;
    println!("y is: {}", y as u8);
}

// How shadowing works
fn shadowing_ex() {
    let country: String = String::from("Japan");
    let country_ref: &String = &country;
    let country = 8; // It will not kill country it will blocks the previous value we can check it by country_ref
    println!("country: {}, country_ref: {}", country, country_ref);
}

fn constants() {
    const SECONDS_IN_MINUTE: u32 = 60;
    static SEASONS: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];

    println!("{}", SECONDS_IN_MINUTE);
    println!("{:?}", SEASONS);
}

// give ownership to the function (move) and return it back
fn print_country(country_name: String) -> String {
    println!("Country: {}", country_name);
    country_name
}

fn country() {
    let country: String = String::from("Japan");
    let country: String = print_country(country);
    print_country(country);
    //print_country(country); // use of moved value: `country`
}

// give reference to the function (borrowing)
fn print_country_2(country_name: &String) {
    println!("Country: {}", country_name);
}

fn country_2() {
    let country: String = String::from("Korea");
    print_country_2(&country);
    print_country_2(&country);
}

// Giving mutable references to functions
fn add_and_print_hungary(country_name: &mut String) {
    country_name.push_str("-Hungary");
    println!("Now it is says: {}", country_name);
}

fn mutable_country() {
    let mut country: String = String::from("Budapest");
    add_and_print_hungary(&mut country);
}

fn adds_hungary(mut country: String) {
    // This func has now full control over variable making it mutable
    country.push_str("-Hungary");
    println!("Country: {}", country);
}

fn print_hungary() {
    let country: String = String::from("Austria");
    adds_hungary(country.clone());
    println!("Country: {}", country);
}

fn main() {
    println!("----mutable----");
    mutable();
    println!("----redeclare----");
    redeclare();
    println!("----shadowing----");
    shadowing_ex();
    println!("----constants----");
    constants();
    println!("----country-----");
    country();
    println!("----country_2-----");
    country_2();
    println!("----mutable_country-----");
    mutable_country();
    print_hungary();

    // // overflow
    // let mut x: i8 = 10;

    // // ! If you have run time logic compiler not going to check it
    // for _i in 0..1000 {
    //     x = x + 100;
    // }
    // println!("x = {}", x)
}
