// // === EXAMPLE: find data
// use std::collections::HashMap;

// fn main() {
//     let mut people = HashMap::new();
//     people.insert("Max", 21);
//     people.insert("Spike", 13);
//     people.insert("Bo", 14);
//     people.insert("Gene", 42);
//     people.insert("Pam", 30);
//     people.insert("Shelly", 27);
//     people.remove("Max");

//     match people.get("Bo") {
//         Some(age) => println!("age = {:?}", age),
//         None => println!("Not found"),
//     }

//     for (person, age) in people {
//         println!("person = {:?}, age = {:?}", person, age);
//     }
// }

// // === EXAMPLE: iterate
// use std::collections::HashMap;

// fn main() {
//     let mut people = HashMap::new();
//     people.insert("Max", 21);
//     people.insert("Spike", 13);
//     people.insert("Bo", 14);
//     people.insert("Gene", 42);
//     people.insert("Pam", 30);
//     people.insert("Shelly", 27);
//     people.remove("Max");

//     match people.get("Bo") {
//         Some(age) => println!("age = {:?}", age),
//         None => println!("Not found"),
//     }

//     // Q: Any difference between people and people.iter()?
//     // A: Yes! If you don't use iter(), then people is MOVED
//     // and then DROPPED from scope! Not an issue if only used
//     // once, but if you need people later then it's gone.
//     for (person, age) in people.iter() {
//         println!("person = {:?}, age = {:?}", person, age);
//     }
//     for person in people.keys() {
//         println!("person = {:?}", person);
//     }
//     for age in people.values() {
//         println!("age = {:?}", age);
//     }
// }

// // === DEMO
// use std::collections::HashMap;

// #[derive(Debug)]
// struct Contents {
//     content: String,
// }

// fn main() {
//     let mut lockers = HashMap::new();
//     lockers.insert(
//         1,
//         Contents {
//             content: "stuff".to_owned(),
//         },
//     );
//     lockers.insert(
//         2,
//         Contents {
//             content: "shirt".to_owned(),
//         },
//     );
//     lockers.insert(
//         3,
//         Contents {
//             content: "gym shorts".to_owned(),
//         },
//     );

//     for (locker_number, content) in lockers.iter() {
//         println!("number: {:?}, content: {:?}", locker_number, content);
//     }
// }

// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
use std::collections::HashMap;

// === MY ATTEMPT
fn main() {
    let mut inventory = HashMap::new();
    inventory.insert("Chair", 5);
    inventory.insert("Bed", 3);
    inventory.insert("Table", 2);
    inventory.insert("Couch", 0);

    for (item, quantity) in inventory.iter() {
        // NOTE inventory quantity is a borrowed reference so need to dereference to get raw int:int comparison
        if *quantity == 0 {
            println!("{:?} is out of stock!", item);
        } else {
            println!("item = {:?}, stock = {:?}", item, quantity);
        }
    }

    let total_items = inventory.keys().len();
    println!("total items = {:?}", total_items);
    let total_quantity: i32 = inventory.values().sum();
    println!("total quantity = {:?}", total_quantity);
}

// === SOLUTION
// fn main() {
//     let mut inventory = HashMap::new();
//     inventory.insert("Chair", 5);
//     inventory.insert("Bed", 3);
//     inventory.insert("Table", 2);
//     inventory.insert("Couch", 0);

//     let mut total_stock = 0;
//     let total_quantity: i32 = inventory.values().sum();

//     for (item, quantity) in inventory.iter() {
//         total_stock = total_stock + quantity;
//         // NOTE stock is a borrowed reference so need to dereference to get raw int:int comparison
//         // You can also borrow the 0 as well (&0).
//         let stock_count = if quantity == &0 {
//             "Out of stock!".to_owned()
//         } else {
//             // Use format! to create a String to save instead of just printing
//             format!("{:?}", quantity)
//         };
//         // Now use the String from stock_count to print
//         println!("item = {:?}, stock = {:?}", item, stock_count);
//     }
//     println!("total stock = {:?}", total_stock);
//     println!("total quantity = {:?}", total_quantity);
// }
