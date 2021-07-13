fn main() {
    another_function(5, 6);
    let z = five(); // statement (doesn't return a value)
    let w = plus_one(z);

    println!("The value of z is: {}", z);
    println!("The value of w is: {}", w);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    // The brackets {} is an expression (returns a value)
    5 // NOTE No ; because it's an EXPRESSION whose value we want to return
}

fn plus_one(n: i32) -> i32 {
    n + 1 // expression
    // n + 1;  // STATEMENT ERROR because return type is empty tuple '()' instead of 'i32'
    // NOTE Again, STATEMENTS do not evaluate to a value!
}
