use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Family {
    name: String,
    country: String,
}

impl Family {
    /// Creates a new Viking.
    fn new(name: &str, country: &str) -> Family {
        Family {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    // Use a HashMap to store the vikings' health points.
    let family: HashMap<Family, i32> = HashMap::from([
        (Family::new("Degmar  ", "Brazil "), 78),
        (Family::new("Lawrence", "Brazil "), 40),
        (Family::new("Kecio   ", "States "), 37),
        (Family::new("Wallace ", "Brazil "), 35),
        (Family::new("Camila  ", "States "), 10),
    ]);

    // Use derived implementation to print the status of the family.
    for (family, health) in &family {
        println!("{:?} age {} ", family, health);
    }
}