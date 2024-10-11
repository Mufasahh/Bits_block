use std::collections::HashMap;

struct Country {
    name: String,
    population: HashMap<i32, i32>,
    capital: String,
}

fn main() {
    let my_city = Country {
        name: "Nigeria".to_string(),
        population: HashMap::new(),
        capital: "Enugu".to_string(),
    }

    my_city.population.insert(1900, 1_500_000);
    my_city.population.insert(1960, 10_000_000);
    my_city.population.insert(2022, 20_000_000);

    for (year, population) in my_city.population {
        println!("In {year} {0} had a population of {population}", my_city.name);
    }
    

    println!("This is my first blockchain project using Rust");
    println!("I just got accepted to a top rated blockchain academy!");
}