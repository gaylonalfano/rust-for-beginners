// Topic: Trait Objects
//
// NOTES:
// * Traits don't have sizes (they're not types!), so cannot calculate
//   their memory allocation/layouts properly.
// * "Runtime Generics" - dynamically allocated object.
// -- More flexible than generics, which determine type at COMPILE time.
// -- Uses "Dynamic Dispatch" (Runtime) vs. "Static Dispatch" (Compile time)
// * Trait Objects can be different types and can be mixed into
//   a single collection (recall Vec<Shape>).
// * Polymorphic program behavior
// * Create using -- let keeb: Box<dyn Clicky> = Box::new(Keyboard);
// -- This allows us to move it around instead of dealing with references.
// * Storing Box types in Vector is preferred, so Vec owns the objects
// * Box is a usize, since it's a pointer
// * 'dyn Trait' tells Rust we're going by the Trait, NOT the Struct
// * Traits can only access data through functions that are implemented
//     on a structure! Traits cannot access structure fields directly,
//     since each struct may have different fields! Therefore, we have to
//     completely rely on functions to get any values.

//
// // === Example ===
// trait Clicky {
//     fn click(&self);
// }
//
// struct Keyboard;
//
// impl Click for Keyboard {
//     fn click(&self) {
//         println!("click clack")
//     }
// }
//
// fn main() {
//     // == Creating Trait Objects Basics ==
//     // Create a Trait Object - Version 1:
//     let keeb = Keyboard;
//     // Create our Trait Object. 'dyn' is Dynamic Dispatch
//     let keeb_obj: &dyn Clicky = &keeb;
//
//     // Create a Trait Object - Version 2:
//     // Immediately borrow a structure you created.
//     // This isn't preferred, since storing a ref into a vector
//     // isn't always useful.
//     let keeb: &dyn Clicky = &Keyboard;
//
//     // Create a Trait Object - Version 3:
//     // Place into a Box (store on Heap), so we can move it around
//     // when needed, instead of managing references (V1 & V2)
//     let keeb: Box<dyn Clicky> = Box::new(Keyboard);
//
//     // == Using Trait Objects as fn parameters ==
//     // V1 - &dyn Trait
//     // Borrow a Trait Object for this fn (&dyn Clicky)
//     // This is the easier/preferred way of creating Trait Objects,
//     // since you don't have to annotate everything. As long as you
//     // annotate somewhere, Rust will be able to infer the type.
//     fn borrow_clicky(obj: &dyn Clicky) {
//         obj.click();
//     }
//     let keeb = Keyboard;
//     borrow_clicky(&keeb);
//
//     // V2 - Box<dyn Trait>
//     fn move_clicky(obj: Box<dyn Clicky>) {
//         obj.click();
//     }
//     let keeb = Box::new(Keyboard);
//     move_clicky(keeb);
//
//     // == Heterogeneous Vectors ==
//     struct Mouse;
//     impl Click for Mouse {
//         fn click(&self) {
//             println!("click");
//         }
//     }
//     // V1
//     let keeb: Box<dyn Clicky> = Box::new(Keyboard);
//     let mouse: Box<dyn Clicky> = Box::new(Keyboard);
//     let clickers = vec![keeb, mouse];
//
//     // V2 - Preferred (easier to change)
//     let keeb = Box::new(Keyboard);
//     let mouse = Box::new(Mouse);
//     let clickers: Vec<Box<dyn Clicky>> = vec![keeb, mouse];
//
//     // Now can use inside a fn
//     fn make_clicks(clickeys: Vec<Box<dyn Clicky>>) {
//         // 'clicker' is Box<dyn Clicky> size=16, align=8
//         for clicker in clickeys {
//             clicker.click();
//         }
//     }
//
//     make_clicks(clickers);
// }

// // === Demo ===
// trait Sale {
//     fn amount(&self) -> f64;
// }
//
// // The (f64) is a Tuple
// struct FullSale(f64);
// impl Sale for FullSale {
//     fn amount(&self) -> f64 {
//         self.0
//     }
// }
//
// struct OneDollarOffCoupon(f64);
// impl Sale for OneDollarOffCoupon {
//     fn amount(&self) -> f64 {
//         self.0 - 1.0
//     }
// }
//
// struct TenPercentOffPromo(f64);
// impl Sale for TenPercentOffPromo {
//     fn amount(&self) -> f64 {
//         self.0 * 0.9
//     }
// }
//
// fn calculate_revenue(sales: &Vec<Box<dyn Sale>>) -> f64 {
//     sales.iter().map(|sale| sale.amount()).sum()
// }
//
// fn main() {
//     let price = 20.0;
//     let regular = Box::new(FullSale(price));
//     let coupon = Box::new(OneDollarOffCoupon(price));
//     let promo = Box::new(TenPercentOffPromo(price));
//
//     // Need to use 'dyn' when working with Trait Objects,
//     // so Rust knows to treat dynamically at runtime.
//     let sales: Vec<Box<dyn Sale>> = vec![regular, coupon, promo];
//     let revenue = calculate_revenue(&sales);
//     println!("{}", revenue);
// }

// === Activity ===
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

// U: I only had cost_per_sq_meter(), but Jayson had more:
trait Material {
    fn cost_per_sq_meter(&self) -> f64;
    fn square_meters(&self) -> f64;
    // Provide default value using helper functions
    // NOTE: This is convenient since we don't have to write similar
    // code for all the possible structures that impl this trait.
    fn total_cost(&self) -> f64 {
        self.cost_per_sq_meter() * self.square_meters()
    }
}

// U: Jayson added a Tuple struct instead of a field like I did
struct Carpet(f64);
impl Material for Carpet {
    fn cost_per_sq_meter(&self) -> f64 {
        10.0
    }
    fn square_meters(&self) -> f64 {
        self.0 // From Carpet(f64) Tuple
    }
}

struct Tile(f64);
impl Material for Tile {
    fn cost_per_sq_meter(&self) -> f64 {
        15.0
    }
    fn square_meters(&self) -> f64 {
        self.0
    }
}

struct Wood(f64);
impl Material for Wood {
    fn cost_per_sq_meter(&self) -> f64 {
        20.0
    }
    fn square_meters(&self) -> f64 {
        self.0
    }
}

fn calculate_total_cost(materials: Vec<Box<dyn Material>>) -> f64 {
    materials.iter().map(|mat| mat.total_cost()).sum()
}

fn main() {
    let carpet = Box::new(Carpet(3.0));
    let tile = Box::new(Tile(12.0));
    let wood = Box::new(Wood(8.0));

    let materials: Vec<Box<dyn Material>> = vec![carpet, tile, wood];
    let total = calculate_total_cost(materials);
    println!("Total cost = {}", total);
}
