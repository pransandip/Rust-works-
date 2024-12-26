/* enum = enumeration = list of choices */
// struct = something AND something AND something
// enum = something OR something OR something -> making one choice

enum ThingsInTheSky {
    Sun(String),
    Starts(String),
}

fn create_skystate(time: u8) -> ThingsInTheSky {
    let string1 = String::from("I can see the sun");
    let string2 = String::from("I can see the stars");

    match time {
        6..=18 => ThingsInTheSky::Sun(string1),
        _ => ThingsInTheSky::Starts(string2),
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    // we pass it as a reference so it doesn't die
    match state {
        ThingsInTheSky::Sun(description) => println!("{}", description),
        ThingsInTheSky::Starts(description) => println!("{}", description),
    }
}

fn main() {
    let time = 19;
    let skystate: ThingsInTheSky = create_skystate(time);
    check_skystate(&skystate);
}
