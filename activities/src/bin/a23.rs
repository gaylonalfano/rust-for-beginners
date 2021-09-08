// NOTES:
// - Option.is_some(&self) -> bool
// - The Debug Macro: dbg!(variable); is handy!

// fn main() {
//     let a: Option<i32> = None;  // Or, Some(1);
//     dbg!(a);


//     let a_is_some = a.is_some();
//     dbg!(a_is_some);

//     let a_is_none = a.is_none();
//     dbg!(a_is_none);

//     // .map() ONLY applies if there is SOME data, if not then nothing happens (NONE)
//     let a_mapped = a.map(|num| num + 1);
//     dbg!(a_mapped);


//     // .filter() body of closure must return true or false. If return value
//     // is TRUE, then we KEEP the data (e.g., return 'a'). Otherwise, we throwaway the optional data
//     // When we throw away, the return value is NONE
//     // NOTE .filter() borrows the inner value (&i32) so you need to borrow the
//     // the comparison value as well inside closure body
//     // NOTE You can borrow (&1) or you can access the raw data by DE-REFERENCING
//     // the borrowed 'num' using *num == 1;
//     // let a_filtered = a.filter(|num: &i32| num == &1);
//     let a_filtered = a.filter(|num| *num == 1);
//     dbg!(a_filtered);



//     // .or_else() returns the original data if present, otherwise we use a
//     // closure to specify what to return if 'a' originally has no data.
//     // NOTE Since we're dealing with Optional data (Some or None), we still
//     // need to 'match' on it later.
//     let a_or_else = a.or_else(|| Some(5));
//     dbg!(a_or_else);


//     // .unwrap_or_else()
//     // NOTE The difference between or_else() and unwrap_or_else() is that
//     // or_else() -> Optional data (i.e., Some or None). unwrap_or_else() actually
//     // takes out the inner data and stores in the variable (so you don't have to
//     // use 'match' to access inner data). If the original 'a' has no data, then
//     // the default value (i.e., 0 in this case) will be stored in variable.
//     // This is useful to set a default value if no data is originally provided
//     // or_else() -> Option<T> whereas unwrap_or_else() -> T
//     let unwrapped = a.unwrap_or_else(|| 0);
//     dbg!(unwrapped);
// }


// Topic: Option combinators
//
// Requirements:
// * Use combinators as described in the functions:
//   part_1, part_2, and part_3
//
// Notes:
// * Run `cargo test --bin a23` to check your program.
// * Only edit the part_1, part_2, and part_3 functions.

fn part_1() -> bool {
    // We are checking whether or not this particular user
    // has an access level. The "admin" user does have
    // an access level.
    // Note: Use is_some or is_none.
    maybe_access("admin").is_some()
}

fn part_2() -> Option<Access> {
    // "Root" is equivalent to Access::Admin, but it is
    // not listed in the maybe_access function.
    // Note: Use or_else and root().
    // NOTE We already have a root() -> Some(Access::Admin)
    // This means I can return the same thing explicitly using
    // .or_else(|| Some(Access::Admin)) or call 'root'
    // maybe_access("root").or_else(|| Some(Access::Admin))
    maybe_access("root").or_else(|| root())
}

fn part_3() -> Access {
    // "Alice" is not a listed user, so she will be a guest.
    // Note: Use unwrap_or_else.
    maybe_access("Alice").unwrap_or_else(|| Access::Guest)
}

#[derive(Debug, Eq, PartialEq)]
enum Access {
    Admin,
    User,
    Guest,
}

fn maybe_access(name: &str) -> Option<Access> {
    match name {
        "admin" => Some(Access::Admin),
        "gary" => Some(Access::User),
        _ => None,
    }
}

fn root() -> Option<Access> {
    Some(Access::Admin)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_part_1() {
        assert_eq!(part_1(), true, "Admins have an access level");
    }

    #[test]
    fn check_part_2() {
        assert_eq!(
            part_2(),
            Some(Access::Admin),
            "Root users have Admin access"
        );
    }

    #[test]
    fn check_part_3() {
        assert_eq!(part_3(), Access::Guest, "Alice is a guest");
    }
}
