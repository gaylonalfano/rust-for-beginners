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
use thiserror::Error;

// NOTE Creating a custom struct that WRAPS a HashMap. We want to store data in HashMap,
// but we don't want to pass around the HashMap to all our functions. Better approach is
// to create this custom struct so whenever we change the structure of our data in the
// HashMap, we just change inside this custom RecordsMap wrapper struct.
#[derive(Debug)]
struct RecordsMap {
    // inner: HashMap<i64, Record>,
    inner: HashMap<String, Record>,
}

impl RecordsMap {
    fn new() -> Self {
        Self { inner: HashMap::new() }
    }

    // Create a function that can add a new Record to our HashMap
    // NOTE We'll use this once we load/parse the CSV
    fn add(&mut self, record: Record) {
        // NOTE HashMaps require a String for the key so need to use clone()
        // I'll later change the key to be i64
        self.inner.insert(record.id.clone(), record); // { 1: Record {} }
    }
}

#[derive(Debug)]
struct Record {
    // id: i64,
    id: String,
    name: String,
    email: Option<String>,
}

// TODO Create a custom ParseError enum for parsing the CSV
#[derive(Debug, Error)]
enum ParseError {

    #[error("invalid id format")]
    InvalidIdFormat,

    #[error("empty record")]
    EmptyRecord,

    #[error("missing required field")]
    MissingRequiredField, 
}


// WORKING EXAMPLE
// fn load_records() -> std::io::Result<()> {
//     let path = PathBuf::from("src/bin/foo.txt");
//     let mut file = File::open(path)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     println!("contents = {:?}", contents);
//     assert_eq!(contents, "Hello, world!\n");
//     Ok(())
// }

fn load_records(file_path: PathBuf) -> std::io::Result<RecordsMap> {
    // Q: How do you slowly debug/iterate when dealing with returning
    // certain types with functions? E.g., I just want to test out
    // how to read the file and parse line-by-line. In order to do that,
    // I have to make it return a simple/empty placeholder (RecordsMap::new())
    // just to get the program to compile.
    // 1. Try to open the file
    let mut file = File::open(file_path)?;
    // 2. Create an empty buffer to store the content
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("contents = {:?}", contents);
    // 3. Everything good to this point, return our Ok variant RecordsMap
    Ok(parse_records(contents))
}

// TODO Need a function that actually goes through the CSV records
fn parse_records(records: String) -> RecordsMap {
    // Create a new RecordsMap to work with
    let mut records_map = RecordsMap::new();

    // TODO Use the 'split' function to split by newline
    let lines: Vec<&str> = records.split('\n').collect();
    // println!("{:?}", lines);  // ["5,Gaylon A.,", "", "7,Ash,a@email.com"]

    // Loop through vector and convert each record to Record type
    for line in lines {
        // Split the line by ','
        let v: Vec<&str> = line.split(',').collect();
        // println!("v= {:?}", v); // v= ["500", "Gonzalo Buxey", "gbuxeydv@dedecms.com"]
        if v.len() < 2 || v.len() > 3 {
            // TODO Return a custom error?
            continue
        } else {
            // FIXME Could make this validation more robust and custom errors
            // Validate each field in v
            // NOTE Vec types have the get() and get_mut() methods
            // E.g., make sure id is_numeric, id and name are there, etc.
            // Capture each field value
            let id = v.get(0).unwrap_or_else(|| &"id_default");
            let name = v.get(1).unwrap_or_else(|| &"name_default");
            let email = v.get(2).unwrap_or_else(|| &"email_default");

            // Convert to Record type
            let new_record: Record = Record {
                id: id.to_string(),
                name: name.to_string(),
                email: Some(email.to_string()),
            };

            // Add this new record to our records_map
            records_map.add(new_record);
        }
    }

    // Return complete RecordsMap
    println!("Final parsed RecordsMap = {:?}", records_map);
    records_map
}

fn main() {

    // let record_one = Record { id: 1, name: "one".to_string(), email: None };
    // let record_two = Record { id: 2, name: "two".to_string(), email: Some("two@email.com".to_owned()) };
    // let record_three = Record { id: 3, name: "three".to_string(), email: Some("three@email.com".to_owned()) };

    // Add our records into our new HashMap
    // records.add(record_one);
    // records.add(record_two);
    // records.add(record_three);
    // println!("{:?}", records);

    // for (id, record) in records.inner.iter() {
    //     println!("id = {:?}, name = {:?}, email = {:?}", id, record.name, record.email);
    // }

    // Let's try to read our CSV file
    let file_path = PathBuf::from(r"src/bin/p2_data.csv");
    load_records(file_path).expect("Unable to load records");


    // == WORKS
    // let mut file = File::open("./src/bin/foo.txt")?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // assert_eq!(contents, "Hello, world!\n");
    // Ok(())

    // == Splitting a String by commas
    // let x = "id,name,email".to_string();
    // let v: Vec<&str> = x.split(',').collect();
    // println!("{:?}", v);
}
