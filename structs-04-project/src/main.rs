// Self = Book
#[derive(Debug)]
struct Book {
    number: u32,
}

impl Book {
    fn get_number(&self) -> u32 {
        self.number
    }

    fn change_number(&mut self, new_number: u32) {
        self.number = new_number;
    }

    // Associated function / static function
    fn new(number: u32) -> Self {
        Self { number }
    }
}

fn main() {
    // let mut my_book: Book = Book { number: 50 };
    let mut my_book: Book = Book::new(50);
    my_book.change_number(100);

    println!("The number is: {:?}", my_book);
    println!("The number is: {}", my_book.get_number());
}
