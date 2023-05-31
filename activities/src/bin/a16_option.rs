// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// fn main() {}

// === NOTES
// - Option is used for scenarios when data may not be required or is
// unavailable (unable to find something, ran out of items in a list, form field not filled in,
// etc.)
// - Option is a special Enum with two variants: Some<T>, None.
// - You can use age: Some(40) or Option::Some(40).

// // === EXAMPLE 1
// struct Customer {
//     age: Option<i32>,
//     email: String,
// }

// fn main() {
//     let mark = Customer {
//         // NOTE
//         age: Some(22),
//         email: "mark@abc.com".to_owned(),
//     };
//     let becky = Customer {
//         age: None,
//         email: String::from("becky@email.com"),
//     };
//     let ruff = Customer {
//         age: Option::Some(14),
//         email: "ruff@blah.com".to_owned(),
//     };

//     // match becky.age {
//     //     // NOTE becky.age is an Option Enum type so you must account for both variants
//     //     Some(age) => println!("customer is {:?} years old", age),
//     //     None => println!("customer age not provided"),
//     // }

//     let customers: Vec<Customer> = vec![mark, becky, ruff];

//     for c in customers {
//         match c.age {
//             Some(age) => println!("{:?} is {:?} years old", c.email, age),
//             None => println!("{:?}'s age not provided", c.email),
//         }
//     }
// }

// // === EXAMPLE 2
// struct GroceryItem {
//     name: String,
//     qty: i32,
// }

// fn find_quantity(product: &str) -> Option<i32> {
//     let groceries = vec![
//         GroceryItem {
//             name: "bananas".to_owned(),
//             qty: 4,
//         },
//         GroceryItem {
//             name: "eggs".to_owned(),
//             qty: 12,
//         },
//         GroceryItem {
//             name: "break".to_owned(),
//             qty: 1,
//         },
//     ];

//     for item in groceries {
//         if item.name == product {
//             // Early return if we look up an existing grocery item
//             return Some(item.qty);
//         }
//     }
//     Option::None
// }

// fn main() {
//     let item_qty = find_quantity("bananas");
//     println!("bananas qty: {:?}", item_qty); // Some(4)
// }

// // === DEMO
// struct Survey {
//     q1: Option<i32>,
//     q2: Option<bool>,
//     q3: Option<String>,
// }

// fn main() {
//     let response = Survey {
//         q1: None,
//         q2: Some(true),
//         q3: Some("A".to_owned()),
//     };

//     match response.q1 {
//         Some(answer) => println!("q1: {:?}", answer),
//         None => println!("q1: no response"),
//     }
//     match response.q2 {
//         Some(answer) => println!("q2: {:?}", answer),
//         None => println!("q2: no response"),
//     }
//     match response.q3 {
//         Some(answer) => println!("q3: {:?}", answer),
//         None => println!("q3: no response"),
//     }
// }

// === ACTVITY
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// struct StudentLocker {
//     name: String,
//     locker: Option<i32>,
// }

// fn main() {
//     let student_lockers: Vec<StudentLocker> = vec![
//         // NOTE .to_string(), to_owned() and String::from() all seem to return String
//         // Recall that string literals (ie string slices) are &str so need to convert
//         StudentLocker {
//             name: "peach".to_string(),
//             locker: Some(1),
//         },
//         StudentLocker {
//             name: String::from("boo"),
//             locker: Some(2),
//         },
//         StudentLocker {
//             name: "bowser".to_owned(),
//             locker: Some(3),
//         },
//         StudentLocker {
//             name: "mario".to_owned(),
//             locker: None,
//         },
//     ];

//     for student in student_lockers {
//         // println!("{:?}'s locker is {:?}", student.name, student.locker); // Some(1)
//         match student.locker {
//             Some(num) => println!(
//                 "{:?}'s locker is {:?} ({:?})",
//                 student.name, num, student.locker
//             ),
//             None => println!("{:?} has no locker assigned", student.name),
//         }
//     }
// }

// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
#[derive(Debug)]
struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let students = vec![
        Student {
            name: "Peach".to_owned(),
            locker: Some(001),
        },
        Student {
            name: String::from("Bowser"),
            locker: None,
        },
        Student {
            name: "Toad".to_owned(),
            locker: Some(002),
        },
    ];

    for student in students {
        match student.locker {
            Some(num) => println!("{:?} has locker number {:?}", student, num),
            None => println!("{:?} has no locker assigned.", student.name),
        }
    }
    // NOTE: Cannot borrow 'students' here since it was moved and dropped
    // inside the for loop.
    // println!("students.len(): {:?}", students.len())
}
