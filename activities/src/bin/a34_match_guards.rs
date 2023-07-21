// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else should not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles
//
#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn print_tile(tile: Tile) {
    match tile {
        t @ Tile::Dirt | t @ Tile::Sand | t @ Tile::Grass => println!("Ground tile: {:?}", t),
        Tile::Brick(bs @ BrickStyle::Red | bs @ BrickStyle::Gray) => {
            println!("Brick color is: {:?}", bs)
        }
        Tile::Brick(bs) => println!("{:?} brick", bs),
        _ => println!("Todo..."),
    }
}

fn main() {
    let red_brick = BrickStyle::Red;
    let dungeon_brick = BrickStyle::Dungeon;
    let dirt_tile = Tile::Dirt;
    print_tile(dirt_tile);
}

// === Example 1
// enum Status {
//     Error(i32),
//     Info,
//     Warn,
// }
//
// fn main() {
//     let status = Status::Error(5);
//
//     match status {
//         Status::Error(s @ 3) => println!("error three"),
//         Status::Error(s @ 5..=6) => println!("error 5 or 6: {}", s),
//         Status::Error(s @ 4..=10) => println!("error 4 through ten: {}", s),
//         Status::Error(s @ 18 | s @ 19) => println!("error 18 or 19"),
//         Status::Error(s @ 4..=10) => println!("error 4 through ten: {}", s),
//         Status::Error(s) => println!("error code: {}", s),
//         Status::Info => println!("info"),
//         Status::Warn => println!("warn"),
//     }
// }

// === Example 2
//
// enum Species {
//     Finch,
//     Hawk,
//     Parrot,
// }
//
// struct Bird {
//     age: usize,
//     species: Species,
// }
//
// #[rustfmt::skip]
// fn main() {
//     let hawk = Bird {
//         age: 13,
//         species: Species::Hawk
//     };
//
//     match hawk {
//         // NOTE: Because we're matching on a struct,
//         // we include the { .. } instead of Enum's ()
//         Bird { age: 4, .. } => println!("4 year old bird"),
//         Bird { age: 4..=10 | 15..=20, .. } => println!("4-10 or 15-20 years old. Don't care about species."),
//         Bird { species: Species::Finch, .. } => println!("finch! Don't care about age."),
//         // NOTE: '..' represents any or all values.
//         Bird { .. } => println!("other bird")
//
//     }
//
// }
//

// === Example 3
//
// #[derive(PartialEq, PartialOrd, Eq, Ord)]
// enum Difficulty {
//     Easy,
//     Normal,
//     Hard,
// }
//
// fn main() {
//     let stage = 5;
//     let diff = Difficulty::Normal;
//
//     match stage {
//         s if (s == 5 && diff == Difficulty::Easy) => println!("easy mode"),
//         s if diff == Difficulty::Normal => println!("normal mode stage: {}", s),
//         s @ 10 | s @ 15 => println!("stage 10 or 15"),
//         s => println!("stage: {}", s),
//     }
// }

// === Example 4
// struct Vehicle {
//     km: usize,
//     year: usize,
// }
//
// fn main() {
//     let car = Vehicle {
//         km: 80_000,
//         year: 2020,
//     };
//
//     match car {
//         Vehicle { km, year } if km == 0 && year == 2020 => println!("new 2020 vehicle"),
//         Vehicle { km, .. } if km <= 50_000 => println!("under 50k km"),
//         Vehicle { km, .. } if km >= 80_000 => println!("at least 80k km"),
//         Vehicle { year, .. } if year == 2020 => println!("made in 2020"),
//         Vehicle { .. } => println!("other mileage"),
//     }
// }
