// JP,2020/09/21 formato yyyy-mm-dd                                   
// Documentação de código RUST     
                                
/////////////////////////////////                                       
    
///////////////////////////////// 
use rand::prelude::*;

pub fn lwr4(){
    println!("Rust usando modulos!");
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(1..=10);
    println!("Random number between 1 and 10: {}", random_number);


    let mut rng = rand::thread_rng();
    let random_bool: bool = rng.gen();
    println!("Random boolean: {}", random_bool);

    let mut rng = rand::thread_rng();
    let mut data = vec![1, 2, 3, 4, 5];
    data.shuffle(&mut rng);
    println!("Shuffled data: {:?}", data);
    println!("------------------------------------");
    let mut rng = rand::thread_rng();
    let random_float: f64 = rng.gen();
    println!("Random floating-point number: {}", random_float);    
    println!("------------------------------------"); 
    println!("------------------------------------");
}

