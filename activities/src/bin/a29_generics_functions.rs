// Topic: Generics & Functions
//
// NOTES:
// * Generic functions are a way to write a function that can
// have a single parameter w/ multiple data types
// * Traits exhibit BEHAVIORS!
// * Trait is used as function param instead of data type
// -- Fn depends on existence of functions declared by trait
// * Less code to write
// -- Automatically works when new data types are introduced
// * Generic Syntax #1:
// fn function<T: Trait1, U: Trait2>(param1: T, param2: U) {}
// -- <T: Trait1, U: Trait2> is a generic constraint
// * Generic Syntax #2 (clearer imo):
// fn function<T, U>(param1: T, param2: U)
// where
//     T: Trait1 + Trait2,
//     U: Trait1 + Trait2 + Trait3,
// {}
// * Use impl Trait syntax with small num of traits and params
// * Use Generic syntax with small num of generic traits and params
// * Use Generic + where clause syntax with 2+ traits. Also easier to change
// the trait bounds later on.
// * Generics may increase size of binary, BUT it's really fast since
// each type of data knows where the fn is located in process memory!
//
// // === Quick Review of Traits ===
// trait Move {
//     fn move_to(&self, x: i32, y: i32);
// }
//
// struct Snake;
// impl Move for Snake {
//     fn move_to(&self, x: i32, y: i32) {
//         println!("slither to ({}, {})", x, y);
//     }
// }
//
// struct Grasshopper;
// impl Move for Grasshopper {
//     fn move_to(&self, x: i32, y: i32) {
//         println!("hop to ({}, {})", x, y);
//     }
// }
//
// // NOTE: Now that we have implemented Move trait,
// // we can use in our program. Impl Trait Syntax:
// fn make_move(thing: impl Move, x: i32, y: i32) {
//     thing.move_to(x, y);
// }
// // NOTE: Generic Function Syntax:
// fn make_move<T: Move>(thing: T, x: i32, y: i32) {
//     thing.move_to(x, y);
// }
//
// // NOTE: Generic + Where Syntax:
// fn make_move<T>(thing: T, x: i32, y: i32)
// where
//     T: Move,
// {
//     thing.move_to(x, y);
// }
//
// // NOTE: This generic fn is just shorthand syntax, which
// // gets expanded to. This is all managed during compilation,
// // and is known as Monomorphization. It's all automatic!
// // However, this does relate to Dynamic Dispatch and
// // Generic Structures (covered later).
// fn make_move(thing: Snake, x: i32, y: i32) {
//     thing.move_to(x, y);
// }
// fn make_move(thing: Grasshopper, x: i32, y: i32) {
//     thing.move_to(x, y);
// }
//
// fn main() {
//     let python = Snake {};
//     make_move(python, 2, 1);
//
//     let hopper = Grasshopper {};
//     make_move(hopper, 3, 3);
// }

// // ======= Demo Generic Function =======
// trait CheckIn {
//     fn check_in(&self);
//     // Moving luggage, boarding plane, etc.
//     fn process(&self);
// }
//
// struct Pilot;
// impl CheckIn for Pilot {
//     fn check_in(&self) {
//         println!("Checked in as pilot")
//     }
//
//     fn process(&self) {
//         println!("Pilot enters cockpit")
//     }
// }
//
// struct Customer;
// impl CheckIn for Customer {
//     fn check_in(&self) {
//         println!("Checked in as customer")
//     }
//
//     fn process(&self) {
//         println!("Customer boards plane")
//     }
// }
// struct Cargo;
// impl CheckIn for Cargo {
//     fn check_in(&self) {
//         println!("Checked in as cargo")
//     }
//
//     fn process(&self) {
//         println!("Cargo stored on plane")
//     }
// }
//
// // NOTE: Out param type T must impl the CheckIn trait!
// // This ensures that T has check_in() and process() functions
// fn process_item<T: CheckIn>(item: T) {
//     item.check_in();
//     item.process();
// }
//
// fn main() {
//     let paul = Customer {};
//     let kathy = Pilot;
//     let cargo1 = Cargo;
//     let cargo2 = Cargo;
//     process_item(paul);
//     process_item(kathy);
//     process_item(cargo1);
//     process_item(cargo2);
// }

// ======== Activity =========
// Requirements:
// * Create a function that accepts the Priority trait as a generic parameter
// * The fn should print out the guest and their Priority
// * Use the fn with at least two different guests
//
// Notes:
// * Use the debug token "{:?}" to print info
// * Use the compiler to guide you to the correct generic constraints needed

#[derive(Debug)]
enum ServicePriority {
    High,
    Standard,
}

trait Priority {
    fn get_priority(&self) -> ServicePriority;
}

#[derive(Debug)]
struct ImportantGuest;
impl Priority for ImportantGuest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::High
    }
}

#[derive(Debug)]
struct Guest;
impl Priority for Guest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::Standard
    }
}

// NOTE: Since using Debug print, we need to further constrain
// our Trait bounds! e.g., T: Priority + std::fmt::Debug
fn print_priority_details<T: Priority + std::fmt::Debug>(person: T) {
    println!("{:?} priority: {:?}", person, person.get_priority())
}

fn main() {
    let joe = ImportantGuest;
    let sue = Guest;
    print_priority_details(joe);
    print_priority_details(sue);
}
