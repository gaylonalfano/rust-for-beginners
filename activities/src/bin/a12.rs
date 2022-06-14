// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// // * Use an enum for the box color
// enum BoxColor {
//     Red,
//     Blue,
//     Green,
// }

// // Use impl to implement functionality for BoxColor
// impl BoxColor {
//     fn print(&self) {
//         // NOTE Use match!
//         match self {
//             BoxColor::Red => println!("red"),
//             BoxColor::Blue => println!("blue"),
//             BoxColor::Green => println!("green"),
//         }
//     }
// }

// struct BoxDimensions {
//     width: f64,
//     height: f64,
//     depth: f64,
// }

// impl BoxDimensions {
//     // NOTE &self refers to a BoxDimensions created elsewhere (unlike Self)
//     fn print(&self) {
//         println!("width: {:?}", self.width);
//         println!("height: {:?}", self.height);
//         println!("depth: {:?}", self.depth);
//     }
// }


// // * Use a struct to encapsulate the box characteristics
// struct Box {
//     weight: f64,
//     // color: match BoxColor {
//     //     BoxColor::Red => println!("red"),
//     //     BoxColor::Blue => println!("blue"),
//     //     BoxColor::Green => println!("green"),
//     // }
//     color: BoxColor,
//     dimensions: BoxDimensions
// }


// impl Box {
//     // NOTE Should use new() per conventions
//     fn new(weight: f64, color: BoxColor, dimensions: BoxDimensions) -> Self {
//         Self {
//             weight,
//             color,
//             dimensions,
//         }
//     }


//     // * Implement functionality on the box struct to create a new box
//     // My Attempt
//     // fn create_green_box() -> Self {
//     //     Self {
//     //         weight: 22.0,
//     //         color: BoxColor::Green,
//     //         dimensions: Dimensions {
//     //             width: 5.2,
//     //             height: 8.4,
//     //             depth: 10.0,
//     //         }
//     //     }
//     // }
//     // * Implement functionality on the box struct to print the characteristics
//     fn print(&self) {
//         println!("weight = {:?}", self.weight);
//         // NOTE Shouldn't use println macro when calling custom print() functions
//         // println!("dimensions = {:?}", self.dimensions.print());
//         // println!("color = {:?}", self.color.print());
//         self.dimensions.print();
//         self.color.print();
//     }
// }

// fn main() {
//     // NOTE Gotta first create BoxDimensions before creating a new Box
//     let small_dimensions = BoxDimensions {
//         width: 5.2,
//         height: 8.4,
//         depth: 10.0,
//     };

//     let small_box = Box::new(9.2, BoxColor::Blue, small_dimensions);
//     small_box.print();
// }


// // DEMO
// struct Temperature {
//     degrees_f: f64,
// }

// impl Temperature {
//     // Can also use impl to create other types of temperatures
//     fn freezing() -> Self {
//         Self { degrees_f: 32.0 }
//     }


//     // Using &self reference instead
//     fn show_temp(&self) {
//         println!("{:?} degrees F", self.degrees_f);
//     }

//     // fn show_temp(temp: Temperature) {
//     //     println!("{:?} degrees F", temp.degrees_f);
//     // }

//     fn boiling() -> Self {
//         Self {degrees_f: 212.0 }
//     }

// }

// fn main() {
//     let hot = Temperature {
//         degrees_f: 99.9
//     };

//     // NOTE We don't have to borrow (reference) since only calling hot
//     // once, which means hot is moved to show_temp for ownership.
//     // Temperature::show_temp(hot); // impl
//     hot.show_temp(); // impl + self

//     let cold = Temperature::freezing();
//     cold.show_temp();
//     // NOTE Can call multiple times since we're using a reference &self
//     cold.show_temp();
//     cold.show_temp();
//     cold.show_temp();

//     let boiling = Temperature::boiling();
//     boiling.show_temp();
//     boiling.show_temp();
// }


// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
#[derive(Debug)]
struct Box {
    weight: f64,
    dimensions: BoxDimensions,
    color: BoxColor
}

#[derive(Debug)]
struct BoxDimensions {
    length: f64,
    width: f64,
    height: f64
}
// * Use an enum for the box color
#[derive(Debug)]
enum BoxColor {
    Brown,
    White,
    Black,
}

// * Implement functionality on the box struct to create a new box
impl Box {
    fn new(weight: f64, dimensions: BoxDimensions, color: BoxColor) -> Self {
        Self {
            weight,
            dimensions,
            color
        }
    }

    fn print_box_details(&self) {
        println!("details: {:?}", self)
    }
}

// * Implement functionality on the box struct to print the characteristics

fn main() {
    let brown_box = Box::new(11.2, BoxDimensions { length: 4.0, width: 2.0, height: 8.0 }, BoxColor::Brown);
    // println!("{:?}", brown_box);
    brown_box.print_box_details();
}

