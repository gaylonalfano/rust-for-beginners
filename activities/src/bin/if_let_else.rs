// NOTES
// - NOTE Generally speaking it's best to just use 'match' but
// this is an option if you ONLY care about one specific thing.
// - Use this for performing actions with some data
// - if...let... is ONLY shorter than a traditional match block
// when you ONLY care about the Some variant. 
// - if...let...else is EXACTLY what a 'match' block does
// - You can also use if...let... on enum variants
// - NOTE 


enum Color {
    Red,
    Blue,
    Green,
}

fn main() {
    // Traditional match on Optional data:
    let maybe_user = Some("Jerry");
    match maybe_user {
        Some(user) => println!("user={:?}", user),
        None => println!("no user"),
    }

    // NOTE If there's a time when we ONLY care if there is a user,
    // otherwise we don't care, then this is a good time to use the
    // if...let...else pattern
    // NOTE This is functionally equivalent to ONLY matching on Some
    // variant of our Optional data.
    if let Some(user) = maybe_user {
        println!("user={:?}", user);
    } else {
        // NOTE To make it EXACTLY the same as the (top) match block, we
        // simply add this 'else' block:
        println!("no user");
    }

    // NOTE Again, this if...let... pattern only matters if you ONLY care
    // about the Some variant. Otherwise, just use 'match' like usual.
    let red = Color::Red;
    if let Color::Red = red {
        println!("it's red!");
    } else {
        println!("It's NOT red");
    }
}
