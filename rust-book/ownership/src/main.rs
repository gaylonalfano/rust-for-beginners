// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#memory-and-allocation
// fn main() {
//     {                      // s is not valid here, it’s not yet declared
//         let s = "hello";   // s is valid from this point forward

//         // do stuff with s
//     }                      // this scope is now over, and s is no longer valid
//                            // and drop() is called
// }


// fn main() {
//     let mut s = String::from("hello");

//     s.push_str(", world!"); // push_str() appends a literal to a String

//     println!("{}", s); // This will print `hello, world!`
// }


// === Ways Variables and Data Interact: MOVE
// Integer
// fn main() {
//     let x = 5;
//     let y = x; // these two 5 values are pushed onto the STACK
// }

// String
// NOTE A String is made up of three parts:
// 1. The pointer to the memory that holds the contens of the string
// 2. Length (how much memory in bytes received from allocator)
// 3. Capacity
// This GROUP of data is stored on the STACK
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1; // s1 is MOVED to s2 (kinda like a shallow copy plus ownership of Rust)
//     // NOTE s1's group of data (pointer, length, capacity) is stored on STACK.
//     // The heap holds the contents (like an array) e.g., ['h', 'e', 'l', 'l', 'o']
//     // It's the pointer that points to 0 index of the help memory (I think).
//     println!("{}, world!", s1); // MOVE ERROR!
// }

// === Ways Variables and Data Interact: Clone (DEEP Copy)
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone(); // Creates a DEEP COPY of HEAP data, not just stack data.

//     println!("s1 = {}, s2 = {}", s1, s2);
// }

// === Stack-Only Data: Copy Trait
// NOTE There is no MOVE performed here because types such as integers that
// have a KNOWN size at compile time are stored entirely on the STACK!
// Therefore, it's cheap/easy to make copies of actual values.
// fn main() {
//     let x = 5;
//     let y = x; // x is not moved to y like it happens with type String

//     println!("x = {}, y = {}", x, y);
// }


// === Ownership and Functions
// Transferring ownership of return values
// fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it's okay to still
//                                     // use x afterward

// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.


// Returning ownership of parameters
// NOTE This example is just showing how to return ownership of parameters,
// HOWEVER, this is too much work! Thankfully Rust has references!
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

