// Define a struct named 'Person' and derive the 'Debug' trait
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Instantiate a 'Person' struct
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Access and print a field of the struct
    println!("Name: {}", person1.name);
    println!("Age : {}", person1.age);
    
    // Display the entire struct
    println!("Person Information: {:?}", person1);
}
