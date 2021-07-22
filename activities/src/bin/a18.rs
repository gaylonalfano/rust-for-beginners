// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

// fn main() {}

// // === EXAMPLE
// fn get_sound(name: &str) -> Result<SoundData, String> {
//     if name == "alert" {
//         Ok(SoundData::new("alert")); // Result::Ok()
//     } else {
//         Err("Unable to find sound data".to_owned()); // Result::Err()
//     }
// }

// fn main() {
//     let sound = get_sound("alert");
//     match sound {
//         Ok(_) => println!("sound data located"),
//         Err(e) => println!("error: {:?}", e),
//     }
// }

// // === DEMO
// // NOTE derive(Debug) means we can print w/o matching on each variant
// #[derive(Debug)]
// enum MenuChoice {
//     MainMenu,
//     Start,
//     Quit,
// }

// fn get_choice(input: &str) -> Result<MenuChoice, String> {
//     match input {
//         "mainmenu" => Ok(MenuChoice::MainMenu),
//         "start" => Ok(MenuChoice::Start),
//         "quit" => Ok(MenuChoice::Quit),
//         _ => Err("Incorrect input".to_owned()),
//     }
// }

// fn print_choice(choice: &MenuChoice) {
//     println!("choice = {:?}", choice);
// }

// // NOTE The ? operator helps us access inner data from Result type
// // NOTE Use the 'unit type ()' if you want to return nothing for Ok variant.
// // This is why sometimes you see a fn returning: Ok(())
// fn pick_choice(input: &str) -> Result<(), String> {
//     // NOTE The ? will auto perform a match operation. If the Result is the Ok
//     // variant, then inner data wrapped by Ok will be stored into 'choice' variable.
//     // Otherwise, the Result::Err variant will be returned instead.
//     let choice: MenuChoice = get_choice(input)?;
//     // Now we can access the inner data and perform other operations with it
//     print_choice(&choice);
//     // Return nothing if everything is okay.
//     Ok(())
// }

// fn main() {
//     // === ? operator syntax
//     let choice = pick_choice("start");
//     println!("choice value = {:?}", choice); // Err("Incorrect input")

//     // === VERBOSE syntax with match expression
//     // println!("Enter a choice: 'mainmenu', 'start', or 'quit'.");

//     // // From Guessing Game code using std::io
//     // // let mut user_input = String::new();
//     // // std::io::stdin().read_line(& user_input).expect()

//     // let user_choice: Result<MenuChoice, String> = get_choice("mainmenu");
//     // // println!("choice = {:?}", user_choice);
//     // // print_choice(&user_choice); // error because &user_choice is type Result (not MenuChoice)
//     // // NOTE Need to use match expression to access the MenuChoice inside the Result's Ok variant
//     // // IMPORTANT This is a verbose approach! There is a shorthand '?' syntax that allows us to
//     // // access the inner data (MenuChoice or String), instead of matching on a Result
//     // match user_choice {
//     //     // Now we can use our custom print_choice function
//     //     // NOTE The argument is type &MenuChoice (borrowed)
//     //     Ok(c) => print_choice(&c),
//     //     Err(e) => println!("error = {:?}", e), // E.g., error = "Incorrect input"
//     // }
// }

// === ACTVITY
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21

// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

// * Use a struct to store at least the age of a customer
struct Customer {
    name: String,
    age: i32,
}

// === MY ATTEMPT
// * Use a function to determine if a customer can make a restricted purchase
fn is_restricted_purchase(customer: &Customer) -> Result<bool, String> {
    // Q: Need match or just simple if/else?
    // A: A simple if seems better for this scenario
    // Q: Do I return bool or unit type ()?
    if customer.age >= 21 {
        // * Return a result from the function
        // NOTE Have to 'return' or else it errors
        println!("Unrestricted purchase: age = {:?}", customer.age);
        // Ok(false); // Error
        // return Result::Ok(false); // Works
        return Ok(false); // Works
    } else {
        // Err("Restricted purchase. Age too low.".to_owned()); // Error
        // return Result::Err("Restricted purchase. Age too low.".to_owned()); // Works
        return Err("Restricted purchase. Age too low".to_owned()); // Works
    }
}

fn print_customer_name_if_not_restricted(is_restricted: bool, customer: &Customer) {
    match is_restricted {
        false => println!("Unrestricted customer: {:?}", customer.name),
        _ => println!("RESTRICTED! Can't print name!"),
    }
}

// == Result<(), String> variant -- Error
// fn is_restricted_purchase(age: i32) -> Result<(), String> {
//     // Q: Need match or just simple if/else?
//     // match age {
//     //     (age < 21) => Err("Restricted purchase. Customer age is: {:?}", age),
//     //     _ => Ok(true),
//     // } // ERROR
//     // Q: Do I return bool or unit type ()?
//     if age >= 21 {
//         println!("Unrestricted purchase: age = {:?}", age);
//         return Result::Ok(());
//     } else {
// * The Err variant should detail the reason why they cannot make a purchase
//         return Result::Err("Restricted purchase. Age too low.".to_owned());
//     }
// }

fn main() {
    let joe = Customer {
        name: String::from("Joe"),
        age: 33,
    };

    // let sam = Customer {
    //     name: "Sam".to_owned(),
    //     age: 18,
    // };

    // === WITHOUT ? operator (Works)
    let is_restricted = is_restricted_purchase(&joe);
    // let is_restricted = is_restricted_purchase(&joe)?; // Error E02777 ? cannot be used
    // println!("Restricted? is_restricted = {:?}", is_restricted); // Err("Restricted purchase")
    println!("Restricted? is_restricted = {:?}", is_restricted); // Ok(false)

    // To access the inner data from Result, need match express
    match is_restricted {
        // Ok(data) => println!("Ok(data) = {:?}", data), // E.g., false
        Ok(data) => print_customer_name_if_not_restricted(data, &joe),
        Err(e) => println!("error = {:?}", e), // E.g., error = "Restricted purchase. Age too low."
    }

    // === WITH ? operator (Error)
    // let is_restricted = is_restricted_purchase(joe.age)?;
    // // println!("Restricted? is_restricted = {:?}", is_restricted); // Err("Restricted purchase")
    // println!("Restricted? is_restricted = {:?}", is_restricted); // Ok(false)
}

// === SOLUTION
// struct Customer {
//     name: String,
//     age: i32,
// }

// fn try_purchase(customer: &Customer) -> Result<(), String> {
//     if customer.age < 21 {
//         Err("Customer must be at least 21".to_owned())
//     } else {
//         Ok(())
//     }
// }

// fn main() {
//     let ash = Customer {
//         name: String::from("Ash"),
//         age: 20,
//     };
//     let purchased = try_purchase(&ash);
//     println!("{:?}", purchased);
// }
