fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // === Method 1: Traditional
    // let mut plus_one: Vec<i32> = vec![];
    // for num in numbers {
    //     plus_one.push(num + 1);
    // }

    // === Method 2: Iterators
    // NOTE Iterators don't collect into anything so you need to collect
    // You can use Vec<_> if you don't know the Type
    // let plus_one: Vec<i32> = numbers
    //     .iter()
    //     .map(|num: &i32| num + 1)
    //     .collect(); // Convert values into a Vector Vec<i32>

    let new_numbers: Vec<_> = numbers.iter().filter(|num| num <= &&3).collect();

    let numbers = vec![1, 2, 3, 4, 5];
    let find_me: Option<_> = numbers.iter().find(|num: i32| num == 3);

    // Return number of elements in iterator
    let count = numbers.iter().count();

    // Q: Rust Analyzer complains when I try to add Type i32. Why?
    let last: Option<_> = numbers.iter().last(); // 5

    let numbers = vec![1, 2, 3, 4, 5];

    // NOTE Need to use '_' as Type or complains
    let min: Option<_> = numbers.iter().min(); // 1

    // NOTE Need to use '_' as Type or complains
    let max: Option<_> = numbers.iter().max(); // 5

    let take: Vec<_> = numbers.iter().take(3).collect(); // [1, 2, 3]

    println!("{:?}", new_numbers);
}
