// JP,2020/10/16 formato yyyy-mm-dd                                                                   -->
// Documentação de código RUST                                                                     -->
////////////////////////////////////////
//Rust's unsigned integer types
//Rust's signed   integer types                                                                                         -->
////////////////////////////////////////
pub fn lwr16() {
    let x = 10i32;
    let y: i32 = 20;

    let z: bool;
    z = true;
    println!("Rust allows you to declare a variable with.."); 
    println!("assigning any specific type and value"); 
    println!("----------------------------- "); 
    println!("x = {} | y = {} | z = {}", x, y, z);
    println!("----------------------------- "); 
    const THRESHOLD: i32 = 10;
    const PI: f32 = 3.14;

    println!("The threshold is {}", THRESHOLD);
    println!("pi value is {}", PI);
    
println!("----------------------------- ");    
println!("Rust's unsigned integer types ");
println!("-----------------------------
u8	0 to 28–1 (0 to 255)
u16	0 to 216–1 (0 to 65,535)
u32	0 to 232–1 (0 to 4,294,967,295)
u64	0 to 264–1 (0 to 18,446,744,073,709,551,615, or 18 quintillion)
u128	0 to 2128–1 (0 to around 3.4✕1038)
usize	0 to either 232–1 or 264−1\n");

println!("Rust's signed integer types ");
println!("-----------------------------
i8	−27 to 27−1 (−128 to 127)
i16	−215 to 215−1 (−32,768 to 32,767)
i32	−231 to 231−1 (−2,147,483,648 to 2,147,483,647)
i64	−263 to 263−1 (−9,223,372,036,854,775,808 to 9,223,372,036,854,775,807)
i128	−2127 to 2127−1 (approx -1.7✕1038 to +1.7✕1038)
isize	Either −231 to 231−1, or −263 to 263−1");
}
