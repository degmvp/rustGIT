// JP,2020/09/21 formato yyyy-mm-dd                                   
// Documentação de código RUST     
                                
/////////////////////////////////                                       
//use rand::prelude::*;    
//random() thread_rng()
//rng.gen() rng.gen_range()    
//shuffle()                             
///////////////////////////////// 

use rand::prelude::*;

pub fn lwr4(){
    println!("modulo lwr004.rs!");
    let x: u8 = random();
    println!("{}", x);

    if random() { // generates a boolean
        println!("Heads!");
    }
    
    let mut rng = thread_rng();
    if rng.gen() { // random bool
        let x: f64 = rng.gen(); // random number in range [0, 1)
        let y = rng.gen_range(-10.0, 10.0);
        println!("x is: {}", x);
        println!("y is: {}", y);
        println!("Number from 0 to 9: {}", rng.gen_range(0, 10));
    }    
    // Sometimes it's useful to use distributions directly:
    let distr = rand::distributions::Uniform::new_inclusive(1, 25);
    let mut nums = [0i32; 5];
    for x in &mut nums {
        *x = rng.sample(distr);
    }
    println!("Some numbers: {:?}", nums);
    
        // We can also interact with iterators and slices:
    let arrows_iter = "➡⬈⬆⬉⬅⬋⬇⬊".chars();
    println!("Lets go in this direction: {}", arrows_iter.choose(&mut rng).unwrap());
    let mut nums = [1, 2, 3, 4, 5];
    nums.shuffle(&mut rng);
    println!("I shuffled my {:?}", nums);
    
}