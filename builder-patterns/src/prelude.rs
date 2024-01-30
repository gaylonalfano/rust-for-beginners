//! Crate prelude
// NOTE: Early dev we start out this way:
pub use crate::error::{Error, Result};

// Generic Wrapper tuple struct for NewType pattern
pub struct W<T>(pub T);
