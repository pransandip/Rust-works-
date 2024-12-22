// Option = may be there is something, maybe there's not

// pub enum Option<T> {
//     Some(T),
//     None,
// }

fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn handle_option(my_option: Vec<Option<i32>>) {
    for item in my_option {
        match item {
            Some(number) => println!("Got a {}", number),
            None => println!("This vec is too short!"),
        }
    }
}

fn main() {
    let my_vec: Vec<i32> = vec![1, 2];
    let bigger_vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let mut option_vec: Vec<Option<i32>> = Vec::new(); // [None, Some(5)]

    option_vec.push(take_fifth(my_vec));
    option_vec.push(take_fifth(bigger_vec));

    handle_option(option_vec);
}
