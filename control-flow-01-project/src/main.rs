// conditionals
fn control_flow() {
    let condition: bool = true;
    let number: u8 = if condition { 5 } else { 6 };
    println!("number: {}", number);
}

fn check_even() {
    let is_even: bool = false;

    if is_even {
        println!("The Number is even");
    } else if !is_even {
        println!("The Number is odd");
    }
}

fn check_male() {
    let is_male: bool = true;
    let is_above_18: bool = true;

    if is_male {
        println!("You are a male");
    } else {
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        println!("You are a legal male")
    }
}

fn is_even(x: i32) -> bool {
    return x % 2 == 0;
}

fn check_odd_even() {
    let x: i32 = 99;
    let is_even = is_even(x);

    if is_even {
        println!("{} is even", x);
    } else {
        println!("{} is odd", x);
    }
}

fn check_number() {
    let my_number: u8 = 17;
    if my_number == 7 {
        println!("Number is 7");
    } else if my_number == 6 {
        println!("Number is 6");
    } else {
        println!("It's something else");
    }
}

fn match_number() {
    let my_number: u8 = 5;
    match my_number {
        5 => println!("Number is 5"),
        6 => println!("Number is 6"),
        _ => println!("It's something else"),
    }
}

fn match_example() {
    let my_number: u8 = 55;
    let something = match my_number {
        5 => 5,
        6 => 200,
        _ => 0,
    };

    println!("Something: {}", something);
}

fn match_tuple() {
    let sky: &str = "Cloudy";
    let temperature: &str = "Warm";

    match (sky, temperature) {
        ("Sunny", "Warm") => println!("It's a sunny and warm day"),
        ("Clear", "Warm") => println!("It's a nice day"),
        ("Cloudy", "Warm") => println!("It's a dark but not bad"),
        _ => println!("Not sure what's the weather is."),
    }
}

fn check_married_children() {
    let children: u8 = 3;
    let married: bool = true;

    match (married, children) {
        (true, 0) => println!("Married but no children"),
        (true, 1) => println!("Married with 1 child"),
        (true, 3) => println!("Married with 3 children"),
        (true, _) => println!("Married with more than 3 children"),
        (false, _) => println!("Not married"),
    }
}

fn check_married_children_2() {
    let children: u8 = 3;
    let married: bool = false;

    match (married, children) {
        (married, children) if married == false => {
            println!("Not married with {} children", children)
        }
        (married, children) if married == true && children == 0 => {
            println!("Married but no children")
        }
        _ => println!("Married {} with children{}", married, children),
    }
}

fn match_color(rbg: (i32, i32, i32)) {
    match rbg {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, b, _) if b < 10 => println!("Not much blue"),
        (_, _, g) if g < 10 => println!("Not much green"),
        _ => println!("Each color has at least 10"),
    }
}

fn match_number_2(input: i32) {
    match input {
        number @ 4 => println!(
            "{} is a unlucky number in china: multiplied by 2 is {}",
            number,
            number * 2
        ),
        number @ 13 => println!("{} is a lucky number in Italy", number),
        _ => println!("Looks like a normal number"),
    }
}

fn main() {
    let second = (200, 0, 0);
    control_flow();
    check_even();
    check_male();
    check_odd_even();
    check_number();
    match_number();
    match_example();
    match_tuple();
    check_married_children();
    check_married_children_2();
    match_color((5, 5, 5));
    match_color(second);
    match_number_2(4);
    match_number_2(13);
    match_number_2(50);
}
