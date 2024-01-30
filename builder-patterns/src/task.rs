// REF:
#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub done: bool,
    pub desc: Option<String>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            done: false,
            desc: None,
        }
    }
}

impl Task {
    // NOTE: 'impl Into<String>' allows for &String, &str and String to be accepted
    // NOTE: If you need multiple constructors, one pattern is: with_title(...)
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            done: false,
            desc: None,
        }
    }
}
