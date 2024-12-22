use std::fmt::Debug;

#[derive(Debug)]
struct Animal {
    city: String,
    name: String,
    age: u8,
}

// Debug is a trait that allows us to print the value of a type = power
fn print_item<MyGenericType: Debug>(item: &MyGenericType) {
    println!("Here is your item: {:?}", item);
}

fn main() {
    let dog = Animal {
        city: String::from("New York"),
        name: "Whiskers".to_string(),
        age: 5,
    };

  let number: u8 = 102;

    print_item(&dog);
    print_item(&number);

    println!("{}, {}, {}", dog.city, dog.name, dog.age);
}
