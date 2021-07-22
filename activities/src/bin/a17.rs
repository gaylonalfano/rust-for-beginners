// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn main() {
    let s1 = "HOWDY";
    println!("s1: {:?}", s1);
    let s2 = "nihao";

    let s1 = s1.to_lowercase();
    println!("s1: {:?}", s1);
    let s2 = s2.to_uppercase();
    println!("s2: {:?}", s2);
}
