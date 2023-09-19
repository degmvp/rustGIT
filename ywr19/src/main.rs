// Remove something to make it work
fn main() {
    let x = 5;
    let y: u32;

    y = 1945;
    println!("{} {}", x,y);

    let z: i32 = 10; // Type of z ?

    println!("{}",z);

        // Bitwise operations
        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        println!("1 << 5 is {}", 1u32 << 5); // 0000 0001 -> 0010 0000
        println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); // 0x80 = 128 (8 x 16) -> 1000 0000 -> 0010 0000
                                                      // -> 32 -> 0x20
        
        let w = 5.0;
        let result = type_of(&w);
        println!("a variavel w é  do tipo: {}", result);

        let yr: f32 = 21474836.48;
        let max_i8 = type_of(&yr);
        println!("a variavel yr f32 é  do tipo: {}", max_i8);

        let ys: u32 = 2147483648;
        let max_u8 = type_of(&ys);
        println!("a variavel ys u32 é  do tipo: {}", max_u8);

        let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255

        println!("v = {}", v);
        type_of(&v);
        println!("a variavel v é  do tipo: {}", max_u8);

        let xx = 1_000.000_1; // ?
        let yy: f32 = 0.12; // f32
        let z = 0.01_f64; // f64
    
        println!("xx = {}", type_of(&xx));
        println!("yy = {}", type_of(&yy));    
        println!("z  = {}", type_of(&z));

        println!("xx  = {}", xx);
        println!("yy  = {}", yy);
        println!("z   = {}", z);
     
    
// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    println!("sum = {}", sum); // -5

    assert!(sum == -5);

    for c in 'a'..='j' {
        println!("{}", c as u32);
    }
}
