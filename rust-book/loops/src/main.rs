fn main() {
    // let mut counter = 0;

    // // NOTE loop block is an expression so can set to right side of let statement
    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         // NOTE We can return values after breaking from loop
    //         break counter * 2;
    //     }
    // };

    // // === Countdown with WHILE loop
    // let mut number = 3;

    // while number != 0 {
    //     println!("{}!", number);

    //     number -= 1;
    // }

    // println!("The result is {}", result);
    // println!("LIFTOFF!!!");

    // === For loops
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // BAD/SLOW bc index could cause panic; slow bc compiler adds runtime code for conditional
    // while index < 5 {
    //     println!("The value is: {}", a[index]);

    //     index += 1;
    // }

    // BETTER
    for element in a.iter() {
        println!("The value is: {}", element);
    }
    
    // === Countdown w/ FOR loop
    // NOTE 1..4 is a Range type
    // NOTE rev() is used to reverse the Range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // Just iterating over a simple Range
    for n in 1..11 {
        println!("{}!", n);
    }
}
