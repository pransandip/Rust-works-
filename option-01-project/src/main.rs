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

fn main() {
    let new_vec: Vec<i32> = vec![1, 2];
    let bigger_vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    println!(
        "{:?} {:?}",
        take_fifth(new_vec).unwrap(), // This one is None .unwrap() will panic!
        take_fifth(bigger_vec).unwrap()
    );
}
