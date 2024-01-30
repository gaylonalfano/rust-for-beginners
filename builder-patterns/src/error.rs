pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev
                                             //
                                             // NOTE: With this generic Error type, you have some basics like:
                                             // return Err("No URL".to_string().into());
                                             // return Err(format!("No URL {url:#?}").into());
