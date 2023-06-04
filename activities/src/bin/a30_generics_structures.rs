// Topic: Generics & Structures
// * NOTE: Similar to Prime's Collidable Trait
//
// NOTES:
// * Generic Structures allow you to store any type within a structure
//   - Trait bounds restrict the type of data the struct can utilize.
//     This is known as "Generic Constraints"
// * Useful for making own data collections
// * Easier to change as program expands. New data types can utilize
//   generic structures
// * Conceptual Example: Template Rendering Engine
//      - Template Source Paths
//      - Variable substitution data
//      - Generic render target
//          - File
//          - Terminal
//          - Image
//          - Bytes
//          - **Any new render target can simply impl the
//            generic structure
// * Syntax Options:
//      struct Name<T: Trait1 + Trait2, U: Trait3> { field1: T, field2, U}
//      struct Name<T,U> where T: Trait1 + Trait2, U: Trait3, { field1: T, field2: U}
// * Can't mix generic structures in a single collection (eg Vec),
//   since technically they're different structs after expanded.
// * Generic Implementation aka 'Blanket Implementation' -- See Prime's course!

// // ===== Example =====
// // Represents some type of seat at a venue or vehicle, etc.
// trait Seat {
//     fn show(&self);
// }
//
// // Ticket struct is generic over the Seat trait. This means
// // we can use this struct with any type that implements Seat,
// // but no other types can be used with this structure.
// struct Ticket<T: Seat> {
//     location: T,
// }
// // NOTE: This Generic Structure (Ticket<T: Seat>) gets expanded
// // during compilation, similar to generic functions!
// // struct AirlineTicket {
// //     location: AirlineSeat,
// // }
// // struct ConcertTicket {
// //     location: ConcertSeat,
// // }
//
// // Before we can use Ticket, we need some types that impl Seat:
// #[derive(Clone, Copy)]
// enum ConcertSeat {
//     FrontRow,
//     MidSection(u32),
//     Back(u32),
// }
//
// impl Seat for ConcertSeat {
//     fn show(&self) {
//         match self {
//             ConcertSeat::FrontRow => println!("FrontRow seat"),
//             ConcertSeat::MidSection(s) => {
//                 println!("MidSection seat: {}", s)
//             }
//             ConcertSeat::Back(s) => {
//                 println!("Back seat: {}", s)
//             }
//         }
//     }
// }
//
// #[derive(Clone, Copy)]
// enum AirlineSeat {
//     BusinessClass,
//     Economy,
//     FirstClass,
// }
//
// impl Seat for AirlineSeat {
//     fn show(&self) {
//         match self {
//             AirlineSeat::Economy => println!("Economy"),
//             AirlineSeat::BusinessClass => println!("BusinessClass"),
//             AirlineSeat::FirstClass => println!("FirstClass"),
//         }
//     }
// }
//
// // Create our fn to handle the Generic Ticket Struct
// // Q: Could I turn this into a Generic Function as well?
// // A: Yes! This is preferred to maximize usage!
// // fn ticket_info(ticket: Ticket<ConcertSeat>) {
// //     ticket.location.show()
// // }
//
// // Q: What's the syntax for a generic fn for this?
// // A: See below:
// // Again 'T' is out type and 'Seat' is the trait that 'T'
// // is constrained by!
// fn ticket_info<T: Seat>(ticket: Ticket<T>) {
//     ticket.location.show()
// }
// // NOTE: Again, this Generic Function expands in compilation to:
// // fn airline_ticket_info(ticket: AirlineTicket) {
// //     ticket.location.show();
// // }
// //
// // fn concert_ticket_info(ticket: ConcertTicket) {
// //     ticket.location.show();
// // }
//
// fn main() {
//     let taylor_swift_concert = Ticket {
//         location: ConcertSeat::MidSection(29),
//     };
//
//     let qatar_airways = Ticket {
//         location: AirlineSeat::FirstClass,
//     };
//
//     // NOTE: You cannot simply add these tickets to a Vec
//     // and then loop over and call ticket_info(ticket).
//     // This errors since technically they are different structures,
//     // thanks to expanded compilation, and Vectors can't handle
//     // multiple types this way.
//
//     ticket_info(taylor_swift_concert);
//     ticket_info(qatar_airways);
// }

