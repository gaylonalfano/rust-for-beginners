enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: f64,
}

fn main() {
    // Simple match
    let n = 3;
    match n {
        3 => println!("three"),
        other => println!("number: {:?}", other),
    }

    // Match on Enums that have variants that contain extra data (eg Discount::Flat(2))
    let flat = Discount::Flat(2);
    match flat {
        // NOTE We could use _, 2, or give a var name for readability.
        Discount::Flat(amount) => println!("Flat discount of: {:?}", amount),
        Discount::Flat(2) => println!("Flat 2"),
        Discount::Percent(_) => (), // Or, _ => () to ignore all Percent discounts
    }

    // Match on Structs
    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50.0,
    };
    match concert {
        // Match at specific price value
        // Ticket {price: 50.0, event} => println!("event @ {} = {}", price, event), // Error
        // Ticket {event, price: 50.0} => println!("event @ {} = {}", price, event), // Error
        Ticket {event, price} => println!("event @ {} = {}", price, event), // Works
        // Ticket {price: 50.0, event} => println!("event @ 50 = {:?}", event), // Works
        // Match on Ticket price only
        // NOTE To ignore other fields/props in struct use .. syntax
        Ticket {price, ..} => println!("price = {:?}", price),
    }

}
