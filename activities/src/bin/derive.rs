// NOTES:
// * All fields within the struct or enum also have to derive
// the same functionality (e.g., you can't use Position enum type 
// inside Employee struct if Position doesn't derive as well).

#[derive(Debug)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug)]
struct Employee {
    position: Position,
    work_hours: i64,
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
    println!("{:?}", me.position);

    // Deriving Debug for BOTH Position and Employee
    // println!("{:?}", me);
}
