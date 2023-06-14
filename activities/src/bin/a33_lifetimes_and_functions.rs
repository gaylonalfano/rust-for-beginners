// Topic: Lifetimes & Functions
//
// Summary:
// Create a program that compares which string is longer (highest length).
//
// Requirements:
// * The comparison must be done using a function named `longest`
// * No data may be copied (cannot use .to_owned() or .to_string())
// * If both strings are the same length, the first one should be returned
fn longest<'a>(one: &'a str, two: &'a str) -> &'a str {
    // NOTE Didn't need to use .len() to compare strings
    // NOTE: When you have multiple borrowed values, then you
    // need to specify where the returned borrowed value
    // comes from, therefore we need to add lifetimes.
    // If we only had one parameter, then we don't need to add
    // the lifetime.
    if two > one {
        two
    } else {
        one
    }
}

fn main() {
    // NOTE: Also, the variables 'short' and 'long' are
    // created BEFORE our function, so it all checks out.
    let short = "hello";
    let long = "this is a long message";
    println!("{}", longest(short, long))
}
