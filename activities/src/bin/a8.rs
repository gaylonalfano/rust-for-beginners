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


// * Use an enum to create different flavors of drinks
#[derive(Debug)]
enum Flavor {
    Banana,
    Grape,
    Strawberry,
}
// * Use a struct to store drink flavor and fluid ounce information
struct DrinkInfo {
    flavor: Flavor,
    fluid_ounces: f64,
}
// * Use a function to print out the drink flavor and ounces
fn print_drink_info(drink: DrinkInfo) {
    // * Use a match expression to print the drink flavor
    match drink.flavor {
        Flavor::Banana => println!("flavor: {:?}", drink.flavor),
        Flavor::Grape => println!("flavor: {:?}", drink.flavor),
        Flavor::Strawberry => println!("flavor: {:?}", drink.flavor),
    }
    // Print out the ounces (oz)
    println!("fluid ounces: {:?}", drink.fluid_ounces);
}
fn main() {
    let fanta = DrinkInfo {
        flavor:  Flavor::Strawberry,
        fluid_ounces:  6.55,
    };

    let dr_pepper = DrinkInfo {
        flavor: Flavor::Grape,
        fluid_ounces: 3.75,
    };

    
    print_drink_info(fanta);
    print_drink_info(dr_pepper);
}



// Demo
// struct GroceryItem {
//     stock: i32,
//     price: f64,
//     // NOTE Strings seem to be handled differently. Can't just assign to 'str'
//     // name: str,
// }

// fn main() {
//     let cereal = GroceryItem {
//         stock: 10,
//         price: 2.99,
//         // name = "cereal",
//     };

//     // println!("Item name: {:?}", cereal.name);
//     println!("stock: {:?}", cereal.stock);
//     println!("price: {:?}", cereal.price);
// }

