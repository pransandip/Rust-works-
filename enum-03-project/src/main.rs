// use = import
enum Mood {
    Happy,
    NotBad,
    Sleepy,
    Angry,
}

fn match_mood(mood: &Mood) -> i32 {
    use Mood::*; // * = everything inside Mood
    match mood {
        Happy => 10, // we are making this local
        NotBad => 6,
        Sleepy => 7,
        Angry => 2,
    }
}

fn main() {
    let my_mood = Mood::NotBad;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 to 10 my Happiness level is: {}", happiness_level);
}
