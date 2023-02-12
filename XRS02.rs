use num::abs;
use num::signum;

//extern "C" {
//fn abs(input: i32) -> i32;
//}

fn main() {
println!("---------------------------------------------------------");
println!("Algoritmo boleano : Solução quando (a = b)");
println!("Delta1 (a = b) solução binaria : 1 -  abs(signum(a - b))");
println!("---------------------------------------------------------");
println!("Crates use num::abs;\n       use num::signum;");
println!("---------------------------------------------------------");
println!("Absolute value of 0 : {}", abs(0));
println!("Absolute value of 1 : {}", abs(1));
println!("Absolute value of -1: {}", abs(-1));
println!("se a = b retorna 1 true:  {}", 1 -  abs(signum(5 - 5))); 
println!("se a > b retorna 0 false: {}", 1 -  abs(signum(5 - 1)));
println!("se a < b retorna 0 false: {}", 1 -  abs(signum(5 - 10))); 


}


use num::abs;

fn main() {
    let x = -42;
    let y = abs(x);
    println!("O valor absoluto de {} é {}.", x, y);
}

use num::signum;

fn main() {
    let x = -42;
    let y = signum(x);
    println!("O sinal de {} é {}.", x, y);
}