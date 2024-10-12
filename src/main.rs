use std::collections::HashMap;

struct Country {
    name: String,
    population: HashMap<i32, i32>,
    capital: String,
}

struct Pet {
    name: String,
    age: u32,
}

fn main() {
    let my_city = Country {
        name: "Nigeria".to_string(),
        population: HashMap::new(),
        capital: "Enugu".to_string(),
    }

    let my_pet = Pet {
        name: "Puffy".to_string(),
        age = 10,
    }
    let Pet {
        name,
        age,
    } = my_pet;

    my_city.population.insert(1900, 1_500_000);
    my_city.population.insert(1960, 10_000_000);
    my_city.population.insert(2022, 20_000_000);

    for (year, population) in my_city.population {
        println!("In {year} {0} had a population of {population}", my_city.name);
    }

    println!("My pet is called {name}, he is {age} years old);
    

    println!("This is my first blockchain project using Rust");
    println!("I just got accepted to a top rated blockchain academy!");
}
