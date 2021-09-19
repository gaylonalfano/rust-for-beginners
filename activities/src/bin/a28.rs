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
//   NOTE The reason for even wrapping Color enum with a new custom
//   TUPLE type/struct, is ensure that we don't accidentally swap the
//   clothing items and display the wrong color info. By creating these
//   separate, unique types of data (PantsColor, ShirtColor, etc) we will
//   get an error if we accidentally switch the variables
//   NOTE You can even restrict certain clothing types to only allow
//   certain colors

#[derive(Debug)]
struct PantsColor(Color);
impl PantsColor {
    // NOTE You can even restrict certain clothing types to only allow
    // certain colors
    pub fn new(color: Color) -> Result<Self, String> {
        match color {
            Color::Purple => Err("Pants cannot be purple".to_owned()),
            c => Ok(Self(c)),
        }
    }
}

#[derive(Debug)]
struct ShirtColor(Color);
impl ShirtColor {
    pub fn new(c: Color) -> Self {
        Self(c)
    }
}

#[derive(Debug)]
struct ShoesColor(Color);
impl ShoesColor {
    pub fn new(c: Color) -> Self {
        Self(c)
    }
}

#[derive(Debug)]
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

fn print_shirt_color(color: ShirtColor) {
    println!("shirt color = {:?}", color);
}

fn print_pants_color(color: PantsColor) {
    println!("pants color = {:?}", color);
}

fn print_shoes_color(color: ShoesColor) {
    println!("shoes color = {:?}", color);
}

fn main() {
    let gray_shirt = ShirtColor::new(Color::Gray);
    let black_pants = PantsColor::new(Color::Black).unwrap_or(PantsColor(Color::Black));
    // Q: How to not allow purple pants? How to get the inner value for Result<PantsColor, String>?
    // A: TL;DR - Better would be to create another enum for pant colors where all are valid,
    // rather than trying to restrict via using this Result<T, E> approach. Ideally you make
    // Struct::new() ALWAYS work!
    // A: If you must stick with Result, then you could use the .unwrap_or() method
    let purple_not_allowed_pants = PantsColor::new(Color::Purple).unwrap_or(PantsColor(Color::Custom(String::from("purple_not_allowed"))));
    let blue_shoes = ShoesColor::new(Color::Blue);
    let orange_pants = PantsColor::new(Color::Custom(String::from("orange"))).unwrap_or(PantsColor(Color::Black));

    print_pants_color(black_pants);
    print_pants_color(purple_not_allowed_pants);
    print_shirt_color(gray_shirt);
    print_shoes_color(blue_shoes);
    print_pants_color(orange_pants);
}
