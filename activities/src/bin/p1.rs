// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

// === L3 Attempt
// FIXME Need to change struct Bills to HashMap<String, Bill> instead of
// HashMap<String, f64>
use std::collections::HashMap;
use std::io;

fn get_user_input() -> Option<String> {
    let mut input = String::new();
    // NOTE Handy trick of using while loop to keep reading until valid input
    while io::stdin().read_line(&mut input).is_err() {
        println!("Please enter your input again");
    }
    // NOTE Give the user a chance to go back/exit the menu. To do this,
    // need to return Option<String> so we can use Some/None
    let input = input.trim().to_lowercase();
    // Q: Need to borrow input? (&input)
    if input == "" {
        None
    } else {
        Some(input)
    }
}

// Q: How to create a global, mutable vector to store all bills?
// A: It's better to define a Bills of type Vec<Bill>
// Can also just do: struct Bills(Vec<Bill>) and access via Bills.0
#[derive(Debug)]
struct Bills {
    // NOTE Common to use inner if it's the only field
    // inner: Vec<Bill>,
    // NOTE For HashMap, you can have the key be String and the value
    // be a full Bill struct. I originally used HashMap<String, f64>
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        // NOTE Set to an empty vector so can have mulptiple Bills structs through
        // and makes it easier if we wish to change structure later as we only
        // need to update the code once inside this struct, instead of everywhere the
        // struct is used.
        // Self { inner: vec![] }
        Self {
            inner: HashMap::new(),
        }
    }

    fn add_bill_menu(&mut self) {
        let new_bill = Bill::new();
        self.add_bill(new_bill);
    }
    // Q: Do I return Result type, Self, or nothing at all?
    // I can return Self for new() but doesn't make sense for other methods.
    // Result type makes sense but adds complexity.
    // UPDATE Trying to pass the Bill as arg.
    // Q: How to make the bills vector mutable?
    // A: You gotta have a &mut self reference otherwise can't figure out how
    fn add_bill(&mut self, bill: Bill) {
        // NOTE With HashMap need to use clone(), otherwise bill.name gets moved into
        // the HashMap. HashMaps require an owned String for the Key, but if we move
        // the original bill.name (no clone) into the HashMap, then we no longer have
        // access to bill.name because the Bill will no longer have ownership of the name
        // property, and therefore won't properly compile (partial move error)
        self.inner.insert(bill.name.clone(), bill);
    }

    fn view_bills_menu(&self) {
        // === Bills as Vec
        // let bills = self.view_bills();
        // println!("bills = {:?}", bills);
        // === Bills as HashMap
        self.view_bills();
    }

    fn view_bills(&self) {
        // NOTE When iterating over HashMaps, the (k, v) are BORROWED (&String, &Bill)
        // If Bills was a Vec, then we'd have to use clone() to stored owned values
        for (bill, amount) in self.inner.iter() {
            println!("bill = {:?}, amount = {:?}", bill, amount);
        }
    }

    fn remove_bill_menu(&mut self) {
        self.remove_bill();
    }

    fn remove_bill(&mut self) {
        println!("Enter name of bill to REMOVE:");
        let bill_to_remove = match get_user_input() {
            Some(bill_to_remove) => bill_to_remove,
            None => return,
        };
        if self.inner.contains_key(&bill_to_remove) {
            self.inner.remove(&bill_to_remove);
            println!("Removed bill: {:?}", &bill_to_remove);
        } else {
            println!("Bill not found. Please double-check bill name and try again.");
            // Q: Do I need to re-run remove_bill_menu()?
            // A: Doesn't seem so because this is called within a loop inside main()
        }
        // println!("Removed bill = {:?}", self.inner[&bill_to_remove]);
    }

    fn edit_bill_menu(&mut self) {
        self.edit_bill();
    }

    fn edit_bill(&mut self) {
        println!("Enter name of bill to EDIT:");
        let bill_name_to_edit = match get_user_input() {
            Some(bill_name_to_edit) => bill_name_to_edit,
            None => return,
        };


        // Q: Do I really need to first do an if self.inner.contains_key() check 
        // if I also use inner.get_mut(), which returns an Option<&mut Bill>, 
        // which means I need to handle the None variant anyway?
        // NOTE Both together works. Let's see if I can remove the contains_key() check...
        // A: NOPE! I can remove the if self.inner.contains_key() check and just use .get_mut()
        let bill_to_edit: Option<&mut Bill> = self.inner.get_mut(&bill_name_to_edit);
        // NOTE Could just directly: match self.inner.get_mut(&bill_name_to_edit) {...}
        match bill_to_edit {
            Some(bill) => {
            // Q: What's a standard way of editing a k:v pair in HashMaps if I wanted to update
            // BOTH the name and amount?
            // NOTE If they change the name then we need to update the KEY value as well
            // Q: Do I even need to do this loop? Tutorial just updates bill.amount basically
            // A: Yes! Still need to do the whole String to f64 conversion...
            loop {
                println!("Enter bill amount:");
                let amount = match get_user_input() {
                    Some(amount) => amount,
                    None => return,
                };
                // NOTE Trick is to return Result<f64, _> instead of f64
                let parsed_amount: Result<f64, _> =  amount.parse();
                match parsed_amount {
                    // IMPORTANT MUST 'return' from inside loop, otherwise infinite!
                    Ok(inner_amount) => {
                        bill.amount = inner_amount;
                        return
                    },
                    Err(_) => println!("Please enter a valid number"),
                }
            }
            }
            None => println!("No matching bill to edit"),
        }
    }
}