// // ====== Generic Structures with impl Blocks ======
// trait Game {
//     fn name(&self) -> String;
// }
//
// enum BoardGame {
//     Chess,
//     Monopoly,
// }
// impl Game for BoardGame {
//     fn name(&self) -> String {
//         match self {
//             Self::Chess => "Chess".to_owned(),
//             Self::Monopoly => "Monopoly".to_owned(),
//         }
//     }
// }
//
// enum VideoGame {
//     Playstation,
//     Xbox,
// }
// impl Game for VideoGame {
//     fn name(&self) -> String {
//         match self {
//             Self::Xbox => "Xbox".to_owned(),
//             Self::Playstation => "Playstation".to_owned(),
//         }
//     }
// }
//
// // Next, we have a generic structure that is generic over any
// // structure that implements the Game trait.
// struct Playroom<T: Game> {
//     game: T,
// }
//
// // This is a CONCRETE Implementation
// // Q: How to make this impl generic? impl<T> Playroom<T>?
// // A: impl<T: Trait1 + Trait2, U: Trait3> Name<T, U> {
// //      fn func(&self, arg1: T, arg2: U) {...}
// // }
// // impl Playroom<BoardGame> {
// //     pub fn cleanup(&self) {
// //         println!("Cleaning up {}", self.game.name())
// //     }
// // }
//
// // This is a GENERIC Implementation
// impl<T: Game> Playroom<T> {
//     pub fn game_info(&self) {
//         println!("{}", self.game.name());
//     }
//     pub fn cleanup(&self) {
//         println!("Cleaning up {}", self.game.name())
//     }
// }
//
// fn main() {
//     let video_room = Playroom {
//         game: VideoGame::Xbox,
//     };
//
//     let board_room = Playroom {
//         game: BoardGame::Monopoly,
//     };
//
//     video_room.game_info();
//     video_room.cleanup();
//     board_room.game_info();
//     board_room.cleanup();
// }

// ====== Activity ======
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

// NOTE: Jayson's solution used 'marker traits', which are empty
// traits but the compiler still constrains/checks everything.
trait Body {
    fn info(&self) -> String;
}
trait Color {
    fn info(&self) -> String;
}

// NOTE: Jayson built empty structs instead of my enums approach:

// struct Car;
// impl Body for Car {}
// struct Truck;
// impl Body for Truck {}
//
// struct Red;
// impl Color for Red {}
// struct Blue;
// impl Color for Blue {}

#[derive(Debug)]
enum VehicleBodies {
    Truck,
    Car,
    Scooter,
    Van,
}
impl Body for VehicleBodies {
    fn info(&self) -> String {
        match self {
            VehicleBodies::Truck => "Truck".to_owned(),
            VehicleBodies::Car => "Car".to_owned(),
            VehicleBodies::Scooter => "Scooter".to_owned(),
            VehicleBodies::Van => "Van".to_owned(),
        }
    }
}

#[derive(Debug)]
enum VehicleColors {
    Red,
    White,
    Black,
    Blue,
    Green,
}
impl Color for VehicleColors {
    fn info(&self) -> String {
        match self {
            VehicleColors::Red => String::from("Red"),
            VehicleColors::White => String::from("White"),
            VehicleColors::Black => String::from("Black"),
            VehicleColors::Blue => String::from("Blue"),
            VehicleColors::Green => String::from("Green"),
        }
    }
}

#[derive(Debug)]
struct Vehicle<B: Body, C: Color> {
    body: B,
    color: C,
}

// If we don't add these trait bounds, the compiler will
// let us know/complain.
impl<B: Body, C: Color> Vehicle<B, C> {
    fn new(body: B, color: C) -> Self {
        Self { body, color }
    }
}

fn main() {
    let white_truck = Vehicle::new(VehicleBodies::Truck, VehicleColors::White);
    let red_scooter = Vehicle::new(VehicleBodies::Scooter, VehicleColors::Red);
    println!("Body {:?}, Color {:?}", white_truck.body, white_truck.color);
    println!("Body {:?}, Color {:?}", red_scooter.body, red_scooter.color);
}
