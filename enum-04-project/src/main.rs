#[derive(Debug)]
enum Something {
    One,
    Two,
}

enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}

enum Number {
    I32(i32),
    U32(u32),
}

fn print_something() {
    println!("{:?}", Something::One);
    println!("{}", Something::Two as u32);
}

fn print_season() {
    use Season::*;
    let four_seasons: Vec<Season> = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        println!("{}", season as u32);
    }
}

fn print_stars() {
    use Star::*;
    let starvec: Vec<Star> = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant];
    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star"),
            size if size > 80 => println!("This is a good-sized star"),
            _ => println!("some other type of star"),
        }
    }
    println!(
        "What about the DeadStar? It's the number {}",
        DeadStar as u32
    );
}

fn get_number(input: i32) -> Number {
    match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input),
    }
}

fn print_number() {
    let my_vec: Vec<Number> = vec![get_number(-800), get_number(8)];
    use Number::*;

    for item in my_vec {
        match item {
            I32(num) => println!("This is a negative number: {}", num),
            U32(num) => println!("This is a positive number: {}", num),
        }
    }
}
fn main() {
    print_something();
    print_season();
    print_stars();
    print_number();
}
