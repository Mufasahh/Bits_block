fn main() {
    my_city.population.insert(1900, 1_500_000);
    my_city.population.insert(1960, 10_000_000);
    my_city.population.insert(2022, 20_000_000);

    for (year, population) in my_city.population {
        println!("In {year} {0} had a population of {population}", my_city.name);
    }
    
    println!("This is my first blockchain project using Rust");
    println!("I just got accepted to a top rated blockchain academy!");
}