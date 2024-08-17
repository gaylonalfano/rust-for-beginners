use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // NOTE: new() vs. build() -- new() is not supposed to possibly error.

    // -- With Iterators and Iterator Trait Methods instead of indexing (i.e. &args[1])
    // NOTE: Because this takes ownership of args Iterator and we'll be mutating
    // args by iterating over it, we add 'mut'
    pub fn build_with_iter(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next(); // For the first stdout line

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    // -- Without Iterators
    pub fn build_with_array(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments provided");
        }

        let query = &args[1];
        let file_path = &args[2];
        // NOTE: !! is_ok() checks whther the var is SET, NOT the
        // actual VALUE (otherwise, we'd use unwrap() to check VALUE)
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // Check ignore_case
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// NOTE: !! The lifetime 'a tells Rust that the data returned by search()
// will live as long as the data passed as the 'contents' argument.
// REF: https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // The function return type tells collect() enough info
    contents.lines().filter(|l| l.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|l| l.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        // WARN: The contents string MUST left-align with 'mod',
        // otherwise you have to account for added tabs/spaces, etc.
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        // WARN: The contents string MUST left-align with 'mod',
        // otherwise you have to account for added tabs/spaces, etc.
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
