// NOTES
// - You can use ranges and convert to a Vec
// - You can collect a range and turn into a Vec,
// which then you can convert to an iterator and then
// use .map() and .filter() as needed.

fn main() {
    let range = 1..=3; // inclusive
    let range = 1..4; // exclusive

    for num in 1..4 {
        println!("{:?}", num);
    }

    for ch in 'a'..='f' {
        println!("{:?}", ch);
    }
}


