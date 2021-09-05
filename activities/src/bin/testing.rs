// NOTES
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
        let expected = String::from("HELLO");
        assert_eq!(result, expected, "String should be all uppercase");
    }

}
