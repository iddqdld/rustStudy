// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum DrinkFlavors {
    Berry,
    Lemon,
    Orange
}

struct Drink {
    flavor: DrinkFlavors,
    fluid: i32,
}

fn drink_display(drink: Drink) {
    match drink.flavor {
        DrinkFlavors::Berry => println!("it's a berry drink"),
        DrinkFlavors::Lemon => println!("it's a lemon drink"),
        DrinkFlavors::Orange => println!("it's an orange drink"),
    }
    println!("{:?}", drink.fluid);
}

fn main() {
    let cherry_cola = Drink {
        flavor: DrinkFlavors::Berry,
        fluid: 5,
    };
    drink_display(cherry_cola);
}
