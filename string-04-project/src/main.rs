/*
* A String actually has ownership of the string data
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
    // Creating a String from a string literal
    let mut txt: String = String::from("hello"); // txt owns the data "hello" on the heap

    // Appending to a String
    txt.push_str(", world"); // txt can grow and change its content

    println!(r"txt is {}", txt);
}
