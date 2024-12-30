use std::cmp::PartialOrd;
use std::fmt::{Debug, Display};

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn compare_and_display<T, U, V>(statement: T, num_1: U, num_2: U, animal: V)
where
    T: Display,
    U: Display + PartialOrd,
    V: Debug,
{
    println!(
        "{}! Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2
    );

    println!("By the way i have an animal: {:?}", animal);
}

fn main() {
    let dog = Animal {
        name: "Charlie".to_string(),
        age: 5,
    };

    println!("{}, {}", dog.name, dog.age);
    compare_and_display("Listen up!".to_string(), 9, 8, dog);
}
