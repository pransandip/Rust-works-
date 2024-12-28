/* ------------------- LOOPS -----------------*/
fn looping() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("The counter is now: {}", counter);
        if counter == 5 {
            break;
        }
    }
}

fn looping_2() {
    let mut counter: u32 = 0;
    let mut counter2: u32 = 0;
    println!("Starting the first loop");
    'first_loop: loop {
        counter += 1;
        println!("The counter is now: {}", counter);
        if counter > 9 {
            println!("Starting the second loop");
            loop {
                counter2 += 1;
                println!("The Second counter is: {}", counter2);
                if counter2 == 3 {
                    break 'first_loop;
                }
            }
        }
    }
}

// while loop = while something is true, do something
fn looping_3() {
    let mut counter = 0;
    while counter < 5 {
        counter += 1;
        println!("The while counter is now: {}", counter);
    }
}

// _ = I'm never going to use this.
// _var = I'm probably going to use this later; please be quiet.
fn looping_4() {
    for _ in 0..3 {
        println!("I am a for loop");
    }
}

fn looping_5() {
    let mut counter = 0;

    let my_number = loop {
        counter += 1;
        if counter % 5 == 3 {
            break counter; // break while returning the variable counter
        }
    };

    println!("{}", my_number)
}

fn match_colors(rbg: (i32, i32, i32)) {
    println!(
        "Comparing with color with {} red {} blue {} green",
        rbg.0, rbg.1, rbg.2
    );
    let new_vec: Vec<(i32, &str)> = vec![(rbg.0, "red"), (rbg.1, "blue"), (rbg.2, "green")];

    let mut all_have_at_least_10 = true;

    for item in new_vec {
        if item.0 < 10 {
            all_have_at_least_10 = false;
            println!("not much {}", item.1)
        }
    }
    if all_have_at_least_10 {
        println!("Every color has at least 10")
    }
    println!();
}

fn looping_6() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colors(first);
    match_colors(second);
    match_colors(third);
}

fn main() {
    looping();
    looping_2();
    looping_3();
    looping_4();
    looping_5();
    looping_6();
}