#[derive(Debug)]
enum MenuOption {
    Add,
    View,
    Remove,
    Edit,
}

impl MenuOption {
    fn show_menu() {
        println!("\n** Manage Bills **");
        println!("'add' - Add bill");
        println!("'view' - View bills");
        println!("'edit' - Edit bill");
        println!("'remove' - Remove bill");
        println!("'quit' - Quit");
        println!("-------------------\n");
    }

    fn get_user_option() -> Option<MenuOption> {
        let input = match get_user_input() {
            Some(input) => input,
            // FIXME Error with just returning ()...
            None => return,  // return; is same as return ();
        };

        match input.as_str() {
            "add" => Some(MenuOption::Add),
            "view" => Some(MenuOption::View),
            "edit" => Some(MenuOption::Edit),
            "remove" => Some(MenuOption::Remove),
            _ => None,
        }
    }

    // Could also consider returning a Result<(), String> based on whether
    // the CRUD commands are successful. This would give me the added bonus
    // of being able to use the ? operator on the CRUD methods as well.
    fn process_option(option: MenuOption, bills: &mut Bills) {
        // Gather the name of the bill from user
        // let name = get_user_input();

        // Q: Where should I create the new Bills?
        // A: For now I've initialized inside main so I can pass Bills
        // and MenuOption to this function.
        match option {
            MenuOption::Add => Bills::add_bill_menu(bills),
            MenuOption::View => Bills::view_bills_menu(bills),
            MenuOption::Edit => Bills::edit_bill_menu(bills),
            MenuOption::Remove => Bills::remove_bill_menu(bills),
        }
    }
}

#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}

