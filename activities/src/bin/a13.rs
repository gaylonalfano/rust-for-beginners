// Topic: Vectors
//
// NOTES:
// Vectors must store the same TYPE of data
// Can push(), pop() and iterate over via loops
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector


// fn main() {
//     // * Use a vector to store 4 numbers
//     let my_nums = vec![10, 20, 30, 40];

//     // * Iterate through the vector using a for..in loop
//     // IMPORTANT You transfer ownership of my_nums to the FOR loop! Once the
//     // loop is finished, my_nums is dropped! Therefore you must borrow the data!
//     for n in &my_nums {
//         // Using if/else:
//         // * Determine whether to print the number or print "thirty" inside the loop
//         // if n == 30 {
//         //     println!("thirty");
//         // } else {
//         //     println!("{:?}", n);
//         // }

//         // Using match:
//         match n {
//             30 => println!("thirty"),
//             _ => println!("{:?}", n),
//         }
//     }

// use std::vec;



//     // * Use the .len() function to print the number of elements in a vector
//     // IMPORTANT You transfer ownership of my_nums to the FOR loop! Once the
//     // loop is finished, my_nums is dropped! Therefore you must borrow the
//     // data (my_nums) inside the FOR loop, otherwise won't compile.
    // println!("number of elements = {:?}", my_nums.len());

// }


// DEMO
// struct Test {
//     score: i32,
// }

// fn main() {
//     let my_scores = vec![
//         Test { score: 90 },
//         Test { score: 80 },
//         Test { score: 88 },
//         Test { score: 77 },
//     ];
//
        // NOTE The vec! macro is equivalent to:
//     let mut my_scores_2 = Vec::new();
//     my_scores_2.push(90);
//     my_scores_2.push(80);
//     my_scores_2.push(88);
//     my_scores_2.push(77);

//     for test in my_scores {
//         println!("score = {:?}", test.score);
//     }
// }


// == Activity pt. 2
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//

// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let numbers = vec![10, 20, 30, 40];

    for n in &numbers {
        match n {
            30 => println!("thirty"),
            _ => println!("{:?}", n)
        }
    }

    println!("total elements: {:?}", numbers.len());
}

