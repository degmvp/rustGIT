// JP,2020/10/16 formato yyyy-mm-dd                                                                   -->
// Documentação de código RUST                                                                     -->
////////////////////////////////////////
//pattern match - args turbo fish
//let args = env::args().collect::<Vec<String>>();                                                                                                  -->
////////////////////////////////////////
use std::env;
pub fn lwr10 () {

let args = env::args().collect::<Vec<String>>();

let value = &args[1];

match value.as_ref() {
 "45" => println!("pattern match: {}", value),
 "83" => println!("pattern match: {}", value),
   _ => println!("final do pattern match"),

}

}
