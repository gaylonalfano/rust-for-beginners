// NOTES
// - The Default Trait allows you to specify a struct's properties
// without the need to pass any arguments.
// - NOTE If you impl Package and have a fn new() WITHOUT any arguments,
// then it's better to use default() instead of new()

#[derive(Debug)]
struct Package {
    weight: f64,
}

impl Package {
    // - NOTE If you impl Package and have a fn new() WITHOUT any arguments,
    // then it's better to use default() instead of new()
    // In this case, our new() DOES have the 'weight' parameter, so it's better
    // to keep/use the new() function. Otherwise, just use default() -> Self
    fn new(weight: f64) -> Self {
        Self { weight }
    }
}

impl Default for Package {
    // NOTE The only function provided by the Default trait is default()
    // and it only returns the specified enum/struct it's applied to
    fn default() -> Self {
        Self { weight: 3.0 }
    }
}


fn main() {
    let p_default = Package::default();
    let p_custom = Package::new(5.5);

    println!("default = {:?}", p_default);
    println!("custom = {:?}", p_custom);
}
