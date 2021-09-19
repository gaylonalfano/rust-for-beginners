// Project 2: Contact manager
//
// User stories:
// * L1: I want to view my saved contacts.
// * L2: I want to add new contacts.
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * Make a backup of the existing `p2_data.csv` file.
// * CSV files contain records and fields:
//   Each line is a "record" which contain "fields" of information.
//   The fields are separated by commas. If a field is not provided,
//   then there is no data for that particular field. There will
//   always be a comma for the field even if no data is present.
// * The `id` and `name` fields are required, the `email` field is optional.
// * Check the documentation on the `std::fs::File` struct for reading
//   and writing files.
// * Use the `split` function from the standard library to extract
//   specific fields.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Create your program starting at level 1. Once finished, advance
//   to the next level.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.


use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use std::collections::HashMap;

// NOTE Creating a custom struct that WRAPS a HashMap. We want to store data in HashMap,
// but we don't want to pass around the HashMap to all our functions. Better approach is
// to create this custom struct so whenever we change the structure of our data in the
// HashMap, we just change inside this custom RecordsMap wrapper struct.
#[derive(Debug)]
struct RecordsMap {
    inner: HashMap<i64, Record>,
}

impl RecordsMap {
    fn new() -> Self {
        Self { inner: HashMap::new() }
    }

    fn add(&mut self, record: Record) {
        self.inner.insert(record.id, record); // { 1: Record {} }
    }
}

#[derive(Debug)]
struct Record {
    id: i64,
    name: String,
    email: Option<String>,
}

fn load_records(path: PathBuf) -> std::io::Result<RecordsMap> {
    let path = PathBuf::from(r"~/Code/rust-for-beginners/activities/src/bin/foo.txt");
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    assert_eq!(contents, "Hello, world!");
    Ok(RecordsMap::new())
}

fn main() {
    let mut records = RecordsMap::new();

    let record_one = Record { id: 1, name: "one".to_string(), email: None };
    let record_two = Record { id: 2, name: "two".to_string(), email: Some("two@email.com".to_owned()) };
    let record_three = Record { id: 3, name: "three".to_string(), email: Some("three@email.com".to_owned()) };

    // Add our records into our new HashMap
    records.add(record_one);
    records.add(record_two);
    records.add(record_three);
    println!("{:?}", records);

    for (id, record) in records.inner.iter() {
        println!("id = {:?}, name = {:?}, email = {:?}", id, record.name, record.email);
    }

    // println!("{:?}", record_one);
    // println!("{:?}", record_two);
    // println!("{:?}", record_three);

    // Let's try to read our CSV file
}
