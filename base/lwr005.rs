// JP,2020/09/20 formato yyyy-mm-dd
// Documentação de código RUST
///////////////////////////////
//Rotina de leitura arquivo txt
/////////////////////////////// 
use std::fs::File;
use std::io::prelude::*;
pub fn lwr5() {
    let mut file = File::open("info").expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
    .expect("Oops! can not read the file...");
    println!("modulo lwr005.rs!");
    println!("File Contents:\n\n{}", contents);

}