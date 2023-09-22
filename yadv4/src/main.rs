
//////////Rust has four primary scalar types://////////////////////
/// integers, floating-point numbers, Booleans, and characters.////
/// ///////////////////////////////////////////////////////////////
use std::io;
fn main() {
    println!("-----------Tupla---------------");
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x, y, z: {x},{y},{z}");

    println!("-----------Vector--------------");
// Create a vector with the values 1983, 1980, and 1945
let mut values = vec![1983, 1980, 1945];

// Use indexing to get and remove values from the vector
let a = values.remove(0);
let b = values.remove(0);
let c = values.remove(0);

// Print the values in variables a, b, and c
println!("a: {}", a);
println!("b: {}", b);
println!("c: {}", c);



////the index of the value to access///

println!("-----------Index--------------------------------");

let x: (i32, f64, u8) = (5500, 6.40, 185);

    let index_0 = x.0;

    let index_1 = x.1;

    let index_2 = x.2;

    println!("access by index_0 {}, index_1 {}, index_2 {}", index_0,index_1,index_2 );
    println!("-----------Index--------------------------------");

    let ax = [1000, 2000, 3000, 4000, 5000];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = ax[index];

    println!("The value of the element at index {index} is: {element}");


    // Iterate through the array and print each element
    fn calc64() -> u128 {
        let result = 2_u128.pow(64); // Calculate 2^64 as a u128
        result
    }

    let result = calc64(); // Call the function with the base value 2

    println!("Result 2 ** 64 : {}", result);



}
