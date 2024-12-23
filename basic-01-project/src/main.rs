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

fn constants() {
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
}

fn main() {
    println!("----mutable---");
    mutable();
    println!("----redeclare----");
    redeclare();
    println!("----constants---");
    constants();

    // overflow
    let mut x: i8 = 10;

    // ! If you have run time logic compiler not going to check it
    for _i in 0..1000 {
        x = x + 100;
    }
    println!("x = {}", x)
}
