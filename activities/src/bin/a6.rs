// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

fn main() {
    let mut counter = 5;
    while counter > 0 {
        println!("{:?}", counter);
        counter = counter - 1;
    }
    println!("Done!")
}

// fn main() {
//     let mut i = 5;
//     while i > 0 {
//         println!("Count down: {:?}", i);
//         i = i - 1;
//     }
//     println!("Done!");
// }
