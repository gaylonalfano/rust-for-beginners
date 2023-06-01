// Topic: Generics & Functions
//
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

fn main() {}
