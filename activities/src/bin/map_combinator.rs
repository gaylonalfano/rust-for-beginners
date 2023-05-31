// NOTES:
// - Map combinator .map() is used to easily transform data in your code
// - "Mapping" is the term used for converting one type of data to another
// - Option.map() ONLY applies IF a value (Some(value)) exists!
// - Even though map() + closures are handy, you still need to use match
// eventually since map() -> Option<T>
// - .map() can be chained multiple times!

fn maybe_num() -> Option<i32> {
    // Some(3) => 3,
    // None => None,
}

fn maybe_word() -> Option<String> {
    // Some(string) => string,
    // None => None,
}

fn main() {
    // // === Example of transforming the data
    // // Verbose way to retrieve number and add one
    // let plus_one = match maybe_num() {
    //     Some(num) => Some(num + 1),
    //     None => None,
    // };

    // // Using Closures instead and Option.map() method to TRANSFORM
    // let plus_one = maybe_num().map(|num| num + 1);

    // === Example of creating a new TYPE (str > int)
    // NOTE This conversion makes word_length type Option<i32>
    // NOTE We can even chain multiple .map() calls!
    // This makes it super easy to chain until you get the data type
    // you want.
    let word_length: Option<usize> = maybe_word().map(|word| word.len()).map(|len| len * 2);
}
