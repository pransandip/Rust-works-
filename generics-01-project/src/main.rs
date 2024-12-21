// GENERICS (in functions)
// generics = can be one type, can be another type
// generic = concrete = one type

// T, U, V - one capital letter in rust mean generic
// monomorphic = make into one form
// polymorphic = of more then one form

// we only used functions with concrete types
fn return_thing<T>(thing: T) -> T {
    return thing;
}

// Get the type of a given variable, return
// a string representation of the type
fn type_of<T>(_: T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn main() {
    println!("Return things is: {}", return_thing("Gold"));

    println!(
        "Type of given variable in string: {}",
        type_of(return_thing(10))
    ); // "i32"
}
