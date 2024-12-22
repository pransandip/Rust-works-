/* A String actually has ownership of the string data
* it stores, and you can modify or destroy that string data.

* A &str is just a view into string data owned by something else.
* For example, you can get a &str to string data in a String.

* As another example, when you type "foo", then this compiles down to an
* immutable global variable that contains the actual string data, and the
* &str is just a view into that global variable.

* String is an owned type that can grow and change its size at runtime.
* It is stored on the heap and has a pointer, a length, and a capacity.
* You can create, modify, and delete String values as you wish.

* &str is an immutable reference to a fixed-length sequence of UTF-8 bytes
* somewhere in memory. It does not own the data it points to, but only borrows
* it for a certain lifetime. You cannot change the size or content of a &str value,
* but you can slice it to get a smaller view of it.
*/

fn main() {
    explain_string();
    print_character();
}

fn explain_string() {
    // Creating a String from a string literal
    let mut s = String::from("hello"); // s owns the data "hello" on the heap

    // Appending to a String
    s.push_str(", world!"); // s can grow and change its content

    // Creating a &str from a string literal
    let hello = "hello"; // hello is a &str that points to the data "hello" in static memory

    // Slicing a &str
    let world = &hello[1..4]; // world is a &str that points to a part of hello's data

    println!("s is {}, world = {}", s, world);
}

fn print_character() {
    let greeting: String = String::from("hello sandip");

    let char1 = greeting.chars().nth(0);

    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No character is index 0"),
    }
}
