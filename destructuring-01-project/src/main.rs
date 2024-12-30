// DESTRUCTURING = pulling apart a complex data structure into its individual parts

#[allow(unused_variables)]
fn add_str() {
    let x: &str = "hello";
    println!("{}, world!", x)
}

fn check_equal() {
    let x: i32 = 5;
    let _y = x;
    assert_eq!(x, 5);
    println!("success!")
}

// Type conversion
fn conversion() {
    let v: u16 = 38_u8 as u16;
    println!("{}", v);
}

// Destructuring
fn destructuring() {
    let rbg = (50, 20, 60);
    let (r, _, g) = rbg;
    println!("Red: {}, Green: {}", r, g);
}

fn destruct_tuple() {
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

// Destructuring struct
#[derive(Debug)]
struct Person {
    name: String,
    real_name: String,
    height: u8,
    happy: bool,
}

fn destructuring_struct() {
    let papa_doc: Person = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 72,
        happy: false,
    };

    println!("{:?}", papa_doc);

    let Person {
        name: nm,
        real_name: rnm,
        .. // I don't care about the rest of the fields
    } = papa_doc;

    println!("Name: {}, Real Name: {}", nm, rnm);
}

fn main() {
    add_str();
    check_equal();
    conversion();
    destructuring();
    destruct_tuple();
    destructuring_struct();
    destructuring_assignment();
}