impl Bill {
    fn new() -> Self {
        println!("Enter bill name:");
        let name = match get_user_input() {
            Some(name) => name,
            None => return,
        };
        // Method 1: Works but panics:
        // let amount = get_user_input();
        // let amount: f64 = amount
        //     .parse()
        //     .expect("Failed to convert amount from String to f64");

        // Method 2: Handle the error from parse() instead of using expect()
        // Otherwise it panics and breaks the program
        // Q: How to handle parse Err variant without panicking?
        // The Err(_) variant can't just return a print because it expects f64
        // A: The trick is to use Result<f64, ParseFloatError> when using parse()!
        // Put it inside a loop
        // TODO Could turn this into its own function get_bill_amount()
        loop {
            println!("Enter bill amount:");
            let amount = match get_user_input() {
                Some(amount) => amount,
                None => return,
            };
            // BROKEN:
            // let amount: Result<f64, _> = match amount.parse() {
            //     Ok(inner_amount) => {
            //         let amount: f64 = inner_amount;
            //         return Self { name, amount};
            //     },
            //     Err(_) => println!("Please enter a valid number")
            // };
            // WORKS:
            // NOTE Trick is to return Result<f64, _> instead of f64
            let parsed_amount: Result<f64, _> =  amount.parse();
            match parsed_amount {
                // NOTE MUST 'return' from inside loop, otherwise infinite!
                Ok(inner_amount) => {
                    let amount: f64 = inner_amount;
                    return Self { name, amount };
                },
                Err(_) => println!("Please enter a valid number"),
            }
        }
    }
}

fn main() {
    let mut bills = Bills::new();

    loop {
        MenuOption::show_menu();
        let user_option: Option<MenuOption> = MenuOption::get_user_option();

        match user_option {
            Some(option) => MenuOption::process_option(option, &mut bills),
            None => {
                println!("Invalid option");
                break;
            }
        }
    }
}

// // === L2 Attempt
// use std::collections::HashMap;
// use std::io;

// fn get_user_input() -> String {
//     let mut input = String::new();
//     // NOTE Handy trick of using while loop to keep reading until valid input
//     while io::stdin().read_line(&mut input).is_err() {
//         println!("Please enter your input again");
//     }
//     input.trim().to_lowercase()
// }

// // Q: How to create a global, mutable vector to store all bills?
// // A: It's better to define a Bills of type Vec<Bill>
// // Can also just do: struct Bills(Vec<Bill>) and access via Bills.0
// #[derive(Debug)]
// struct Bills {
//     // NOTE Common to use inner if it's the only field
//     // inner: Vec<Bill>,
//     inner: HashMap<String, f64>,
// }

// impl Bills {
//     fn new() -> Self {
//         // NOTE Set to an empty vector so can have mulptiple Bills structs through
//         // and makes it easier if we wish to change structure later as we only
//         // need to update the code once inside this struct, instead of everywhere the
//         // struct is used.
//         // Self { inner: vec![] }
//         Self {
//             inner: HashMap::new(),
//         }
//     }

//     fn add_bill_menu(&mut self) {
//         let new_bill = Bill::new();
//         self.add_bill(new_bill);
//     }
//     // Q: Do I return Result type, Self, or nothing at all?
//     // I can return Self for new() but doesn't make sense for other methods.
//     // Result type makes sense but adds complexity.
//     // UPDATE Trying to pass the Bill as arg.
//     // Q: How to make the bills vector mutable?
//     // A: You gotta have a &mut self reference otherwise can't figure out how
//     fn add_bill(&mut self, bill: Bill) {
//         // self.inner.push(bill); // Or, accessed via self.0.push(new_bill);
//         self.inner.insert(bill.name, bill.amount);
//     }

//     fn view_bills_menu(&self) {
//         // === Bills as Vec
//         // let bills = self.view_bills();
//         // println!("bills = {:?}", bills);
//         // === Bills as HashMap
//         self.view_bills();
//     }

//     fn view_bills(&self) {
//         for (bill, amount) in self.inner.iter() {
//             println!("bill = {:?}, amount = {:?}", bill, amount);
//         }
//     }

//     fn remove_bill_menu(&mut self) {
//         self.remove_bill();
//     }

//     fn remove_bill(&mut self) {
//         // let removed_bill = self.inner.pop();
//         let bill_to_remove = get_user_input();
//         // TODO Determine whether bill exists
//         self.inner.remove(&bill_to_remove);
//         // println!("Removed bill = {:?}", self.inner[&bill_to_remove]);
//     }
// }

// #[derive(Debug)]
// enum MenuOption {
//     Add,
//     View,
//     Remove,
//     // Edit,
// }

