// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

// // === SOLUTION
// use std::io;

// enum PowerState {
//     Hibernate,
//     Off,
//     Reboot,
//     Shutdown,
//     Sleep,
// }

// impl PowerState {
//     fn new(state: &str) -> Option<PowerState> {
//         // Confirm that user input is lowercase to help with match
//         // NOTE We're using shadowing to convert 'state' from &str to String
//         let state = state.trim().to_lowercase();

//         // NOTE Convert String (returned from to_lowercase()) to &str using .as_str() method
//         // Otherwise, get type mismatch error - expected String, found &str
//         match state.as_str() {
//             "hibernate" => Some(PowerState::Hibernate),
//             "off" => Some(PowerState::Off),
//             "reboot" => Some(PowerState::Reboot),
//             "shutdown" => Some(PowerState::Shutdown),
//             "sleep" => Some(PowerState::Sleep),
//             _ => None,
//         }
//     }
// }

// fn print_power_action(state: PowerState) {
//     // NOTE Neat trick to use everything in your enum so you don't have
//     // to type out each, since this is only printing the state.
//     // This means that we don't have to type out PowerState::STATE for each.
//     use PowerState::*;

//     match state {
//         Hibernate => println!("Hibernating"),
//         Off => println!("Off"),
//         Reboot => println!("Rebooting"),
//         Shutdown => println!("Shutting down"),
//         Sleep => println!("Sleeping"),
//     }
// }

// fn main() {
//     let mut buffer = String::new();

//     let user_input_status = io::stdin().read_line(&mut buffer);
//     if user_input_status.is_ok() {
//         match PowerState::new(&buffer) {
//             Some(state) => print_power_action(state),
//             None => println!("Invalid input"),
//         }
//     } else {
//         println!("Error reading input");
//     }
// }

// === MY ATTEMPT w/ notes
use std::io;

// Notes:
// * Use an enum to store the possible power states
enum PowerState {
    Hibernate,
    Off,
    Reboot,
    Shutdown,
    Sleep,
}

// * Use a function with a match expression to print out the power messages
// * The function should accept the enum as an input
fn print_power_state(state: PowerState) {
    match state {
        PowerState::Hibernate => println!("Current state: Hibernating..."),
        PowerState::Off => println!("Current state: Off..."),
        PowerState::Reboot => println!("Current state: Rebooting..."),
        PowerState::Shutdown => println!("Current state: Shutting down..."),
        PowerState::Sleep => println!("Current state: Sleeping..."),
    }
}

// NOTE Could also return Option<PowerState>. See alternate solution.
fn convert_input_to_state(input: &str) -> Result<PowerState, String> {
    match input {
        "hibernate" => Ok(PowerState::Hibernate),
        "off" => Ok(PowerState::Off),
        "reboot" => Ok(PowerState::Reboot),
        "shutdown" => Ok(PowerState::Shutdown),
        "sleep" => Ok(PowerState::Sleep),
        _ => Err("Invalid input".to_owned()),
    }
}

fn main() {
    let mut user_input = String::new();
    println!("Enter power state:");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Invalid user input");

    user_input = user_input.trim().to_lowercase();
    println!("user_input = {:?}", user_input);

    // * Use a match expression to convert the user input into the power state enum
    // * The program should be case-insensitive (the user should be able to type
    //   Reboot, reboot, REBOOT, etc.)
    // === WITHOUT helper fn - Error
    // Q: How to properly convert input string to state with match?
    // A: I created helper. String.as_str() is helpful to note as well.
    // let input_to_state = match user_input {
    //     "hibernate" => PowerState::Hibernate,
    //     "off" => PowerState::Off,
    //     "reboot" => PowerState::Reboot,
    //     "shutdown" => PowerState::Shutdown,
    //     "sleep" => PowerState::Sleep,
    // };

    // === WITH helper fn
    match convert_input_to_state(&user_input) {
        Ok(state) => print_power_state(state),
        Err(e) => println!("error = {:?}", e),
    };
}

// // === MY ATTEMPT w/o notes
// use std::io;

// enum PowerOption {
//     Hibernate,
//     Off,
//     Reboot,
//     Shutdown,
//     Sleep,
// }

// fn get_command_input() -> io::Result<String> {
//     let mut command = String::new();
//     io::stdin().read_line(&mut command)?;
//     Ok(command.trim().to_lowercase().to_owned())
// }

// fn main() {
//     let user_input = get_command_input();
//     match user_input {
//         Ok(input) => match &input[..] {
//             "hibernate" => println!("Hibernating"),
//             "off" => println!("Off"),
//             "reboot" => println!("Rebooting"),
//             "shutdown" => println!("Shutting down"),
//             "sleep" => println!("Sleeping"),
//             _ => println!("Invalid input. Try again"),
//         },
//         Err(e) => println!("Error = {:?}", e),
//     }
// }

// // === DEMO
// use std::io;

// // io has its own special Result type that has built-in Error
// fn get_input() -> io::Result<String> {
//     let mut buffer = String::new();
//     io::stdin().read_line(&mut buffer)?;
//     // NOTE Use .trim() to remove \n from Enter key but it returns &str so need .to_owned()
//     Ok(buffer.trim().to_owned())
// }

// fn main() {
//     // We want two inputs from the user so storing in vector
//     let mut all_input = vec![];
//     // Keep track of how many times user has input
//     let mut times_input = 0;

//     while times_input < 2 {
//         match get_input() {
//             Ok(words) => {
//                 all_input.push(words);
//                 times_input += 1;
//             }
//             Err(e) => println!("error = {:?}", e),
//         }
//     }

//     for input in all_input {
//         println!(
//             "original = {:?}, capitalized = {:?}",
//             input,
//             input.to_uppercase()
//         );
//     }
// }
