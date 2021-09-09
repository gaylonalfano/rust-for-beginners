// Topic: Iterator
//
// Requirements:
// * Triple the value of each item in a vector.
// * Filter the data to only include values > 10.
// * Print out each element using a for loop.
//
// Notes:
// * Use an iterator chain to accomplish the task.

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    // * Triple the value of each item in a vector.
    let processed: Vec<i32> = data
        .iter()
        // Q: Why don't I have to add a reference to 3 (ie &3)?
        // A: Because we're not making a comparison/evaluation!
        // We are performing an operation!
        .map(|num: &i32| num * 3)
        // * Filter the data to only include values > 10.
        .filter(|num: &i32| num > &10)
        // NOTE collect() is what signals/executes all the functions
        // and closures we've used up to this point and packages it
        // all together to save into the variable.
        // NOTE * de-references
        .filter(|num| *num < 13)
        .collect();
        
    // * Print out each element using a for loop.
    for item in processed {
        println!("{:?}", item);
    }
}
