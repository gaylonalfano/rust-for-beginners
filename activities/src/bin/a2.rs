// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn add_two_numbers(a: i32, b: i32) -> i32 {
    // NOTE Don't add ';' or it becomes a statement, not an expression (returns a value)
    a + b // Returns the value as i32 type
}

fn display_result(result: i32) {
    println!("Result is: {:?}", result);
}

fn main() {
    let result = add_two_numbers(3, 7);
    display_result(result);
}
