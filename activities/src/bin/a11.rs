// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter


// * Use a struct for the grocery item
struct GroceryItem {
    // * Use two i32 fields for the quantity and id number
    quantity: i32,
    id: i32,
}

// * Create a function to display the quantity, with the struct as a parameter
fn display_quantity(item: &GroceryItem) {
    println!("quantity = {:?}", item.quantity);
}

// * Create a function to display the id number, with the struct as a parameter
fn display_id(item: &GroceryItem) {
    println!("id = {:?}", item.id);
}

fn main() {
    let pringles = GroceryItem {
        quantity: 12,
        id: 2,
    };

    display_quantity(&pringles);
    display_id(&pringles);
}

// DEMO
// struct Book {
//     pages: i32,
//     rating: i32,
// }

// fn display_page_count(book: &Book) {
//     println!("pages = {:?}", book.pages);
// }

// fn display_rating(book: &Book) {
//     println!("rating = {:?}", book.rating);
// }

// fn main() {
//     // NOTE All data is owned by some part within the program
//     // e.g., 'book' is owned by main().
//     // This means that main() is responsible for cleaning up the memory,
//     // which is called a 'drop' - dropping the memory. Memory gets automatically
//     // dropped once the end of block is reached.
//     let book = Book {
//         pages: 5,
//         rating: 9,
//     };
//     // NOTE Now if we transfer ownership of book to another fn, then that new fn
//     // is the new owner and it will drop 'book' once the end of its scope is reached.
//     // This also means that the 'book' will no longer be available inside main()
//     // UNLESS you BORROW using &book. When we borrow the data, we cannot drop it like
//     // a move can, since the original owner is maintained.
//     display_page_count(&book);
//     display_rating(&book);
// }
