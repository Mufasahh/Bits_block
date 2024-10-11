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
    println!("This is my first blockchain project using Rust");
    println!("I just got accepted to a top rated blockchain academy!");
}