// impl MenuOption {
//     fn show_menu() {
//         println!("\n** Manage Bills **");
//         println!("'add' - Add bill");
//         println!("'view' - View bills");
//         println!("'remove' - Remove bill");
//         println!("'quit' - Quit");
//         println!("-------------------\n");
//     }

//     fn get_user_option() -> Option<MenuOption> {
//         let input = get_user_input();

//         match input.as_str() {
//             "add" => Some(MenuOption::Add),
//             "view" => Some(MenuOption::View),
//             "remove" => Some(MenuOption::Remove),
//             _ => None,
//         }
//     }

//     // TODO Could consider adding a process_option() function
//     // Could also consider returning a Result<(), String> based on whether
//     // the CRUD commands are successful. This would give me the added bonus
//     // of being able to use the ? operator on the CRUD methods as well.
//     fn process_option(option: MenuOption, bills: &mut Bills) {
//         // Gather the name of the bill from user
//         // let name = get_user_input();

//         // Q: Where should I create the new Bills?
//         // A: For now I've initialized inside main so I can pass Bills
//         // and MenuOption to this function.
//         match option {
//             MenuOption::Add => Bills::add_bill_menu(bills),
//             MenuOption::View => Bills::view_bills_menu(bills),
//             MenuOption::Remove => Bills::remove_bill_menu(bills),
//         }
//     }
// }

// #[derive(Debug)]
// struct Bill {
//     name: String,
//     amount: f64,
// }

// impl Bill {
//     fn new() -> Self {
//         println!("Enter bill name:");
//         let name = get_user_input();
//         println!("Enter bill amount:");
//         let amount = get_user_input();
//         let amount: f64 = amount
//             .parse()
//             .expect("Failed to convert amount from String to f64");
//         // FIXME Could handle the error from parse() instead of using expect()
//         // Otherwise it panics and breaks the program
//         // Q: How to handle parse Err variant without panicking?
//         // The Err(_) variant can't just return a print because it expects f64
//         // Put it inside a loop? BROKEN ATTEMPT:
//         // loop {
//         //     let amount = get_user_input();
//         //     println!("Enter bill amount:");
//         //     let amount: f64 = match amount.parse() {
//         //         Ok(inner_amount) => inner_amount,
//         //         Err(_) => {
//         //             println!("Please enter a valid number.");
//         //             continue;
//         //         }
//         //     };
//         // }
//         Self { name, amount }
//     }
// }

// fn main() {
//     let mut bills = Bills::new();

//     loop {
//         MenuOption::show_menu();
//         let user_option: Option<MenuOption> = MenuOption::get_user_option();

//         match user_option {
//             Some(option) => MenuOption::process_option(option, &mut bills),
//             None => {
//                 println!("Invalid option");
//                 break;
//             }
//         }
//     }
// }

// // === L1 Attempt
// use std::io;

// fn get_user_input() -> String {
//     let mut input = String::new();
//     // NOTE Handy trick of using while loop to keep reading until valid input
//     while io::stdin().read_line(&mut input).is_err() {
//         println!("Please enter your input again");
//     }
//     input.trim().to_lowercase()
// }

// // Q: How to create a global, mutable vector to store all bills?
// // A: It's better to define a Bills of type Vec<Bill>
// // Can also just do: struct Bills(Vec<Bill>) and access via Bills.0
// #[derive(Debug)]
// struct Bills {
//     // NOTE Common to use inner if it's the only field
//     inner: Vec<Bill>,
// }

// impl Bills {
//     fn new() -> Self {
//         // NOTE Set to an empty vector so can have mulptiple Bills structs throughout
//         // and makes it easier if we wish to change structure later as we only
//         // need to update the code once inside this struct, instead of everywhere the
//         // struct is used.
//         Self { inner: vec![] }
//     }

