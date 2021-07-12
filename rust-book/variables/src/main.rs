use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // === Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);
    
    // === Floating Point Numbers
    let x = 2.0; // f64 double-point precision
    let y: f32 = 3.0; // f32 single-point precision

    // === Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    // === Boolean
    let t = true;
    let f: bool = false; // explicit type annotation

    // === Character Type (single character - char)
    let c = 'z';

    // === Compound Types: Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is {}", y);

    let w: (i32, f64, u8) = (200, 3.2, 4);
    let two_hundred = w.0;
    let three_point_two = w.1;
    let four = w.2;

    // === Compound Types: Array
    // NOTE Must be the same type! Arrays also have fixed length like tuples.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5]; // initializing array (more concise)
    let first = a[0];
    let second = a[1];

    // === Invalid Array Element Access
    // NOTE This will result in a RUNTIME error if out of bounds
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    // Shadowing index to transform type
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );


}
