// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

// My Solution:
fn print_message(b: bool) {
    match b {
        true => println!("its big"),
        false => println!("its small")
    }
}

fn main() {
    let num = 1000;
    let is_big = num > 100;
    print_message(is_big);
}




// EXAMPLE
// let my_num = 3;
// let is_lt_5 = my_num < 5;
// // Or, verbose
// let is_lessthan_5 = if my_num < 5 {
//     true
// } else {
//     false
// };
// // match expressions
// let message = match my_num {
//     1 => "hello",
//     _ => "goodbye"
// };
// // Shorthand
// // NOTE The expression is: my_num < 5
// let is_lt_5 = my_num < 5;

// // Nesting
// enum Menu {
//     Burger,
//     Fries,
//     Drink,
// }

// let paid = true;
// let item = Menu::Drink;
// let drink_type = "water";
// let order_placed = match item {
//     Menu::Drink => {
//         if drink_type == "water" {
//             true
//         } else {
//             false
//         }
//     }
//     _ => true,
// };


// DEMO
// enum Access {
//     Admin,
//     Manager,
//     User,
//     Guest
// }

// fn main() {
//     // Secret file - Admin only
//     let access_level = Access::Guest;
//     let can_access_file = match access_level {
//         Access::Admin => true,
//         _ => false,
//     };
//     println!("can access: {:?}", can_access_file);
// }
