// The Rules of References
// 1. At any given time, you can have either one mutable
// reference or any number of immutable reference.
// 2. References must always be valid.

fn main() {
    mut_ref();
    value_ref();
    check_length();
    string_slices();
    print_pointer();
}

// Value of reference
// & = *
// && = **
fn value_ref() {
    let my_number = 8;
    let my_reference = &my_number;
    println!("{}", my_number == *my_reference); // value of reference
}

// Mutable reference
fn mut_ref() {
    let mut s: String = String::from("Sandy ");
    change(&mut s);
    println!("s: {}", s)
}

fn change(_some_string: &mut String) {
    _some_string.push_str("Roy");
}

/*-------- dangling references ---------*/
// fn check_ref() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("sam");
//     &s
// }
/*---------------- End ----------------*/

// Check length of the word
fn check_length() {
    let mut s = String::from("hello world!");
    let word = first_word(&s);
    println!("word1: {}", word);
    s.clear(); // empty string
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// Slice
fn string_slices() {
    let s = String::from("hello world");
    let _hello = &s[..5]; // if we starting beginning of the string we can omit 0
    let _world = &s[6..11]; // if our range continuous to the end of the string can omit 11
    let _hello_world = &s[..];

    let word = first_word_2(&s);
    println!("word2: {}", word);
}

// String reference (&s) automatically co-works to string slice (&str)
fn first_word_2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("i: {}", i);
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// print pointer (memory address)
fn print_pointer() {
    let number: i8 = 9;
    let number_ref = &number;
    println!("{:p}", number_ref);
}
