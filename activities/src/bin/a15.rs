// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

// MY ATTEMPT
enum Ticket {
    Backstage(f64, String), // price, name
    Standard(f64),          // price
    Vip(f64, String),       // price, name
}

struct Event {
    name: String,
    date: String,
    ticket: Ticket,
}

fn main() {
    let tickets = vec![
        Event {
            name: String::from("Radiohead"),
            date: "5/29/1981".to_owned(),
            ticket: Ticket::Vip(100.5, "Mario".to_owned()),
        },
        Event {
            name: String::from("Radiohead"),
            date: "5/29/1981".to_owned(),
            ticket: Ticket::Standard(40.9),
        },
        Event {
            name: String::from("Radiohead"),
            date: "5/29/1981".to_owned(),
            ticket: Ticket::Backstage(80.0, "Luigi".to_owned()),
        },
        Event {
            name: String::from("Radiohead"),
            date: "5/29/1981".to_owned(),
            ticket: Ticket::Vip(10.0, "Peach".to_owned()),
        },
    ];

    for event in tickets {
        match event.ticket {
            // NOTE Single expressions (no parens) are followed by commas, whereas as {} create a
            // new scope for the match arm/pattern.
            Ticket::Standard(price) => {
                println!("(arm 1) name: {:?}, date: {:?}, price: {:?}", event.name, event.date, price)
            }
            Ticket::Backstage(price, ticket_holder) => println!(
                "(arm 2) name: {:?}, date: {:?}, price: {:?}, ticket_holder: {:?}",
                event.name, event.date, price, ticket_holder
            ),
            // "match guard" using conditional
            // https://www.udemy.com/course/rust-coding-for-beginners/learn/lecture/23530310#questions/15403030
            Ticket::Vip(price, ticket_holder) if ticket_holder == "Mario" => {
                println!("(arm 3) name: {:?}, price: {:?}", ticket_holder, price)
            }
            // Using 'at binding' to only when price is at 10
            // Eg. Ticket {price: p @ 10, event} => println!("event @ {} = {}, p, event")
            // FIXME Can't get 'at binding' to work on this arm because Ticket is enum (not struct)
            // Ticket::Vip(price: p @ 10.0, ticket_holder) => {
            //     println!("(arm 4) name: {:?}, date: {:?}, price: {:?}", event.name, event.date, p)
            // }
            // Match on price only
            Ticket::Vip(price, ..) => println!("(arm 5) date: {:?}, price: {:?}", event.date, price),
        }
    }
}
