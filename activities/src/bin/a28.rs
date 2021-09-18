// // === DEMO
// #[derive(Debug, Copy, Clone)]
// struct NeverZero(i32); // This is a Tuple structure!

// impl NeverZero {
//     // Ensure that this NeverZero Type can never contain zero!
//     // NOTE If we were using separate modules for code organization (recommended),
//     // then we'd need to make this function public so it can be accessed from
//     // other modules.
//     pub fn new(i: i32) -> Result<Self, String> {
//         if i == 0 {
//             Err("Cannot be ZERO".to_owned())
//         } else {
//             Ok(Self(i))
//         }
//     }
// }

// fn divide(a: i32, b: NeverZero) -> i32 {
//     // Let's access the inner value of our Result<NeverZero>
//     // and reassign to 'b'
//     // NOTE To access tuple fields we use the index
//     let b = b.0;  // i32

//     // Now we can divide knowing that it's not 0
//     a / b
// }

// fn main() {
//     match NeverZero::new(0) {
//         Ok(nz) => println!("{:?}", divide(10, nz)),
//         Err(e) => println!("{:?}", e),
//     }
// }


// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

struct Pants(Color);
impl Pants {
    pub fn new(clothing_type: Self, c: Color) -> Result<Self, String> {
        match c {
            Black => Ok(Self(Color::Black)),
            _ => Err("Not Black".to_owned())
        }
    }
}

enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

fn main() {
    match Pants::new(Pants, Color::Black) {
        Ok(clothing) => println!("{:?}", clothing),
        Err(e) => println!("{:?}", e),
    }
}
