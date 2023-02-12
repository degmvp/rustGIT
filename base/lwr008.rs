// JP,2020/10/16 formato yyyy-mm-dd                                                                   -->
// Documentação de código RUST                                                                     -->
////////////////////////////////////////
// extern "C"                                                                                                      -->
////////////////////////////////////////
//---------------------------------------------------------
//"Algoritmo boleano : Solução quando (a = b)"
//"Delta1 (a = b) solução binaria : 1 -  abs(sign(a - b))"
//---------------------------------------------------------
//Crates extern C fn abs;"       
//use crate::base::lwr007::lwr7
//---------------------------------------------------------

extern "C" {
    fn abs(input: i32) -> i32;
    }

 use crate::base::lwr007::lwr7;

pub fn lwr8() {
println!("---------------------------------------------------------");
println!("Algoritmo boleano : Solução quando (a = b)");
println!("Delta1 (a = b) solução binaria : 1 -  abs(sign(a - b))");
println!("---------------------------------------------------------");
println!("Crates extern C fn abs;");       
println!("use crate::base::lwr007::lwr7;");
println!("---------------------------------------------------------");
unsafe {
println!("Absolute value of 0 : {}", abs(0));
println!("Absolute value of 1 : {}", abs(1));
println!("Absolute value of -1: {}", abs(-1));

println!("se a = b retorna  0  true:  {}", 1 -  abs(lwr7(5,5))); 
println!("se a > b retorna  1  false: {}", 1 -  abs(lwr7(5,1)));
println!("se a < b retorna -1  false: {}", 1 -  abs(lwr7(5,10))); 
}
println!("---------------------------------------------------------");
}