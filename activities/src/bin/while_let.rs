// NOTES
// while...let... is really useful when working with iterators
// because many iterators return optional data
fn main() {
    let mut data = Some(3);

    // Create a while loop as long as there is Some data available
    while let Some(i) = data {
        // Count how many iterations we run throug
        println!("loop"); 
        // Exit the loop
        data = None;
    }

    let numbers = vec![1, 2, 3];
    let mut numbers_iter = numbers.iter();
    // iter().next() returns optional values as long as data is available or None
    while let Some(num) = numbers_iter.next() {
        println!("num={:?}", num);
    }
    println!("done");
}
