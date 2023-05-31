// NOTES:
// - Map combinator .map() is used to easily transform data in your code
// - "Mapping" is the term used for converting one type of data to another
// - Option.map() ONLY applies IF a value (Some(value)) exists!
// - Even though map() + closures are handy, you still need to use match
// eventually since map() -> Option<T>
// - .map() can be chained multiple times!
//
// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn main() {
    let sam = find_user("sam").map(|id| User {
        user_id: id,
        name: "sam".to_owned(),
    });
    let matt = find_user("matt").map(|id| User {
        user_id: id,
        name: "matt".to_owned(),
    });
    let katie = find_user("katie").map(|id| User {
        user_id: id,
        name: "katie".to_owned(),
    });
    let mario = find_user("mario").map(|id| User {
        user_id: id,
        name: "mario".to_owned(),
    });

    let users = vec![sam, matt, katie, mario];

    for user in users {
        // NOTE Still should use match to verify the user was found (i.e., mario returns None)
        match user {
            Some(user) => println!("{:?}", user),
            None => println!("User not found"),
        }
    }
}
