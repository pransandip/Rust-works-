// IMPLEMENTING STRUCTS AND ENUMS
#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

impl Animal {
    fn new() -> Self {
        Self {
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }

    fn change_to_dog(&mut self) {
        println!("Changing animal to Dog");
        self.animal_type = AnimalType::Dog;
    }

    fn change_to_cat(&mut self) {
        println!("Changing animal to Cat");
        self.animal_type = AnimalType::Cat;
    }

    fn check_type(&self) {
        use AnimalType::*;
        match self.animal_type {
            Cat => println!("This is a Cat"),
            Dog => println!("This is a Dog"),
        }
    }
}
fn main() {
    let mut new_animal: Animal = Animal::new();
    println!("{:?}", new_animal);

    new_animal.change_to_dog();
    new_animal.check_type();

    new_animal.change_to_cat();
    new_animal.check_type();
}
