/* Shadowing: when you shadow a variable you don't destroy it. you block it. */

// redeclare
fn redeclare1() {
    let x: u8 = 4;
    println!("x is {}", x);
    let x: u8 = 5;
    println!("x is {}", x);
    let x = x + 1;
    println!("x is {}", x);
}

/* easily change type in re definition */
fn redeclare2() {
    let k: u8 = 10;
    println!("k is {}", k);

    let k = "hello";
    println!("k is {}", k);
}

/* Name Shadowing: using same variable name from different scope */
fn name_shadowing() {
    let y: u8 = 4;
    println!("y is {}", y);
    {
        let y: u8 = y + 2;
        println!("y is {}", y);
    }
    let y: u8 = y + 1;
    println!("y is {}", y);
}

// Interior and exterior scope
fn scope() {
    let z: u8 = 4;
    println!("z is {}", z);
    {
        let z: u8 = z - 2;
        println!("z is {}", z);
    }
    let z: u8 = z + 2;

    println!("z is {}", z);
}

// len() gives the size is bytes
// len() returns size of a string in bytes
// char length will be four bytes in size
fn check_bytes1() {
    println!("{}", "a".len()); // These are strings not char because it is in double quotes and string can be any sort of length in size
    println!("{}", "ƒoo".len()); // 4 bytes
    println!("{}", "ą".len());
    println!("{}", "£".len());
    println!("{}", "Ⅴ".len());
}

fn check_bytes2() {
    let slice = "Hello!";
    println!("slice is {} bytes", slice.len());
    let slice2 = "﷼";
    println!("slice2 is {} bytes", slice2.len())
}

fn check_length() {
    let slice = "Hello";
    println!("length of slice is {} char long", slice.chars().count());
}

// Shadowing
fn times_two(number: i32) -> i32 {
    number * 2
}

fn shadowing() {
    let final_number = {
        let y = 10;
        let x = 9; // x starts at 9
        let x = times_two(x); // shadow with new x: 18
        let x = x + y; // shadow with new x: 28
        x // return: x final_number is now the value of x
    };

    println!("The number is now: {}", final_number)
}

fn main() {
    println!("--------redeclare1-------");
    redeclare1();
    println!("--------redeclare2-------");
    redeclare2();
    println!("--------name shadowing-------");
    name_shadowing();
    println!("--------scope-------");
    scope();
    println!("--------check_bytes1-------");
    check_bytes1();
    println!("--------check_bytes2-------");
    check_bytes2();
    println!("--------check_length-------");
    check_length();
    println!("--------shadowing-------");
    shadowing();
}
