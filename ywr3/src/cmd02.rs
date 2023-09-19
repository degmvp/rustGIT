use num::abs;
use num::signum;

//extern "C" {
//fn abs(input: i32) -> i32;
//}

pub fn cmd02_fn() {
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
let life = 78;
println!("idade {:?}",life );
println!("idade {life}" );

println!("se a = b signum retorna 0 false:  {}",signum(5 - 5)); 
println!("se a > b signum retorna 1 true:   {}",signum(5 - 1));
println!("se a < b signum retorna -1 false: {}",signum(5 - 10)); 

fn add(a: i32, b: i32) -> i32 {
    a + b 
}
let x = add(1, 1);
let y = add(3, 0);
let z = add(x, 1);

println!("x {},y {}, z {}", x,y,z);

let elements = [4, 8, 15, 16, 23, 42, 7, 11, 6, 32, 19, 5];
    let target = 23;
    let mut found = false;

    for element in elements.iter() {
        if *element == target {
            found = true;
            break; // Exit the loop when the element is found
        }
    }

    if found {
        println!("Element {} found!", target);
    } else {
        println!("Element {} not found!", target);
    }


}