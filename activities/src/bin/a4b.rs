// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

// fn random_number() -> i32 {
//     100 % 4 * 4 - 1
// }

// fn main() {
//     let number = random_number();
//     match number {
//         1 => println!("one"),
//         2 => println!("two"),
//         3 => println!("three"),
//         _ => println!("other"),
//     }
// }


// Notes:

fn random_number() -> i32 {
    // ((100 % 4) * 4) - 1
    8
}

fn main() {
    // * Use a variable set to any integer
    let number = random_number();
    println!("number is: {:?}", number);

    // * Use a match expression to determine which message to display
    // * Use an underscore (_) to match on any value
    match number {
        1 => println!("It's one!"),
        2 => println!("It's two!"),
        3 => println!("It's three!"),
        _ => println!("Number is: {:?}", number)
    }
}
