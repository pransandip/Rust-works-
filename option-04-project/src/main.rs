fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn main() {
    let my_vec: Vec<i32> = vec![1, 2];
    let bigger_vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let vec_of_vecs: Vec<Vec<i32>> = vec![my_vec, bigger_vec];

    for vec in vec_of_vecs {
        let inside_number: Option<i32> = take_fifth(vec);
        if inside_number.is_some() {
            println!("We got: {}", inside_number.unwrap());
        } else {
            println!("We got nothing!");
        }

        println!("The number is: {}", inside_number.unwrap_or(0));
    }
}
