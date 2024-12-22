// ------------ Ownership rules -------------
// 1. Each value in rust has a variable that's called its owner, means one variable one owner.
// 2. There can only be one owner at a time, means variable can't have two owner at the same time.
// 3. When owner goes out of scope, the value will be droped.
// 4. Rust has a copy trait a simple type stored on the stack such as (int boolean and char) this traits allows those types to be copied instade of move

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
        println!("s5: {}", s5)
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
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string)
}

fn makes_copy(some_integer: u8) {
    println!("makes_copy: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string: String = String::from("Alex");
    some_string
}

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
