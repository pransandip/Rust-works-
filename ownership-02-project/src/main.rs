// --------------------------- Ownership rules --------------------------------
// 1. Each value in rust has a variable that's called its owner, means one variable one owner.
// 2. There can only be one owner at a time, means variable can't have two owner at the same time.
// 3. When owner goes out of scope, the value will be dropped.
// 4. Rust has a copy trait a simple type stored on the stack such as (int boolean and char) this traits allows those types to be copied instead of move

// The Rules of References
// 1. At any given time, you can have either one mutable reference
// or any number of immutable reference.
//
// 2. References must always be valid.

fn main() {
    {
        let words = String::from("Ownership");
        takes_ownership(words); /* move */
        // println!("{}", words); // value borrowed here after move
    }

    {
        let x: u8 = 15;
        makes_copy(x); // int are copied here
        println!("x: {}", x)
    }

    {
        let s1: String = gives_ownership(); // returning the string moves the ownership to s1 variable
        println!("s1: {}", s1);
        let country_name = return_str(); // Now ownership is moved to country_name
        println!("country_name: {}", country_name);
    }

    {
        let s2: String = String::from("Hello");
        let s3: String = takes_and_gives_back_ownership(s2);
        println!("s3: {}", s3);
    }

    {
        let s4: String = String::from("World!");
        let len: usize = calculate_length(&s4); //  reference don't take ownerships
        println!("length of {} is {}", s4, len)
    }

    {
        let mut s5: String = String::from("Hi! ");
        change(&mut s5); //  mut reference don't take ownerships
        println!("s5: {}", s5);
        change_value();
    }

    /*  you can't have mut reference if immutable reference already exists */
    {
        let mut s: String = String::from("King");

        let r1: &String = &s;
        let r2: &String = &s;

        println!("r1: {}, r2: {}", r1, r2); // here r1 and r2 scope ends so we can declare mut reference

        let r3: &mut String = &mut s;
        println!("r3: {}", r3);
    }

    // can't have mutable and immutable reference at the same time
    // {
    //     let mut number: i32 = 10;
    //     let number_ref: &i32 = &number; // number_ref is expecting a unchanged data
    //     let number_change: &mut i32 = &mut number; // cannot borrow `number` as mutable because it is also borrowed as immutable
    //     *number_change += 10;
    //     println!("number_ref: {}", number_ref);
    // }

    // However, this code works
    {
        let mut number: u8 = 10;
        let number_change: &mut u8 = &mut number; // create a mutable reference
        *number_change += 10; // use mutable reference to add 10 and done
        let number_ref: &u8 = &number; // create an immutable reference
        println!("number_ref: {}", number_ref);
    }

    // It's asking to get a reference to data that does't exist anymore
    // * let country = return_str();
}

// Take ownership
fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string)
}

// Make Copy
fn makes_copy(some_integer: u8) {
    println!("makes_copy: {}", some_integer);
}

// Give ownership
fn gives_ownership() -> String {
    let some_string: String = String::from("Alex");
    some_string
}

fn return_str() -> String {
    let country: String = String::from("Austria");
    country
}

// Take and give back ownership
fn takes_and_gives_back_ownership(a_string: String) -> String {
    a_string // returning the string moves the ownership to s3 variable
}

// References
/* Moving ownership into a function and backout is tedious,
 * what if we just use a variable without taking ownership,
 * thats where reference come in.
 */

// Borrowing
/* Passing a reference as a function parameters it's called Borrowing
 * because we are borrowing value but we are not taking ownership of it.
 * references are immutable by default
 */
fn calculate_length(s: &String) -> usize {
    let length = s.len(); // len() returns size of a string in bytes
    length
}

// Mutable reference
fn change(some_string: &mut String) {
    some_string.push_str("Windows");
}

fn change_value() {
    let mut my_number: u8 = 10; // don't forget to write mut here
    let num_ref: &mut u8 = &mut my_number;

    *num_ref += 10;

    println!("my_number: {}", my_number);
}

// ! ---------------------  dangling references --------------------- //
// fn return_str() -> &'static str {
//     // this function owner of the country variable
//     let country: String = String::from("Austria"); // country starts here
//     let country_ref: &str = &country; // country_ref, referring a data that is about to be dropped
//     country_ref // reference to the data that is about to be dropped
// } // country ends here
// ! ------------------------------- End ---------------------------- //
