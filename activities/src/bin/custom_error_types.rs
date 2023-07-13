// ==== Manual Error Creation
// - Implement Debug trait (can use derive() macro)
// - Implement Error trait using std::error::Error;
// -- Writing impl details is optional since the trait
//    has Default implementation for all the details.
// - Implement Display trait for user

// #[derive(Debug)]
// enum LockError {
//     MechanicalError(i32),
//     NetworkError,
//     NotAuthorized,
// }
//
// use std::error::Error;
// impl Error for LockError {}
//
// use std::fmt;
// impl fmt::Display for LockError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Self::MechanicalError(code) => write!(f, "Mechanical Error: {}", code),
//             Self::NetworkError => write!(f, "Network Error"),
//             Self::NotAuthorized => write!(f, "Not Authorized"),
//         }
//     }
// }

// // ==== Using 'thiserror' Crate
// use thiserror::Error;
//
// #[derive(Debug, Error)]
// enum LockError {
//     #[error("Mechanical Error: {0}")]
//     MechanicalError(i32),
//     #[error("Network Error")]
//     NetworkError,
//     #[error("Not Authorized Error")]
//     NotAuthorized,
// }
//
// fn lock_door() -> Result<(), LockError> {
//     // ... some code...
//     Err(LockError::NetworkError)
// }

// ==== Error Conversion
// use thiserror::Error;
//
// #[derive(Debug, Error)]
// enum NetworkError {
//     #[error("Connection timed out")]
//     TimeOut,
//     #[error("Unreachable")]
//     Unreachable,
// }
//
// // NOTE: If we get a NetworkError, it will auto-convert
// // into a LockError thanks to #[from] annotation.
// // The LockError will be a Network variant. This is type conversion.
// enum LockError {
//     #[error("Mechanical Error: {0}")]
//     MechanicalError(i32, i32),
//     #[error("Network Error")]
//     Network(#[from] NetworkError),
//     #[error("Not Authorized")]
//     NotAuthorized,
// }

// ===== Demo Custom Errors
use chrono::{DateTime, Duration, Utc};
use thiserror::Error;

struct SubwayPass {
    id: usize,
    funds: isize,
    expires: DateTime<Utc>,
}

#[derive(Debug, Error)]
enum PassError {
    #[error("Expired pass")]
    PassExpired,
    #[error("Insufficient funds: {0}")]
    InsufficientFunds(isize),
    #[error("Pass read error: {0}")]
    ReadError(String),
}

fn swipe_card() -> Result<SubwayPass, PassError> {
    // Err(PassError::PassExpired)
    // Err(PassError::ReadError("Magstrip failed to read".to_owned()))

    Ok(SubwayPass {
        id: 0,
        funds: 200,
        expires: Utc::now() - Duration::weeks(52),
    })
}

fn use_pass(pass: &mut SubwayPass, cost: isize) -> Result<(), PassError> {
    if Utc::now() > pass.expires {
        Err(PassError::PassExpired)
    } else {
        if pass.funds - cost < 0 {
            Err(PassError::InsufficientFunds(pass.funds))
        } else {
            pass.funds = pass.funds - cost;
            Ok(())
        }
    }
}

fn main() {
    let pass_status = swipe_card().and_then(|mut pass| use_pass(&mut pass, 3));

    match pass_status {
        Ok(_) => println!("Okay to board"),
        Err(e) => match e {
            // NOTE: Could do more with these arms and inner values
            PassError::ReadError(s) => (),
            PassError::PassExpired => (),
            PassError::InsufficientFunds(a) => (),
        },
    }
}
