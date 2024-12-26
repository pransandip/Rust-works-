// ------------ Ownership rules -------------
// 1. Each value in rust has a variable that's called its owner, means one variable one owner.
// 2. There can only be one owner at a time, means variable can't have two owner at the same time.
// 3. When owner goes out of scope, the value will be dropped.
// 4. Rust has a copy trait, a simple type stored on the stack such as (int boolean and char) this
//    traits allows those types to be copied instead of move

// The Rules of References
// 1. At any given time, you can have either one mutable reference
// or any number of immutable reference.
// 2. References must always be valid.

// OWNERSHIP = who owns a value

fn main() {
    {
        let x: i32 = 5;
        let _y: i32 = x; // Copy

        {
            // s is not valid here, it's not yet declared
            let _s: &str = "Hello"; // s is valid from this point forward
                                    // do stuff with s, and s is a string literal it's a fixed in size
                                    // string literals directly stores in binary
        } // this scope is now over, and s is no longer valid

        let txt1: String = String::from("Sandy!"); // txt1 is String type and dynamic in size stored in heap memory
        let txt2: String = txt1; // Move, (not shallow copy) txt1 will be dropped (rust invalidate the txt1)
        let txt3: String = txt2.clone(); // Rust defaults moving a value, if you want to clone a value there is a method call clone()
        println!("txt2: {} txt3: {}", txt2, txt3);
    }

    {
        let words = String::from("ownership");
        takes_ownership(words);
        // println!("{}", words); // value borrowed here after move
    }

    {
        let x: i32 = 15;
        makes_copy(x);
        println!("makes_copy: {}", x)
    }

    {
        let s1: String = gives_ownership();
        let s2: String = String::from("drinks");
        let s3: String = takes_and_gives_back_ownership(s2);
        println!("{} {}", s1, s3);
    }
    {
        let s1: String = String::from("Sandip!");
        let len: usize = calculate_length(&s1);
        println!("length of {} is {} bytes", s1, len)
    }
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string)
}

fn makes_copy(some_integer: i32) {
    println!("makes_copy: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("cola");
    return some_string;
}

fn takes_and_gives_back_ownership(a_string: String) -> String {
    a_string
}

// References
/* Moving ownership into a function and backout is tedious.
 * what if we just use a variable without taking ownership
 * thats where reference come in.
 * reference don't take ownerships
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
