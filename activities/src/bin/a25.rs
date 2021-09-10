// === EXAMPLE 1
trait Noise {
    // Define functions that describe what the trait does
    fn make_noise(&self);
}

struct Person;
impl Noise for Person {
    fn make_noise(&self) {
        println!("Hello");
    }
}

struct Dog;
impl Noise for Dog {
    fn make_noise(&self) {
        println!("Woof");
    }
}

// Create a function that utilizes the Noise trait
// NOTE The function parameter does not know whether the
// passed arg is an enum or struct. It simply only accepts
// SOMETHING that has implemented this Noise trait!
// The thing/arg MUST impl this trait in order to use this fn!
fn hello(noisy: impl Noise) {
    noisy.make_noise();
}

fn main() {
    hello(Person {});
    hello(Dog); // Optional: Add '{}' if you want
}


// === EXAMPLE 2
// NOTE A trait can have multiple functions to describe it
trait Racer {
    fn go (&self);
    fn is_ready(&self) -> bool;
    fn checkpoint(&self, position: i32);
}


// DEMO
// 1. Create a trait to describe an object falling
trait Fall {
    // NOTE This function won't return anything.
    // We'll just print messages for this demo
    fn hit_ground(&self); 
}

// 2. Create some data to work with e.g, vase, ball, cat, etc.
struct Vase;
impl Fall for Vase {
    fn hit_ground(&self) {
        println!("The vase broke!");
    }
}

struct Cat;
impl Fall for Cat {
    fn hit_ground(&self) {
        println!("The cat casually walked away");
    }
}

// 3. Create a function that utilizes the Trait
fn fall(falling: impl Fall) {
    falling.hit_ground();
}

fn main() {
    fall(Cat);
    fall(Vase);
}


// === EXERCISE
// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// == MY ATTEMPT
// NOTE Similar to solution, but I did the printing from within
// the calculate() methods instead of within a single function
// Notes:
// * Use a trait to declare a perimeter calculation function
// 1. Create a trait to calculate the perimeter of various shapes
trait Perimeter {
    fn calculate(&self);
}

// 2. Create some objects/data/shapes to work with
struct Square {
    side_length: i32,
}
impl Perimeter for Square {
    fn calculate(&self) {
        let perimeter = self.side_length * 4;
        println!("Square perimeter = {:?}", perimeter);
    }
}

struct Triangle {
    a_length: i32,
    b_length: i32,
    c_length: i32,
}
impl Perimeter for Triangle {
    fn calculate(&self) {
        let perimeter = self.a_length + self.b_length + self.c_length;
        println!("Triangle perimeter = {:?}", perimeter);
    }
}

// 3. Create a function that utilizes the Perimeter trait
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter
fn calculate_perimeter(shape: impl Perimeter) {
    // NOTE Just have to simply run the internal trait function
    shape.calculate();
}

fn main() {
    calculate_perimeter(Square { side_length: 4});
    calculate_perimeter(Triangle {
        a_length: 2,
        b_length: 4,
        c_length: 6,
    });
}


// == SOLUTION
trait Perimeter {
    fn calculate(&self) -> i32;
}

// 2. Create some objects/data/shapes to work with
struct Square {
    side_length: i32,
}
impl Perimeter for Square {
    fn calculate(&self) -> i32 {
        self.side_length * 4
    }
}

struct Triangle {
    a_length: i32,
    b_length: i32,
    c_length: i32,
}
impl Perimeter for Triangle {
    fn calculate(&self) -> i32 {
        self.a_length + self.b_length + self.c_length
    }
}

// 3. Create a function that utilizes the Perimeter trait
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter
fn print_perimeter(shape: impl Perimeter) {
    // NOTE Just have to simply run the internal trait function
    let perimeter = shape.calculate();
    println!("perimeter = {:?}", perimeter);
}

fn main() {
    print_perimeter(Square { side_length: 4});
    print_perimeter(Triangle {
        a_length: 2,
        b_length: 4,
        c_length: 6,
    });
}
