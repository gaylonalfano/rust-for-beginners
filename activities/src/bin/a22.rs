// Topic: Testing
// === DEMO
// NOTES
// - cargo test --help
// - To run a test use: cargo test --bin [filename]
// - For test modules you need to include a macro that tells the
// compiler that this module is specific to testing
// - Testing in Rust works where if the test ABORTS the program,
// then the test fails. Otherwise, it passes.
// - To ABORT the program we use assert_eq! macro
// - assert_eq! has three args: value, value_expected, message
// - Access functions outside your test module by: use crate::*;
// - Run the test using: cargo test


fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

fn main() {}


// NOTE To test, we can create a new module and configure this module
// to tell compiler it's only for testing
#[cfg(test)]
mod test {
    // NOTE In Rust, a collection of code (i.e., all this code in this file) is
    // known as a CRATE. Let's access all of the other functionality in this
    // crate (outside of this test module).
    use crate::*;

    // NOTE All internal functions need the 'test' macro as well
    // to tell compiler that this function is testing other code
    // NOTE Functions used as tests cannot have arguments
    #[test]
    fn check_all_caps() {
        let result = all_caps("hello");
        let expected = String::from("hello");
        assert_eq!(result, expected, "String should be all uppercase");
    }

}

// === ACTIVITY
// requirements:
// * write tests for the existing program to ensure proper functionality.
//
// notes:
// * create at least two test cases for each function.
// * use `cargo test` to test the program.
// * there are intentional bugs in the program that need to be fixed.
//   * check the documentation comments for the functions to
//     determine how the they should operate.

/// ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// divides a and b.
fn div(a: i32, b: i32) -> option<i32> {
    // note need to add a check for dividing by zero
    if b == 0 {
        return none;
    } else {
        some(a / b)
    }
}

/// takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> string {
    format!("{}{}", first, second)
}

fn main() {}


#[cfg(test)]
mod test {
    // access all of the functions in this file/crate
    use crate::clamp;
    use crate::div;
    use crate::concat;

    #[test]
    fn check_clamp_lower() {
        let result = clamp(10, 100, 1000);
        let expected = 100;
        assert_eq!(result, expected, "unequal to lower");
    }

    #[test]
    fn check_clamp_upper() {
        let result = clamp(1000, 10, 100);
        let expected = 100;
        assert_eq!(result, expected, "unequal to upper");
    }

    #[test]
    fn check_div_some() {
        let result = div(8, 4);
        // note since div() -> option<i32>, the 'expected' value needs to
        // equal the some(i32) variant of option type!
        let expected = some(2);
        assert_eq!(result, expected, "failed to return some variant of option type");
    }

    // q: how to test the none variant for the div() function?
    // a: add a if b == 0 check inside the div() function logic!
    #[test]
    fn check_div_none() {
        let result = div(1, 0);
        let expected = none;
        assert_eq!(result, expected, "failed cannot divide by zero");
    }

    #[test]
    fn check_concat() {
        let result = concat("hello", "world");
        let expected = "helloworld".to_owned();
        assert_eq!(result, expected, "failed to concat two strings")
    }

}
