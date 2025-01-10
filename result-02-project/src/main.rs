//TURBOFISH AND FINDING AN ERROR TYPE
fn return_number(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse::<i32>()
}

fn main() {
    let my_number = return_number("10");
    if my_number.is_ok() {
        println!("The number is: {}", my_number.unwrap());
    } else {
        println!("Error: {}", my_number.unwrap_err());
    }

    let my_vec: Vec<&str> = vec!["5", "Five", "8.7", "6tyFive", "789"];

    for number in my_vec {
        match return_number(number) {
            Ok(n) => println!("The number is: {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }
}
