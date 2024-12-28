#[allow(unused_variables)]

fn a() {
    let x: i32 = 5;
    let y = x;

    assert_eq!(x, 5);
    println!("success!")
}

fn define_x() {
    let x: &str = "hello";
    println!("{}, world!", x)
}

// Destructuring
fn destruct() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
    println!("Destructuring successful!")
}

// Destructuring Assignment
fn destructuring_assignment() {
    let (x, y);

    (x, ..) = (3, 4);
    [.., y] = [1, 2];

    assert_eq!([x, y], [3, 2]);
    println!("success!")
}

// Type conversion
fn conversion() {
    let v: u16 = 38_u8 as u16;
    println!("{}", v);
}
fn main() {
    a();
    define_x();
    destruct();
    destructuring_assignment();
    conversion();
}
