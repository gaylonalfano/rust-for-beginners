fn main() {
    // let number = 6;

    // === Using if/else
    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }
    
    // === Using match
    // match number % 4 {
    //     0 => println!("number is divisible by 4"),
    //     _ => println!("number is not divisible by 4")
    // }

    // === Using if in a 'let' Statement
    // NOTE 'if' is an EXPRESSION, therefore you can use it on the right side of a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
