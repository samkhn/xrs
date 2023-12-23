fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Too sugary."),
        Some(inner) => println!("{}?. nice!", inner),
        None => println!("No drink? That's okay"),
    }
}

fn drink(drink: Option<&str>) {
    let inside = drink.unwrap();
    if inside == "lemonade" {
        panic!("Ah! not lemonade!");
    }
    println!("I love {}!", inside)
}

fn main() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;
    give_adult(water);
    give_adult(lemonade);
    give_adult(void);
    let coffee = Some("coffee");
    drink(coffee);
    drink(void); // this line throws global panic bc you unwrap a None
}
