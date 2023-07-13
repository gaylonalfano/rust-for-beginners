// Custom Errors
// NOTES
// - Need to derive(Error, Debug) in order to print out errors
// and get more functionality working with errors
// - The #[from] macro converts one type of error into another
// E.g., DatabaseError(#[from] SqlError) converts SqlError to DBE



// // === DEMO
// use thiserror::Error;

// #[derive(Error, Debug)]
// enum LoginError {
//     #[error("database error")]
//     DatabaseError(#[from] SqlError),

//     #[error("password expired")]
//     PasswordExpired,

//     #[error("user not found")]
//     UserNotFound,

//     #[error("network connection error")]
//     NetworkError(#[from] std::io::Error),

//     #[error("wrong password")]
//     WrongPassword,
// }

// fn login(user: &str, password: &str) -> Result<String, LoginError> {
//     // NOTE The Result String represents a session id for a user
//     // NOTE If the connect() fails, it will convert the std::io::Error
//     // into a LoginError, which will be the variant NetworkError. This
//     // means we can 'match' on the LoginError to see what happened.
//     let connection: Result<Connection, std::io::Error> = connect()?;
//     let user_id = get_user_id(user)?; // Error would be UserNotFound
//     if try_password(user_id, password)? {
//         // NOTE If try_password() fails then it returns PasswordExpired
//         // NOTE get_session() returns session ID or fails
//         // NOTE SqlError gets converted into a DatabaseError
//         let session: Result<String, SqlError> = get_session(user_id)?;
//         Ok(session)
//     } else {
//         Err(LoginError::WrongPassword)
//     }
// }

// fn main() {
//     login("kate", "123");
// }

// Topic: Custom error types
//
// Requirements:
// * Modify the `ProgramError` enum in order to make the program compile
//   and run. Do not modify any other part of the program.
// * The output should display a menu error followed by a math error when running.
//
// Notes:
// * Use `#[error("description")]` on the enum variants
// * Use `#[from] ErrorType` to convert the existing errors into a `ProgramError`

use thiserror::Error;

#[derive(Debug, Error)]
enum ProgramError {
    // * Modify the `ProgramError` enum in order to make the program compile
    //   and run. Do not modify any other part of the program.
    #[error("menu error occurred")]
    Menu(#[from] MenuError),

    #[error("math error occurred")]
    Math(#[from] MathError),
}

#[derive(Debug, Error)]
enum MenuError {
    #[error("menu item not found")]
    NotFound,
}

#[derive(Debug, Error)]
enum MathError {
    #[error("divide by zero error")]
    DivideByZero,
}

fn pick_menu(choice: &str) -> Result<i32, MenuError> {
    match choice {
        "1" => Ok(1),
        "2" => Ok(2),
        "3" => Ok(3),
        _ => Err(MenuError::NotFound),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b != 0 {
        Ok(a / b)
    } else {
        Err(MathError::DivideByZero)
    }
}

fn run(step: i32) -> Result<(), ProgramError> {
    if step == 1 {
        pick_menu("4")?;
    } else if step == 2 {
        divide(1, 0)?;
    }
    Ok(())
}

fn main() {
    println!("{:?}", run(1));
    println!("{:?}", run(2));
}
