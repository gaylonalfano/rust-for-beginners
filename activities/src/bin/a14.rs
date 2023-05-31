// Topic: Strings
//
// === NOTES:
// Two common used types of strings:
//https://github.com/akinsho/toggleterm.nvim#setup String - owned
// &str - borrowed String slice ("string slice")
// Must use an owned String to store in a struct -- Currently CAN'T
// store a slice in a struct.
// Use &str when passing to a function
// Use .to_owned() or String::from() to create an OWNED copy of a string slice
// Use an owned String whenever storing in a struct
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
// struct Person {
//     // * The color and name should be stored as a String
//     age: i32,
//     name: String,
//     favorite_color: String
// }

// fn print_property(data: &str) {
//     println!("{:?}", data);
// }

// fn main() {
//     // * Create and store at least 3 people in a vector
//     let people = vec![
//         Person {
//             age: 8,
//             name: "Aaron".to_owned(),
//             favorite_color: String::from("black"), // Transforming string slice (&str) into owned String
//         },
//         Person {
//             age: 10,
//             name: "Adrian".to_owned(),
//             favorite_color: String::from("green"),
//         },
//         Person {
//             age: 7,
//             name: "Archie".to_owned(),
//             favorite_color: String::from("purple"),
//         },
//         Person {
//             age: 11,
//             name: "Gaylon".to_owned(),
//             favorite_color: String::from("orange"),
//         },
//     ];

//     // * Iterate through the vector using a for..in loop
//     for p in people {
//         // * Use an if expression to determine which person's info should be printed
//         if p.age <= 10 {
//             // * The name and colors should be printed using a function
//             // NOTE Recall that p is type Person with String properties
//             print_property(&p.name);
//             print_property(&p.favorite_color);
//         }
//     }
// }

// === DEMO
// struct LineItem {
//     name: String, // Item name
//     count: i32, // Number of item purchased
// }

// fn print_name(name: &str) {
//     println!("name: {:?}", name);
// }

// fn main() {
//     let receipt = vec![
//         LineItem { name: "carrot".to_owned(), count: 3 },
//         LineItem { name: String::from("tomato"), count: 2 },
//         LineItem { name: "cheerios".to_owned(), count: 1 },
//     ];

//     for item in receipt {
//         // NOTE Must borrow the name string from the LineItem since our
//         // custom print fn takes a string slice instead of String.
//         print_name(&item.name);
//         // NOTE Of course, you don't have to create a custom print function
//         // but this is to show how using &str string slices in functions work.
//         println!("count = {:?}", item.count);
//     }
// }

// === EXAMPLE - Pass string data to a function
// fn print_it(data: &str) {
//     println!("{:?}", data);
// }

// fn main() {
//     print_it("a string slice");
//     let owned_string = "owned string".to_owned();
//     let another_owned = String::from("another");
//     print_it(&owned_string);
//     print_it(&another_owned);
// }

// // === EXAMPLE - Error vs. Solution Scenario (&str vs. String)
// struct Employee {
//     // name: &str,  // ERROR: Expected named lifetime parameter
//     name: String, // SOLUTION
// }

// fn main() {
//     // ERROR Scenario
//     // NOTE Cannot store borrowed data inside a struct because when the struct
//     // is dropped at the end of its scope, it's responsible for cleaning up its
//     // own memory. The struct isn't allowed to clean up borrowed memory.
//     // let emp_name = "Gaylon"; // Automatically BORROWED
//     // let emp = Employee {
//     //     name: emp_name
//     // };
//     // SOLUTION Scenario
//     // Inside the struct you can change type from &str to String
//     // let emp_name = "Gaylon".to_owned();
//     let emp_name = String::from("Gaylon");
//     // NOTE Now OWNED data can be transferred/stored inside the struct, so the
//     // Employee now OWNS the 'name' data, so whenever the struct scope is complete
//     // and is eventually dropped by the program, the struct will be allowed to clear
//     // the name: String memory and the compile will succeed.
//     let emp = Employee {
//         name: emp_name,
//     };
// }

// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    age: i32,
    name: String,
    color: String,
}

// * The name and colors should be printed using a function
impl Person {
    fn print_name(&self) {
        println!("name: {:?}", self.name);
    }
    fn print_fav_color(&self) {
        println!("fav color: {:?}", self.color);
    }
}
// NOTE Or, you can define a general-use print() fn
fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    // * Create and store at least 3 people in a vector
    let peeps = vec![
        Person {
            age: 8,
            name: "Archie".to_owned(),
            color: String::from("Yellow"),
        },
        Person {
            age: 9,
            name: "Aaron".to_owned(),
            color: String::from("Black"),
        },
        Person {
            age: 11,
            name: "Adrian".to_owned(),
            color: String::from("Blue"),
        },
    ];

    // * Iterate through the vector using a for..in loop
    for p in peeps {
        // * Use an if expression to determine which person's info should be printed
        if p.age < 10 {
            p.print_name();
            p.print_fav_color();
        } else {
            println!("{:?} is too old!", p.name);
        }
    }
}
