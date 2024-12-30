use std::fmt::{Display, Formatter, Result};

// Define a structure for which `fmt::Display` will be implemented.
// This is a tuple struct named `Structure` that contains an `i32`.
#[derive(Debug)]
struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented manually for the type.
impl Display for Structure {
    // This trait requires `fmt` with this exact signature. which formats the received argument. `fmt` will
    // return a result of type `fmt::Result`. This result is a typedef of `Result`, which is a common type
    // for functions that might encounter an error. Note that `fmt::Result` must be imported.
    // This method returns a result in which the error type is `fmt::Error`.
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Write strictly the first element into the supplied output stream: `f`. Returns `fmt::Result`
        // which indicates whether the operation succeeded or failed. Note that `write!` uses syntax
        // which is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

fn main() {
    let structure = Structure(3);
    println!("This tuple struct `{}` will print...", structure);
    println!("Debug: {:?}", structure);
}
