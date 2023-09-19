use rand::Rng;
use rand::seq::SliceRandom;

fn main() {
    let mut rng = rand::thread_rng();
    let min = 1;
    let max = 100;

    let random_number = rng.gen_range(min..=max);

    println!("Calling extern crate rand ");
    println!("Random number between {} and {}: {}", min, max, random_number);

    let mut vec = vec![1, 2, 3, 4, 5];
    let mut rng = rand::thread_rng();

    vec.shuffle(&mut rng);

    println!("Shuffled vector: {:?}", vec);


    let mut rng = rand::thread_rng();
    let random_bool: bool = rng.gen();
    println!("Random boolean: {}", random_bool);

    let items = ["apple", "banana", "orange", "grape", "pear"];
    let mut rng = rand::thread_rng();

    if let Some(random_item) = items.choose(&mut rng) {
        println!("Random fruit: {}", random_item);
    } else {
        println!("No fruit found!");
    }
}


   
