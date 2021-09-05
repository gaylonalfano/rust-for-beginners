// NOTES
// - You can think of modules as their own individual file
// - Use 'mod' keyword and then add all functionality inside it
// - A 'mod' is private by default so if you want to use it, you need to 'use'
// it e.g.: use greet::*;
// - It's good practice to group up your code into modules to organize. Below, we're
// grouping the printing functions and the mathematical functions.
// - These 'mod' (modules) will also show up in your documentation tool
// - If you want your module to use additional functionality from std library,
// you need to add it: use std::collections::HashMap;
//

mod greet {
    // NOTE Need to 'use' other functionality you want in your module
    // use std::collections::HashMap;
    pub fn hello() {
        println!("Hello!");
    }

    pub fn goodbye() {
        println!("Goodbye!");
    }
}

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}


fn main() {
    use greet::hello;  // Can use all: use greet::*;
    hello();
    greet::goodbye(); // Not 'used'
    println!("add: {:?}", math::add(1,1));
    use math::sub;
    println!("sub: {:?}", sub(3,2));
}
