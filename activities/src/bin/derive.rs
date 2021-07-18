// https://www.udemy.com/course/rust-coding-for-beginners/learn/lecture/23530280#overview
// NOTES:
// * All fields within the struct or enum also have to derive
// the same functionality (e.g., you can't use Position enum type 
// inside Employee struct if Position doesn't derive as well).
// - Clone and Copy traits are related to storing data inside
// the STACK (Copy) vs. HEAP (Clone is a DEEP copy, so get get
// two separate copies inside the heap where ownership matters).
// - Copy (stack-only data) is only for simple scalar values known
// at compile time (u32, bool, f64, char, tuples (if items are scalar too))

// Example 1 - Enums can hold variants that also has associated data.
enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll(i32), // +/- for up/down
    Move(i32, i32), // x, y values
}

// Example 2 - Can have enums inside other enums
// NOTE The additional data is ALWAYS required when you.
enum PromoDiscount {
    NewUser,
    Holiday(String),
}

enum Discount {
    Percent(f64),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String),
}




#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}


fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };
    // Without deriving any functionality you have to match all cases
    // match me.position {
    //     Position::Manager => println!("manager"),
    //     Position::Supervisor => println!("supervisor"),
    //     Position::Worker => println!("worker"),
    // }
    // Deriving Debug for ONLY Position
    // NOTE This means we don't need the above match block at all
    // println!("{:?}", me.position);

    // Deriving Debug for BOTH Position and Employee
    // println!("{:?}", me);

    // NOTE After deriving Clone and Copy traits, the 'me' variable
    // will NOT be moved to print_employee() function!
    // Q: Is it Copy or Clone that's handling this behind the scenes?
    // Instructor said 'copy' but Copy is Stack-only data for simple
    // scalar values. Is Employee struct considered scalar since inside
    // it has an enum and i64?
    // If it is indeed a Copy, then from my understanding, a fresh copy
    // of 'me' is made and placed inside print_employee() function since
    // making copies to the stack is fast/cheap.
    // A: I *believe* it's a bit of both. If I remove Clone from derive,
    // then I get errors saying Clone needs to be implemented basically.
    // If I remove both, then it errors because 'me' is getting moved into
    // print_employee() and then dropped, so cannot be referenced again in
    // the second call.
    print_employee(me); // NOT moved because of Copy!
    print_employee(me); // Another copy is made and stored in STACK
}
