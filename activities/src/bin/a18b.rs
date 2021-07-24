//// https://www.udemy.com/course/rust-coding-for-beginners/learn/lecture/23530616#questions
//// Topic: Result & the question mark operator
////
//// === SOLUTION
//// Requirements:
//// * Determine if an employee can access a building using a digital keycard
//// * Employees that can access the building are:
////   * Maintenance crews
////   * Marketing department employees
////   * Managers
//// * Other employees that work at the company are:
////   * Line supervisors
////   * Kitchen staff
////   * Assembly technicians
//// * Ensure that terminated employees cannot access the building
////   regardless of their position
////
//// Notes:
//// * Use an enum to represent all types of employees
//// * Use a struct to store the employee type and whether they are
////   still employed
//// * Use a function that returns a Result to determine if the employee
////   may enter the building
//// * Print whether the employee may access the building
////   * Must use a function that utilizes the question mark operator to do this

//enum Position {
//    Maintenance,
//    Marketing,
//    Manager,
//    LineSupervisor,
//    KitchenStaff,
//    AssemblyTechnician,
//}

//enum Status {
//    Active,
//    Terminated,
//}

//struct Employee {
//    position: Position,
//    status: Status,
//}

// fn try_access(employee: &Employee) -> Result<(), String> {
//     match employee.status {
//         // NOTE Early 'return' if terminated so the function stops executing
//         // and won't bother running the next match employee.position block.
//         Status::Terminated => return Err("Employee is terminated".to_owned()), // Works
//         // Q: What if you don't early 'return'? Which Err will get returned?
//         // A: Error! match arms have incompatible types.
//         // Status::Terminated => Err("Employee is terminated".to_owned()), // Error
//         // Q: Return Ok(()) or just ()?
//         // A: Seems like () which is essentially nothing
//         // _ => Ok(()), // Error: Expected () found Result
//         _ => (), // Works
//     }

//    match employee.position {
//        Position::Maintenance => Ok(()),
//        Position::Manager => Ok(()),
//        Position::Marketing => Ok(()),
//        _ => Err("Position is not authorized".to_owned()),
//    }
//}

//fn print_access(employee: &Employee) -> Result<(), String> {
//    // NOTE If try_access fails, then the early return of Err will return the
//    // error String, which is part of the match print_access(&manager) line in main.
//    // So the print result would be: "Access denied: Employee is terminated"
//    // Q: Do I have to set this to a new variable or can I run try_access(employee)? directly?
//    // A: Both work!
//    // let access_attempt = try_access(employee)?; // Works
//    try_access(employee)?; // Works

//    // NOTE This is the strength of the ? operator as it allows you to check
//    // multiple things/conditions if needed. If any of these fail (returns Err)
//    // then the Err inner String will be returned from this function.
//    // let another_attempt = try_department(employee)?;
//    // let one_more_attempt = try_promotion(employee)?;
//    println!("Employee has access");
//    // Finally here we return Ok(()), not just inner () unit type
//    Ok(())
//}

//fn main() {
//    let spike = Employee {
//        position: Position::Manager,
//        status: Status::Active,
//    };
//    let bull = Employee {
//        position: Position::LineSupervisor,
//        status: Status::Active,
//    };
//    let colt = Employee {
//        position: Position::Marketing,
//        status: Status::Terminated,
//    };
//    let employees = vec![spike, bull, colt];

//    for e in employees {
//        // NOTE Use a match on the print_access Result type!
//        match print_access(&e) {
//            Err(error) => println!("Access denied: {:?}", error),
//            _ => (),
//        }
//    }
//}

// === MY ATTEMPT
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position

#[derive(Debug)]
enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

#[derive(Debug)]
enum Status {
    Active,
    Terminated,
}

#[derive(Debug)]
struct Employee {
    name: String,
    status: Status,
    position: Position,
    age: i32,
}

impl Employee {
    fn new(name: &str, status: Status, position: Position, age: i32) -> Self {
        Self {
            name: name.to_owned(),
            status,
            position,
            age,
        }
    }

    // Q: How to create a method that terminates employee
    // by changing 'active' property to false?
    // A: Make it mutable! &mut self
    fn deactivate(&mut self) {
        self.status = Status::Terminated;
    }

    fn activate(&mut self) {
        self.status = Status::Active;
    }
}

fn try_status(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Active => Ok(()),
        Status::Terminated => Err(String::from("Status is terminated")),
    }
}

fn try_position(employee: &Employee) -> Result<(), String> {
    match employee.position {
        Position::Maintenance => Ok(()),
        Position::Manager => Ok(()),
        Position::Marketing => Ok(()),
        _ => Err("Position is not authorized".to_owned()),
    }
}

fn try_age(employee: &Employee) -> Result<(), String> {
    // Q: How to match on integer values/ranges?
    // A: Use if/else instead or use match guards!
    // https://stackoverflow.com/questions/47852269/can-i-use-and-in-match
    // match employee.age {
    //     a if a < 18 => Err("Age is too young".to_owned()),
    //     _ => Ok(()),
    // }
    if employee.age < 18 {
        Err("Age is too young".to_owned())
    } else {
        Ok(())
    }
}

fn try_access(employee: &Employee) -> Result<(), String> {
    // Check whether the employee's position is authorized or
    // return the Err() variant
    // == WITH helpers + ? operator:
    try_status(employee)?;
    try_position(employee)?;
    try_age(employee)?;
    // NOTE Must finally return Ok(()), not just inner () unit type
    Ok(())

    // == WITHOUT helpers:
    // // NOTE Only return Ok(()) at the end! Otherwise just return ()
    // // NOTE I find using helpers with ? (see above) cleaner
    // match employee.position {
    //     Position::Maintenance => (),
    //     Position::Manager => (),
    //     Position::Marketing => (),
    //     _ => return Err("Position is not authorized".to_owned()),
    // }

    // if employee.age < 18 {
    //     return Err("Age is too young".to_owned());
    // } else {
    //     ()
    // }

    // // NOTE This should only run if above checks pass
    // // NOTE Need to return Ok and Err variants since end of scope
    // match employee.status {
    //     Status::Active => Ok(()),
    //     Status::Terminated => Err(String::from("Status is terminated")),
    // }
}

fn main() {
    let carl = Employee {
        name: String::from("carl"),
        status: Status::Active,
        position: Position::Manager,
        age: 20,
    };
    let max = Employee {
        name: "max".to_owned(),
        status: Status::Terminated,
        position: Position::Marketing,
        age: 25,
    };
    let sandy = Employee::new("sandy", Status::Active, Position::KitchenStaff, 16);
    let piper = Employee::new("piper", Status::Terminated, Position::LineSupervisor, 18);
    let stu = Employee::new("stu", Status::Active, Position::Maintenance, 17);

    let employees = vec![carl, max, sandy, piper, stu];

    for e in employees {
        let access = try_access(&e);
        println!("{:?}", access);
    }
}
