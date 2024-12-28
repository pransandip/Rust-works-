#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

fn main() {
    let population: u32 = 500_000;
    let capital: String = String::from("New Delhi");
    let leader_name = String::from("Modi");

    let india = Country {
        population,
        capital,
        leader_name,
    };

    println!(
        "capital: {}, population: {}, leader name: {}",
        india.capital, india.population, india.leader_name
    );

    println!("{:?}", india);
}
