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

// === NOTES
// - String.split('\n').enumerate() is handy to get an index + value


use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use std::collections::HashMap;
use thiserror::Error;
use structopt::StructOpt;


// NOTE To use StructOpt CLI we're going to need one struct + one enum:
// struct to handle standard options
// enum to handle each command given to the program
/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "contacts", about = "project 2: Contact Manager")]
struct Opt {
    /// Set file_path
    #[structopt(short, parse(from_os_str), default_value = "src/bin/p2_data.csv")]
    file_path: PathBuf,

    #[structopt(subcommand)]
    cmd: Command,
}

// NOTE subcommands have to be structures (e.g., List {})
#[derive(StructOpt, Debug)]
enum Command {
    List {

    },
}


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
        self.inner.insert(record.id.to_string(), record); // { 1: Record {} }
    }

    fn get_contact_by_id(self, id: &str) {
        match self.inner.get(&id.to_owned()) {
            Some(record) => println!("record={:?}",record),
            None => println!("Contact not found"),
        }
    }
}

#[derive(Debug)]
struct Record {
    id: i64,
    // id: String,
    name: String,
    email: Option<String>,
}

// TODO Create a custom ParseError enum for parsing the CSV
#[derive(Debug, Error)]
enum ParseError {
    #[error("invalid id format")]
    // NOTE This must impl the From trait from ParseIntError
    InvalidIdFormat(#[from] std::num::ParseIntError),

    #[error("empty record")]
    EmptyRecord,

    #[error("missing required field")]
    MissingRequiredField, 
}


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
    // println!("contents = {:?}", contents);
    // 3. Everything good to this point, return our Ok variant RecordsMap
    Ok(parse_records(contents))
}

fn parse_records(records: String) -> RecordsMap {
    let mut records_map = RecordsMap::new();

    // NOTE Use for loop w/ String.split().enumerate() for less temp vars
    for (i, record) in records.split('\n').enumerate() {
        // NOTE Only check the TRUE condition to save code
        if record != "" {
            match parse_record(record) {
                Ok(r) => {
                    records_map.add(r);
                }
                Err(e) => { println!("error on line {}: {}\n  >> \"{}\"\n", i + 1, e, record); }
            }
        }
    }

    // // === ORIGINAL parse record attempt
    // // TODO Use the 'split' function to split by newline
    // let lines: Vec<&str> = records.split('\n').collect();
    // // println!("{:?}", lines);  // ["5,Gaylon A.,", "", "7,Ash,a@email.com"]

    // // Loop through vector and convert each record to Record type
    // for line in lines {
    //     // === ORIGINAL ATTEMPT:
    //     // Split the line by ','
    //     let fields: Vec<&str> = line.split(',').collect();
    //     // println!("fields= {:?}", fields); // fields= ["500", "Gonzalo Buxey", "gbuxeydv@dedecms.com"]
    //     // NOTE Solution use 'if record != ""' instead
    //     if fields.len() < 2 || fields.len() > 3 {
    //         // TODO Return a custom error?
    //         // Q: How to add/use these custom errors?
    //         // A: Solution created separate parse_record(record: &str) -> Result<Record,
    //         // ParseError> function to handle
    //         continue
    //     } else {
    //         // FIXME Could make this validation more robust and custom errors
    //         // NOTE Solution created separate parse_record(record: &str) -> Result<Record,
    //         // ParseError> function to handle
    //         // NOTE Vec types have the get() and get_mut() methods
    //         // E.g., make sure id is_numeric, id and name are there, etc.
    //         // Capture each field value
    //         let id = fields.get(0).unwrap_or_else(|| &"id_default");
    //         let name = fields.get(1).unwrap_or_else(|| &"name_default");
    //         let email = fields.get(2).unwrap_or_else(|| &"email_default");

    //         // Convert to Record type
    //         let new_record: Record = Record {
    //             id: id.to_string(),
    //             name: name.to_string(),
    //             email: Some(email.to_string()),
    //         };

    //         // Add this new record to our records_map
    //         records_map.add(new_record);
    //     }
    // }

    // Return complete RecordsMap
    println!("Final parsed RecordsMap = {:?}", records_map);
    records_map
}

fn parse_record(record: &str) -> Result<Record, ParseError> {
    // E.g.: "5,Gaylon A.,", "", "7,Ash,a@email.com"
    let fields: Vec<&str> = record.split(',').collect();

    // NOTE Use the Vec.get() method but along w/ match since get() -> Option
    // My original attempt used .get().unwrap_or_else(|| &"default_id")
    // IMPORTANT Assigning variables using multiple expressions (like below) is
    // a good practice. We're giving it a couple places to fail before finally
    // assigning our variable a value. This makes it more robust. When you
    // use expressions, this allows the compiler to check all possible use cases.
    let id = match fields.get(0) {
        // Q: How to convert a &str to i64?
        // A: Solution used i64::from_str_radix(id, 10)?
        // A: Quite a few options: https://stackoverflow.com/questions/27043268/convert-a-string-to-int?rq=1
        // Some(id) => id.to_string(),
        // NOTE I need to ensure that my custom error impl the 'From' trait
        // from the ParseIntError i.e.: #[from] std::num::ParseIntError
        // NOTE These are TWO different error types!
        Some(id) => i64::from_str_radix(id, 10)?,
        // NOTE It's here that we can return early our custom error.
        None => return Err(ParseError::EmptyRecord),
    };

    let name = match fields.get(1) {
        // Q: What's the difference between to_owned() vs to_string()?
        // Only when I use to_string() does it not error...
        Some(name) => name.to_string(), // Error if I use to_owned()
        // NOTE It's here that we can return early our custom error
        None => return Err(ParseError::MissingRequiredField),
    };

    // NOTE Solution used fields.get(2).map(|e| e.to_string()).filter(|e| e != "");
    // The difference between approaches is that the solution will return None
    // instead of an empty string for the 'email' field.
    // I still need to handle getting email into a String type for Record struct
    let email = fields.get(2).unwrap_or_else(|| &"").to_string();
    // let email = fields.get(2).map(|e| e.to_string()).filter(|e| e != "");

    // Return our completed Record
    let new_record = Record {
        id,
        name,
        email: Some(email)
    };

    Ok(new_record)
}

fn run(opt: Opt) -> Result<(), std::io::Error> {
    match opt.cmd {
        // NOTE Have to use struct pattern syntax ie. Command::List {}
        // Can't simply use struct variant ie. Command::List =>
        // NOTE To ignore everything inside the List we use 'List { .. }' syntax
        Command::List {..} => {
            let records = load_records(opt.file_path)?;
            println!("records={:?}", records);
        }
    }
    Ok(())
}

fn main() {


    // // === BEFORE adding StructOpt CLI
    // // Let's try to read our CSV file
    // let file_path = PathBuf::from(r"src/bin/p2_data.csv");
    // // Q: Where should I init the RecordsMap? Inside main?
    // // Challenge is I want to add, search contacts within the HM so
    // // need one to work with...
    // // let records = match load_records(file_path) {
    // //     Ok(records_map) => records_map,
    // //     Err(e) => println!("Unable to load file: {:?}", e),
    // // }; // ERROR Incompatible match arm types
    // // Q: Do I need to use match or can I just use expect()? I want to store
    // // a copy of the RecordsMap so I can add, find contacts, etc.
    // // A: This works but is there a better way?
    // let records = load_records(file_path).expect("Unable to load records"); // WORKS. Better way?

    // let contact_id = "197";
    // records.get_contact_by_id(contact_id);


    // === AFTER adding StructOpt CLI
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        println!("An error occurred: {:?}", e);
    }

}
