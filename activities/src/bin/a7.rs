// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

// NOTE Enum + Match is a great and safe combo!

// * Use an enum with color names as variants
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}
// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_color_name(color_name: Color) {
    // * Use a match expression to determine which color
    //   name to print
    match color_name {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::Yellow => println!("yellow"),
    }
}
fn main() {
    print_color_name(Color::Yellow);
}
