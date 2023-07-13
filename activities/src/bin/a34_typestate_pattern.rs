// Topic: TypeState Pattern

// === Example 1
// struct BusTicket;
// struct BoardedBusTicket;
//
// impl BusTicket {
//     // NOTE: By using 'self' without '&' borrowing,
//     // once board() is called the BusTicket is destroyed!
//     fn board(self) -> BoardedBusTicket {
//         BoardedBusTicket
//     }
// }
//
// let ticket = BusTicket;
// let boarded = ticket.board();
//
// // Compile error
// ticket.board()

// === Example 2
// struct File<'a>(&'a str);
// impl <'a> File<'a> {
//     fn read_bytes(&self) -> Vec<u8> {
//         // ... read data ...
//     }
//
//     fn delete(self) {
//         // ... delete file ...
//     }
// }
//
// let file = File("data.txt");
// let data = file.read_bytes();
// // NOTE: Here we move 'self' (File) into the delete()
// // function, and once delete() fn ends, the file gets deleted
// // and is no longer available in the program.
// file.delete();
//
// // Compile error
// let read_again = file.read_bytes();

// // === Demo
// // NOTE: 'State' is a generic param (e.g, T,U,V)
// // Q: param == trait, right?
// // A: Eh, not exactly... 'State' is similar to 'Shape' from Prime's
// // NOTE: By using typestate transitions by using different
// // structures as types, Rust will check that the types align,
// // e.g., Employee<Signature> only implements a sign() function,
// // that returns a new type, Employee<Training>
// struct Employee<State> {
//     name: String,
//     state: State,
// }
// // Blanket implementation
// // NOTE: We could impl the state transitions on the
// // structures themselves (Agreement, Signature, etc.),
// // we're using this blanket impl on the main Employee
// // struct using a generic typestate contraints
// impl<State> Employee<State> {
//     fn transition<NextState>(self, state: NextState) -> Employee<NextState> {
//         // NOTE: This is a move/destroy since using 'self'
//         // If we borrow, then old Employee would still be accessible
//         Employee {
//             name: self.name,
//             state: state,
//         }
//     }
// }
//
// struct Agreement;
// struct Signature;
// struct Training;
// struct FailedTraining {
//     score: u8,
// }
// struct OnboardingComplete {
//     score: u8,
// }
// impl Employee<Agreement> {
//     fn new(name: &str) -> Self {
//         Self {
//             name: name.to_string(),
//             state: Agreement,
//         }
//     }
//
//     fn read_agreement(self) -> Employee<Signature> {
//         self.transition(Signature)
//     }
// }
//
// impl Employee<Signature> {
//     fn sign(self) -> Employee<Training> {
//         self.transition(Training)
//     }
// }
//
// impl Employee<Training> {
//     fn train(self, score: u8) -> Result<Employee<OnboardingComplete>, Employee<FailedTraining>> {
//         // NOTE: Again, we use 'self' to consume/destroy self and return new
//         if score >= 7 {
//             Ok(self.transition(OnboardingComplete { score }))
//         } else {
//             Err(self.transition(FailedTraining { score }))
//         }
//     }
// }
//
// fn main() {
//     let employee = Employee::new("George");
//     let onboarded = employee.read_agreement().sign().train(9);
//
//     match onboarded {
//         Ok(_emp) => println!("onboarding complete"),
//         Err(emp) => println!("training failed: {:?}", emp.state.score),
//     }
// }

// ===== Activity
// Topic Typestates
//
// Summary: An airline wants to reduce the amount of lost luggage by
// ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
//      * Check-in       (passenger gives luggage to airport)
//      * OnLoading      (luggage is loaded onto correct plane)
//      * OffLoading     (luggage is taken off plane at destination)
//      * AwaitingPickup (luggage at destination waiting for pickup)
//      * EndCustody     (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state
#[derive(Copy, Clone)]
struct LuggageId(usize);

struct Luggage(LuggageId);

// Various states a Luggage can be in
struct CheckIn(LuggageId);
struct OnLoad(LuggageId);
struct OffLoad(LuggageId);
struct AwaitingPickup(LuggageId);
struct EndCustody(LuggageId);

impl Luggage {
    fn new(id: LuggageId) -> Self {
        Luggage(id)
    }

    fn check_in(self) -> CheckIn {
        // Access the LuggageId value from
        // the field/tuple using 0 index
        CheckIn(self.0)
    }
}

impl CheckIn {
    fn onload(self) -> OnLoad {
        OnLoad(self.0)
    }
}

impl OnLoad {
    fn offload(self) -> OffLoad {
        // Off load the luggage at the destination airport
        OffLoad(self.0)
    }
}

impl OffLoad {
    // Place on luggage carousel to await pickup
    fn carousel(self) -> AwaitingPickup {
        AwaitingPickup(self.0)
    }
}

impl AwaitingPickup {
    fn pickup(self) -> (Luggage, EndCustody) {
        // Return a tuple with the actual luggage and end custody
        (Luggage(self.0), EndCustody(self.0))
    }
}

fn main() {
    let luggage_id = LuggageId(1);
    let luggage = Luggage(luggage_id);

    // Now run through the process
    // Q: Is this shadowing?
    let luggage = luggage.check_in().onload().offload().carousel();
    // Can destructure to get actual luggage piece
    let (luggage, _) = luggage.pickup();
    // TODO: Run cargo check
}
