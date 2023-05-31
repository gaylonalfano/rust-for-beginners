// NOTES:
// - Closures are anonymous (no names, but you can give one)
// - Closures must always be defined within ANOTHER function!
// - Use | character to define e.g. |a: i32, b: i32| -> i32 {...}
// - You call it just like a regular function
// - Succinct syntax: |a, b| a + b;
// - Also used for passing a function as another function's parameter.
// This is referred as a "Function Combinators".

fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(1, 1);

    // === Closure VERBOSE
    let add = |a: i32, b: i32| -> i32 { a + b };
    // Then, you can call the Closure like a regular fn!
    // let sum = add(1, 1);

    // === Closure w/o name and type annotations
    // NOTE Rust is smart enough to know the types
    let add = |a, b| a + b;
    let sum = add(1, 1);
}
