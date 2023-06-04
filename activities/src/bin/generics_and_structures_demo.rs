struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

trait Convey {
    fn weight(&self) -> f64;
    fn dimensions(&self) -> Dimensions;
}

// NOTE: Tell the compiler we have a generic param
// inside that impls the Convey trait.
struct ConveyorBelt<T: Convey> {
    pub items: Vec<T>,
}

// The ConveyorBelt is generic over type T.
// Type T must impl the Convey Trait.
impl<T: Convey> ConveyorBelt<T> {
    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }
}

struct CarPart {
    width: f64,
    height: f64,
    depth: f64,
    weight: f64,
    part_number: String,
}

impl Default for CarPart {
    fn default() -> Self {
        Self {
            width: 5.0,
            height: 1.0,
            depth: 2.0,
            weight: 3.0,
            part_number: "abc".to_owned(),
        }
    }
}

impl Convey for CarPart {
    fn weight(&self) -> f64 {
        self.weight
    }

    fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.width,
            height: self.height,
            depth: self.depth,
        }
    }
}

fn main() {
    // Restricting this 'belt' for only type CarPart
    let mut belt: ConveyorBelt<CarPart> = ConveyorBelt { items: vec![] };
    belt.add(CarPart::default());

    let mut belt = ConveyorBelt { items: vec![] };
    belt.add(5); // Error since integer doesn't impl Convey trait
}
