// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    // Descending Order
    // let mut i = 4;
    // loop {
    //     println!("{:?}", i);
    //     i = i - 1;
    //     if i == 0 {
    //         println!("Loop completing, breaking out.");
    //         break;
    //     }
    // }
    // println!("Done!");

    // Ascending Order
    let mut n = 1;
    loop {
        println!("{:?}", n);
        if n == 4 {
            println!("n == 4, done!");
            break;
        }
        n = n + 1;
    }
}
