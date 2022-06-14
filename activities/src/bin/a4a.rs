// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

// fn main() {
//     // let my_name = "Gaylon";
//     // match my_name {
//     //     // NOTE match expression is checked by compiler, unlike if else
//     //     // All options must be accounted for (ie checked by compiler)
//     //     // This is good bc if you add a new possibility, you'll be notified.
//     //     // We use a comma since it's an expression, not a statement with semicolon
//     //     "Stu" => println!("Hey, Stu!"),
//     //     "Max" => println!("Super fast, Max!"),
//     //     "Gaylon" => println!("That's my name!"),
//     //     _ => println!("Nice to meet you, stranger!"),
//     // }

//     let my_bool = false;
//     match my_bool {
//         true => println!("It's true!"),
//         false => println!("It's false!"),
//     }
// }



// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    let is_raining: bool = false;
    match is_raining {
        true => println!("is_raining={:?}", is_raining),
        false => println!("is_raining={:?}", is_raining)
    }
    
}