//     fn add_bill_menu(&mut self) {
//         let new_bill = Bill::new();
//         self.add_bill(new_bill);
//     }
//     // Q: Do I return Result type, Self, or nothing at all?
//     // I can return Self for new() but doesn't make sense for other methods.
//     // Result type makes sense but adds complexity.
//     // UPDATE Trying to pass the Bill as arg.
//     // Q: How to make the bills vector mutable?
//     // A: You gotta have a &mut self reference otherwise can't figure out how
//     fn add_bill(&mut self, bill: Bill) {
//         self.inner.push(bill); // Or, accessed via self.0.push(new_bill);
//     }

//     fn view_bills_menu(&self) {
//         let bills = self.view_bills();
//         println!("bills = {:?}", bills);
//     }

//     fn view_bills(&self) -> &Vec<Bill> {
//         // NOTE Returning borrowed Vec from Bills since already
//         // created the Vec.
//         &self.inner
//     }

//     // fn edit_bill(&mut self, name: &str) -> Result<(), String> {
//     //     println!("EDITING bill!");
//     //     // Retrieve the bill (better would be a HashMap)
//     //     let mut bill: Bill = self.bills[0];
//     //     // Make edits to the bill
//     //     let new_amount = get_user_input();
//     //     let new_amount: f64 = new_amount.parse().expect("Please enter a number!");
//     //     bill.amount = new_amount;

//     //     Ok(())
//     // }
//     // fn remove_bill(&mut self, name: &str) -> Result<(), String> {
//     //     println!("REMOVING bill!");
//     //     self.bills.pop();
//     //     Ok(())
//     // }
// }

// #[derive(Debug)]
// enum MenuOption {
//     Add,
//     View,
//     // Edit,
//     // Remove,
// }

// impl MenuOption {
//     fn show_menu() {
//         println!("\n** Manage Bills **");
//         println!("'add' - Add bill");
//         println!("'view' - View bills");
//         println!("'quit' - Quit");
//         println!("-------------------\n");
//     }

//     fn get_user_option() -> Option<MenuOption> {
//         let input = get_user_input();

//         match input.as_str() {
//             "add" => Some(MenuOption::Add),
//             "view" => Some(MenuOption::View),
//             // "edit" => Some(MenuOption::Edit),
//             // "remove" => Some(MenuOption::Remove),
//             _ => None,
//         }
//     }

//     // TODO Could consider adding a process_option() function
//     // Could also consider returning a Result<(), String> based on whether
//     // the CRUD commands are successful. This would give me the added bonus
//     // of being able to use the ? operator on the CRUD methods as well.
//     fn process_option(option: MenuOption, bills: &mut Bills) {
//         // Gather the name of the bill from user
//         // let name = get_user_input();

//         // Q: Where should I create the new Bills?
//         // A: For now I've initialized inside main so I can pass Bills
//         // and MenuOption to this function.
//         match option {
//             MenuOption::Add => Bills::add_bill_menu(bills),
//             MenuOption::View => Bills::view_bills_menu(bills),
//             // MenuOption::Edit => Bills::edit(&name),
//             // MenuOption::Remove => Bills::remove(&name),
//         }
//     }
// }

// #[derive(Debug)]
// struct Bill {
//     name: String,
//     amount: f64,
// }

// impl Bill {
//     fn new() -> Self {
//         println!("Enter bill name:");
//         let name = get_user_input();
//         println!("Enter bill amount:");
//         let amount = get_user_input();
//         // TODO Could handle the error from parse() instead of using expect()
//         // Otherwise it panics and breaks the program
//         let amount: f64 = amount
//             .parse()
//             .expect("Failed to convert amount from String to f64");
//         Self { name, amount }
//     }
// }

// fn main() {
//     let mut bills = Bills::new();

//     loop {
//         MenuOption::show_menu();
//         let user_option: Option<MenuOption> = MenuOption::get_user_option();

//         match user_option {
//             Some(option) => MenuOption::process_option(option, &mut bills),
//             None => {
//                 println!("Invalid option");
//                 break;
//             }
//         }
//     }
// }
