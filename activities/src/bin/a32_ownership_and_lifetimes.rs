// Topic: Lifetimes & Structures
//
// NOTES:
// * Lifetimes are way to tell the compiler how to handle BORROWED data.
// * Lifetimes enable you to store borrowed data in structures, and
//   return borrowed data from functions.
// * Lifetime annotations (eg. <'a>) indicate that there exists
//   some owned data that:
//   -- Lives at least as long as the borrowed data
//   -- Outlives or outlasts the scope of a borrow
//   -- Exists longer than the scope of a borrow
//
// // === Demo ===
// #[derive(Debug)]
// struct Cards {
//     inner: Vec<IdCard>,
// }
//
// #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
// enum City {
//     Barland,
//     Bazopolis,
//     Fooville,
// }
//
// #[derive(Debug)]
// struct IdCard {
//     name: String,
//     age: u8,
//     city: City,
// }
// impl IdCard {
//     pub fn new(name: &str, age: u8, city: City) -> Self {
//         Self {
//             name: name.to_string(),
//             age,
//             city,
//         }
//     }
// }
//
// fn new_ids() -> Cards {
//     Cards {
//         inner: vec![
//             IdCard::new("Amy", 1, City::Fooville),
//             IdCard::new("Matt", 10, City::Barland),
//             IdCard::new("Bailee", 20, City::Barland),
//             IdCard::new("Anthony", 30, City::Bazopolis),
//             IdCard::new("Tina", 40, City::Bazopolis),
//         ],
//     }
// }
//
// // We want to create a structure that borrows the
// // existing IdCards, but we want to filter based on
// // person's age.
// #[derive(Debug)]
// struct YoungPeople<'a> {
//     inner: Vec<&'a IdCard>,
// }
// // Implement functionality to filter by City
// // NOTE: When impl on a structure that has lifetimes, we need
// // to annotate after 'impl<'a>' and the struct<'a>.
// impl<'a> YoungPeople<'a> {
//     fn living_in_fooville(&self) -> Self {
//         Self {
//             inner: self
//                 .inner
//                 .iter()
//                 .filter(|id| id.city == City::Fooville)
//                 // NOTE: Need to use .map(|id| *id) to deref, otherwise we have
//                 // two borrows &&, when we just want one & (Vec<&'a IdCard>)
//                 .map(|id| *id)
//                 .collect(),
//         }
//     }
// }
//
// fn main() {
//     let ids = new_ids();
//
//     // NOTE: iter() automatically borrows the items, so .collect()
//     // will only have borrowed items. This borrow is allowed because
//     // 'ids' were created prior to our 'young' structure.
//     let young = YoungPeople {
//         inner: ids.inner.iter().filter(|&id| id.age < 18).collect(),
//     };
//
//     println!("\nids");
//     for id in ids.inner.iter() {
//         println!("{:?}", id);
//     }
//
//     println!("\nyoung");
//     for id in young.inner.iter() {
//         println!("{:?}", id);
//     }
//
//     println!("\nliving in fooville");
//     for id in young.living_in_fooville().inner.iter() {
//         println!("{:?}", id);
//     }
// }

// === Activity ===
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

use std::borrow::Borrow;

const MOCK_DATA: &'static str = include_str!("mock-data.csv");

struct Employee<'a> {
    name: &'a str,
    title: &'a str,
}
impl<'a> Employee<'a> {
    fn print_info(&self) {
        println!("name: {} | title: {}", self.name, self.title);
    }
}

// Jayson did two structs with str types
struct Names<'a> {
    inner: Vec<&'a str>,
}

struct Titles<'a> {
    inner: Vec<&'a str>,
}

fn main() {
    let data: Vec<_> = MOCK_DATA.split('\n').skip(1).collect();
    let names: Vec<_> = data
        .iter()
        .filter_map(|line| line.split(',').nth(1))
        .collect();
    // Using Jayson's two struct approach
    // We 'shadow' our variables and store into structs
    let names = Names { inner: names };

    // for n in names.iter().take(3) {
    //     println!("{}", n);
    // }

    let titles: Vec<_> = data
        .iter()
        .filter_map(|line| line.split(',').nth(4))
        .collect();
    let titles = Titles { inner: titles };

    let data_zipped = names.inner.iter().zip(titles.inner.iter());

    let mut employees: Vec<Employee> = vec![];
    for (name, title) in data_zipped.take(5) {
        employees.push(Employee {
            name: *name,
            title: *title,
        });
        println!("Name: {}, Title: {}", name, title);
    }

    // Trying my version - WORKS
    for e in employees.iter().take(5) {
        e.print_info()
    }
}
