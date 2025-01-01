// GENERICS (in functions)
// generics = can be one type, can be another type
// generic = concrete = one type

// Till now we only used functions with concrete types like i32, f64, etc.

// T, U, V - one capital letter in rust mean generic
// monomorphic = make into one form
// polymorphic = more then one form

use std::fmt::Display;

// This is a type that doesn't implement Display
// struct Book {
//     number: u8,
// }

fn return_thing<T>(thing: T) -> T {
    return thing;
}

// Get the type of a given variable,
// return a string representation of the type
fn type_of<T>(_: T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// This is a grantee to rust that T is going to be Type that can display. its not giving the type Display
// We are not giving T the power to display, we are saying that,
// any type that we give you is going to be a type that implements display trait.
// (T must have the power to display) any thing that can display can be passed.
fn print_number<T: Display>(number: T) {
    println!("Here is Number: {}", number);
}

fn main() {
    println!("Return things is: {}", return_thing("Gold"));

    println!(
        "Type of given variable in string: {}",
        type_of(return_thing(10))
    ); // "i32"

    print_number(8); // i32 = already has (implemented) Display
    print_number(8.9); // f64 = already has (implemented) Display

    // let book = Book {
    //     // This is a type that doesn't implement Display
    //     number: 10,
    // };
    // print_number(book);
}
