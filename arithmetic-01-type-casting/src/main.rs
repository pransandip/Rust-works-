/*--------- TypeConversion ----------*/
fn conversion1() {
    let m = 127_000 as i64; // let binding
    let n = 20_i32;
    let p = m / (n as i64);
    println!("p: {}", p);
}

fn conversion2() {
    let x: u8 = 9; // 0 to 255
    let y: i8 = 10; // -128 to 127
    let z = x + (y as u8);
    println!("z: {}", z);

    {
        let x = 9.5f32;
        let y = 10.5f32;
        let z = x + y;
        println!("z as float: {}", z);
    }
}

fn conversion3() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));
    println!("success!");
}

// Get the type of a given variable, return a string representation of the type
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn main() {
    conversion1();
    conversion2();
    conversion3();
}
