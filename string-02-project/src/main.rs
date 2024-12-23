// ------------------------------- STRING &Str ---------------------------- //

/*
 * String literals(or String constant) is a sequence of characters enclosed in double quotation marks.
 * that represents a null-terminated string. The characters within the quotation marks are interpreted
 * exactly as they appear, without any special meaning or processing.
 *
 *
 * The two most used string types in Rust are String and &str.
 *
 * A String is stored as a vector of bytes (Vec<u8>), but guaranteed
 * to always be a valid UTF-8 sequence. String is heap allocated,
 * growable and not null terminated.
 *
 * &str is a slice (&[u8]) that always points to a valid UTF-8 sequence,
 * and can be used to view into a String, just like &[T] is a view into Vec<T>.
 *
 */

/*
 * In Rust, String::new() creates a new, empty String.
 * while String::from() creates a String from an existing string slice (&str)
 * essentially converting a string literal or another string reference into a fully owned String object;
 *
 * use String::new() when you need an initially empty string to add data to later,
 * and String::from() when you want to convert an existing string into a String object.
 *
 */

// Key points:
/*
 * String::new():
 *
 * Creates an empty String.
 * Useful when you want to build a string by adding characters or substrings iteratively.
 * Does not allocate any initial buffer, so can be more efficient for very large strings
 * that may not be fully populated.
 *
 * String::from():
 *
 * Creates a String from an existing string slice (&str).
 * Takes a string literal or another string reference and converts it to a fully owned String
 * Useful when you need to manipulate a string that is already provided.
 *
 */

// size_of == Returns the size of a type in bytes.
// size_of_val == Returns the size of the pointed-to value in bytes.

fn print_ex() {
    println!(
        "A String is always {:?} bytes. It is Sized.",
        std::mem::size_of::<String>()
    ); // std::mem::size_of::<Type>() gives you the size in bytes of a type
    println!(
        "And an i8 is always {:?} bytes. It is Sized.",
        std::mem::size_of::<i8>()
    );
    println!(
        "And an f64 is always {:?} bytes. It is Sized.",
        std::mem::size_of::<f64>()
    );
    println!(
        "But a &str? It can be anything. '서태지' is {:?} bytes. It is not Sized.",
        std::mem::size_of_val("서태지")
    ); // std::mem::size_of_val() gives you the size in bytes of a variable
    println!(
        "And 'Adrian Fahrenheit Țepeș' is {:?} bytes. It is not Sized.",
        std::mem::size_of_val("Adrian Fahrenheit Țepeș")
    );
}

fn print_char_string() {
    let mut empty_string = String::new(); // An empty string
    let my_string = String::from("NewYork"); // A string from a literal

    // Adding to an empty string
    empty_string.push_str("Sandy");

    println!("{}", my_string);

    // Accessing characters from 'my_string'
    println!("{}", my_string.chars().nth(3).unwrap());
}

fn print_formatted_string() {
    let my_name = "Sandy";
    let my_country = "USA";
    let my_home = "Korea";

    let together = format!(
        "I am {} and I come from {} but I live in {}.",
        my_name, my_country, my_home
    );
    println!("{}", together);
}

fn main() {
    print_ex();
    print_char_string();
    print_formatted_string();

    let my_string_1: String = "Try to make into a String with into()".into();
    let my_string_2: String = "Try to make into a String with to_string()".to_string();
    println!("my_string_1: {}", my_string_1);
    println!("my_string_2: {}", my_string_2);
}
