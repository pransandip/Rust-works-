/*
* Primitive dataTypes: Basic/Fundamental data type used to declare a value
* Primitive data types has two category: 1.) Scalar type 2.) Compound type
* Scalar: A scalar type represents a single value. Rust has four primary
* scalar types: integers, floating-point numbers, Booleans, and characters.
* Compound: compound types can group multiple values into one type.
* Rust has two primitive compound types: tuples and arrays.
*/

// 1. Tuple is fixed length sequence of elements that is immutable
// 2. Tuple can have multiple types
// 3. Arrays have to have same element inside, arrays are fixed length

/*
* Rust has scaler and compound datatype
* By default int in rust is i32 bit
* By default float in rust is f64 bit
* u8: 0 to 2^8 - 1 = 0 to 255 range of numbers you can represent
* i8: -2^7 to 2^7 - 1 = -128 to 127 range of numbers you can represent
*/

// Type Inference
fn type_inference() {
    let my_num = 1_______0_______8__________u8;
    println!("my_num: {}", my_num)
}

fn arrays() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[4] = 6;
    println!("{}", arr[4]);

    let byte = [0; 8]; // length of arr is 8 and all elements are 0
    println!("2nd element of byte array is: {}", byte[1]);
}

// tuple is a fixed length sequence of element that is immutable
fn tuples() {
    let mut tup1: (i32, bool, char) = (1, true, 'a');
    tup1.0 = 10;
    println!("{}", tup1.0);

    let tup2: (&str, u8) = ("Sandip Roy", 56);
    let (name, weight) = tup2;
    println!("name is: {} and weight: {}kg", name, weight)
}

fn tuples_2() {
    let random_tuple: (u8, u8, &str, [i8; 3], Vec<u8>, f32) =
        (7, 8, "My name is sandy", [1, 2, 3], vec![3, 4], 10.5);

    let (_a, b, _c, _d, _e, _f) = random_tuple.clone();

    println!("{:?}", random_tuple);
    println!("{:?}", random_tuple.3);
    println!("{}", b);
}

// Destructuring
fn destructure_vec() {
    let str_vec = vec!["one", "two", "three"];
    let (a, b, _) = (str_vec[0], str_vec[1], str_vec[2]); // don't give it a name not going to use it
    println!("a: {}, b: {}", a, b);
}

fn main() {
    type_inference();
    arrays();
    tuples();
    tuples_2();
    destructure_vec();
}
