// IF LET

fn check_numbers() {
    let my_vec = vec![1, 2, 3, 4, 5];
    let get_one = my_vec.get(0);
    let get_two = my_vec.get(10);
    println!("{:?} {:?}", get_one, get_two);
}

fn check_numbers_2() {
    let my_vec = vec![1, 2, 3, 4, 5];
    for index in 0..10 {
        match my_vec.get(index) {
            Some(number) => println!("The number is {}", number),
            None => {}
        }
    }
}

fn check_numbers_if_let() {
    let my_vec = vec![1, 2, 3];
    for index in 0..10 {
        if let Some(number) = my_vec.get(index) {
            println!("The number is {}", number);
        }
    }
}

fn check_weather() {
    let weather_vec = vec![
        vec!["berlin", "rainy", "5", "-7", "78"],
        vec!["london", "sunny", "not humid", "12", "3", "45"],
    ];

    for mut city in weather_vec {
        println!("For the city of {}", city[0]);
        while let Some(info) = city.pop() {
            if let Ok(number) = info.parse::<i32>() {
                println!("The number is {}", number);
            }
        }
    }
}

fn main() {
    check_weather();
    check_numbers();
    check_numbers_2();
    check_numbers_if_let();
}
