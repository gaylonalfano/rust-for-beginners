use std::io;

fn main() {
    // Q: How to type a simple string?
    // A: &str
    // Q: How to splice a string to retrieve a single char?
    // Q: How to make a tuple storing the string and float inputs?

    println!("Enter degrees:");

    let mut degrees = String::new();
    io::stdin()
        .read_line(&mut degrees)
        .expect("Failed to read DEGREES line!");
    // Shadow to convert to float
    let degrees: f64 = degrees.trim().parse().expect("Please type a number!");
    println!("degrees: {}", degrees);


    println!("Enter metric (c or f):");
    let mut metric = String::new(); // What about char? Or, enum?
    io::stdin()
        .read_line(&mut metric)
        .expect("Failed to read METRIC line!");
    let metric = metric.trim();
    println!("metric: {}", metric);

    let tup: (f64, &str) = (degrees, metric);

    // convert_temperature(&degrees, &metric)
    convert_temperature(&tup.0, &tup.1)

}

fn convert_temperature(d: &f64, m: &str) {
    let mut result: f64 = 0.0;

    match m {
        // "c" => (d * (9.0/5.0)) + 32.0,
        // "f" => (d - 32.0) * (5.0/9.0),
        // _ => println!("Please enter either 'c' or 'f'.")
        // Q: How to perform PEMDAS arithmetic?
        "c" => result = d * (9.0/5.0) + 32.0,
        "f" => result = (d - 32.0) * (5.0/9.0),
        _ => println!("Please enter either 'c' or 'f'.")
    }

    println!("result is: {}", result);
}


// // === Alternative 1:
// use std::io;

// fn main() {
//     println!("Enter a temperature.");

//     let mut temperature = String::new();
//     io::stdin()
//         .read_line(&mut temperature)
//         .expect("Failed to read temperature");
//     let temperature = temperature;

//     let mid = temperature.len().checked_sub(2).expect("Input string was empty");
//     if !temperature.is_char_boundary(mid) {
//         panic!("Input should end in a single-byte char (the unit)");
//     }
//     let (value, unit) = temperature.split_at(mid);

//     let value: f32 = value.trim().parse().unwrap_or_else(|_| {
//         panic!("{} is not a number", value)
//     });
//     let unit = unit.trim().to_lowercase();

//     println!("The input temperature is: {}", temperature);

//     match unit.as_str() {
//         "c" => println!("{}", c_to_f(value)),
//         "f" => println!("{}", f_to_c(value)),
//         _ => panic!("Unsupported unit"),
//     }
// }

// fn c_to_f(c: f32) -> String {
//     output(c * 1.8 + 32.0, "F")
// }

// fn f_to_c(f: f32) -> String {
//     output((f - 32.0) / 1.8, "C")
// }

// fn output(t: f32, u: &str) -> String {
//     format!("The output temperature is: {}{}", t, u)
// }


// // === Alternative 2:
// use std::io;

// fn main() {
//     println!("This is a program that converts temperatures between Fahrenheit and Celsius.");

//     loop {
//         println!("\nEnter one of the following commands:");
//         println!(" - 0: convert from Fahrenheit to Celsius");
//         println!(" - 1: convert from Celsius to Fahrenheit");
//         println!(" - 2: quit");

//         let mut command = String::new();
//         match io::stdin().read_line(&mut command) {
//             Ok(_) => {}
//             Err(_) => {
//                 println!("Failed to read command.");
//                 continue;
//             }
//         }

//         let command: u32 = match command.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("Invalid command.");
//                 continue;
//             }
//         };
//         match command {
//             0 => fahrenheit_to_celsius(),
//             1 => celsius_to_fahrenheit(),
//             2 => break,
//             _ => println!("Invalid command."),
//         }
//     }
// }

// fn fahrenheit_to_celsius() {
//     println!("Enter the temperature in Fahrenheit.");

//     let mut temperature = String::new();
//     match io::stdin().read_line(&mut temperature) {
//         Ok(_) => {}
//         Err(_) => {
//             println!("Failed to read temperature.");
//             return;
//         }
//     }

//     let temperature: f64 = match temperature.trim().parse() {
//         Ok(t) => t,
//         Err(_) => {
//             println!("Invalid temperature.");
//             return;
//         }
//     };
//     let converted = (temperature - 32.0) / 1.8;
//     println!("{} Fahrenheit = {} Celsius", temperature, converted);
// }

// fn celsius_to_fahrenheit() {
//     println!("Enter the temperature in Celsius.");

//     let mut temperature = String::new();
//     match io::stdin().read_line(&mut temperature) {
//         Ok(_) => {}
//         Err(_) => {
//             println!("Failed to read temperature.");
//             return;
//         }
//     }

//     let temperature: f64 = match temperature.trim().parse() {
//         Ok(t) => t,
//         Err(_) => {
//             println!("Invalid temperature.");
//             return;
//         }
//     };
//     let converted = temperature * 1.8 + 32.0;
//     println!("{} Celsius = {} Fahrenheit", temperature, converted);
// }
