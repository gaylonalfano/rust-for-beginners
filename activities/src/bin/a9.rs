// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

// * Use a function that returns a tuple
// fn return_a_coord() -> (i32, i32) {
//     (4, 5)
// }
// fn main() {
//     // * Destructure the return value into two variables
//     let (x, y) = return_a_coord();
//     // * Use an if..else if..else block to determine what to print
//     // * Print whether the y-value of a cartesian coordinate is
//     //   greater than 5, less than 5, or equal to 5
//     if y > 5 {
//         println!("greater than 5");
//     } else if y < 5 {
//         println!("less than 5");
//     } else {
//         println!("equal to 5");
//     }
// }



// enum Access {
//     Full,
// }

// fn one_two_three() -> (i32, i32, i32) {
//     (1, 2, 3)
// }

// let numbers = one_two_three();
// let (x, y, z) = one_two_three();
// println!("{:?}, {:?}", x, numbers.0); // 1
// println!("{:?}, {:?}", y, numbers.1); // 2
// println!("{:?}, {:?}", z, numbers.2); // 3

// NOTE It's recommended to Destructure when using tuples
// let (employee, access) = ("Jake", Access::Full);

// let coord = (2, 3);
// println("{:?}, {:?}", coord.0, coord.1);


// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
fn create_coord(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}

fn main() {
    // * Destructure the return value into two variables
    let (x, y) = create_coord(7, 3);
    // * Use an if..else if..else block to determine what to print
    if y > 5 {
        println!("GT 5");
    } else if y < 5 {
        println!("LT 5")
    } else if y == 5 {
        println!("Equal to 5")
    }
}

