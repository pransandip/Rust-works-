//------------------- LOOPS -----------------//
// while loop = while something is true

fn looping() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("The counter is now: {}", counter);
        if counter == 10 {
            break;
        }
    }
}

fn looping_2() {
    let mut counter: u32 = 0;
    let mut counter2: u32 = 0;
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

fn looping_3() {
    let mut counter = 0;
    while counter < 5 {
        counter += 1;
        println!("The counter is now: {}", counter);
    }
}

fn main() {
    looping();
    looping_2();
    looping_3();
}
