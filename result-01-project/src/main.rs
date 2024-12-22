//-------- RESULT --------//

// Option == maybe there's something, maybe there's nothing
// Result == maybe it will work, or maybe it will fail

// pub enum Option<T> {
//     Some(T),
//     None,
// }

// is_some() -> bool
// is_none() -> bool

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// is_ok() -> bool
// is_err() -> bool

fn give_result(input: i32) -> Result<(), ()> {
    // nothing if it works, nothing if it doesn't work
    if input % 2 == 0 {
        Ok(()) // returning nothing inside of Ok
    } else {
        Err(()) // returning nothing inside of Err
    }
}

fn check_even() {
    if give_result(5).is_ok() {
        println!("Number is ok! guys");
    } else {
        println!("It's an error guys");
    }
}

fn main() {
    check_even();
}
