use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number between 1 and 10!");

    let secret_number = rand::thread_rng().gen_range(1..11);
    
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Alternative w/o use statement: 
        // std::io::stdin().read_line().expect();
        io::stdin()
            .read_line(&mut guess) // read whatever user types plus \n
            .expect("Failed to read line!");

        // Shadow the previous value of guess (String) with a new one (u32) to convert types.
        let guess: u32 = match guess.trim().parse() {
            // NOTE parse() -> Result type (enum) with Ok and Err variants
            // NOTE We're handling the error using match instead of crashing on
            // an error using expect.
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
