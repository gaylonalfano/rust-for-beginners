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
    // match concert {
    //     // Match at specific price value
    //     // Ticket {price: 50.0, event} => println!("event @ {} = {}", price, event), // Error
    //     // Ticket {event, price: 50.0} => println!("event @ {} = {}", price, event), // Error
    //     Ticket {event, price} => println!("event @ {} = {}", price, event), // Works
    //     // Ticket {price: 50.0, event} => println!("event @ 50 = {:?}", event), // Works
    //     // Match on Ticket price only
    //     // NOTE To ignore other fields/props in struct use .. syntax
    //     Ticket {price, ..} => println!("price = {:?}", price),
    // }

    match concert {
        // In this match arm, we are saying that we want to get the `event` information
        // from the structure, and we only want it when the price is 50. We aren't interested
        // in the price when using this syntax, just that the structure has a specific price.
        // Ticket {price: 50.0, event} => println!("(match arm 1) event @ 50 = {:?}", event),

        // In this match arm, we are saying that we want to get the `event` information
        // from the structure, and we only want it when the price is 50. Additionally, we are
        // binding the variable `p` (that we just made up) to the value of 50.0 using the `at` (@)
        // symbol. This is called an `at binding`. The `at binding` both checks the value in
        // `price`, and binds it to `p` whenever the price is `50.0.` `p` can then be used to print the price.
        // You can also use `price` instead of `p, but it might become ambiguous depending on how
        // your match arm is structured.
        Ticket {
            price: p @ 50.0,
            event,
        } => println!("(match arm 2) event @ {} = {}", p, event),
        // Ticket {price: banana @ 50.0, event} => println!("(match arm 2) event @ {} = {}", banana, event),

        // In this match arm, we are saying that we want to get the `event` information and the
        // `price` information from the structure. But we only want to get these pieces of
        // information when the price is 50.0. This is called a `match guard`.
        Ticket { price, event } if price == 50.0 => {
            println!("(match arm 3) event @ {} = {}", price, event)
        }

        // Match on Ticket price only
        Ticket { price, .. } => println!("price = {:?}", price), // Works
    }
}
