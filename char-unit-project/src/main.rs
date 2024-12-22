/* size_of_val return size in bytes */
use std::mem::size_of_val;

fn check_size() {
    let c1: char = 'a'; // 4 bytes
    assert_eq!(size_of_val(&c1), 4);

    let c2: char = '$';
    println!("{}", size_of_val(&c2));

    println!("success!")
}

// 1. string will be in double quotes
// 2. char will be in single quotes

fn print_char(c: char) {
    println!("{}", c)
}

fn string_or_char() {
    let c1: char = 'a';
    print_char(c1);
}

fn check_boolean() {
    let _f: bool = false;
    let t: bool = true;

    if !t {
        println!("success!");
    } else {
        println!("not successful");
    }
}

fn check_true_false() {
    let t: bool = false;
    let f: bool = true && false; // false

    assert_eq!(t, f);
    println!("success!")
}

// unit type
/* Unit type doesn't hold any value means 0 bytes */

fn check_unit_type() {
    let v: () = ();

    let _y: (i32, i32) = (2, 3);
    assert_eq!(v, implicitly_ret_unit());

    println!("success!")
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn _explicitly_ret_unit() -> () {
    println!("I will return a ()");
}

// Size of unit type
fn main() {
    check_size();
    string_or_char();
    check_boolean();
    check_true_false();
    check_unit_type();

    let unit: () = ();
    assert!(size_of_val(&unit) == 0);
    println!("unit size checked!")
}
