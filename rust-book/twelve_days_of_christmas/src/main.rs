use std::convert::TryFrom;

struct DayOfChristmas {
    day: u32,
    ordinal: String,
    lyric: String,
}

impl DayOfChristmas {
    // TODO Could consider impl a print_intro() as well...
    // Q: Can I use ordinal: String instead of ordinal: &str?
    // A: Yes, but when invoking you must use .to_owned() or String::from()
    // Using &str also allows me to do the conversion etc behind the scenes
    fn new(day: u32, ordinal: &str, lyric: &str) -> Self {
        Self {
            day,
            ordinal: ordinal.to_owned(),
            lyric: lyric.to_owned(),
        }
    }

    fn print(&self) {
        println!("{}:{}::{}", self.day, self.ordinal, self.lyric);
    }


    fn print_lyric(&self) {
        println!("{}", self.lyric);
    }
}

fn main() {
    let days: [DayOfChristmas; 12] = [
        DayOfChristmas {
            day: 1,
            ordinal: String::from("first"),
            lyric: "a partridge in a pear tree".to_owned(),
        },
        DayOfChristmas {
            day: 2,
            ordinal: "second".to_owned(),
            lyric: String::from(
                "two turtle doves"
            )
        },
        DayOfChristmas::new(3, "third", "three French hens"),
        DayOfChristmas::new(4, "fourth", "four calling birds"),
        DayOfChristmas::new(5, "fifth", "five gold rings"),
        DayOfChristmas::new(6, "sixth", "six geese a laying"),
        DayOfChristmas::new(7, "seventh", "seven swans a swimming"),
        DayOfChristmas::new(8, "eighth", "eight maids a milking"),
        DayOfChristmas::new(9, "ninth", "nine ladies dancing"),
        DayOfChristmas::new(10, "tenth", "ten lords a leaping"),
        DayOfChristmas::new(11, "eleventh", "eleven pipers piping"),
        DayOfChristmas::new(12, "twelfth", "twelve drummers drumming"),
    ];

    for d in &days {
        // Need to borrow &days
        compile_intro(d);

        // Q: How to get a slice/splice of days array?
        // I'm loop in reverse over days array and compile_line() for all
        // previous days in the array.
        // A: Can consider converting type from u32 to usize to slice array
        // https://stackoverflow.com/questions/43704758/how-to-idiomatically-convert-between-u32-and-usize
        let day_index: usize = usize::try_from(d.day).unwrap();
        // let slice = &days[1..(day_index + 1)]; // error if index isn't usize
        // let slice = &days[1..3];  // works since it's a range
        // &days[day_index - 1].print();

        for i in (0..day_index).rev() {

            // if day_index == 1 {
            //     // Only print the lyric, don't mess with prefix from compile_line().
            //     println!("{}", &days[i].lyric);
            // } else {
            //     compile_line(&days[i]);
            // }

            // NOTE Cannot use match because 1 => String and _ => compile_line() function!
            // But could consider using DayOfChristmas.print() function so that the match
            // arms types are both functions.
            match day_index {
                1 => days[i].print_lyric(),
                _ => compile_line(&days[i]),
            }
        }
    }
}

fn compile_intro(day: &DayOfChristmas) {
    let intro = &day.ordinal;

    println!("\nOn the {} day of Christmas my true love gave to me:", intro);
}

fn compile_line(day: &DayOfChristmas) {
    let line = &day.lyric;

    // Shadow the passed arg to determine if it's the first day (ie uses "and")
    let prefix = match day.day {
        1 => "and ",
        _ => "",
    };

    println!("{}{}", prefix, line);
}
