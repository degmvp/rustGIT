// JP,2020/09/21 formato yyyy-mm-dd                                   
// Documentação de código RUST     
                                
////////////////////////////////////                                       
// Function lwr3()
// `i32` for 32-bit signed integers
// Immutable bindings  
// Integer/float suffixes 
// Arithmetic  String literals   
// A string slice    
/////////////////////////////////// 

#[allow(dead_code)]
// Functions
// `i32` is the type for 32-bit signed integers
fn add2(x: i32, y: i32) -> i32 {
    // Implicit return (no semicolon)
    x + y
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
// exec function
pub fn  lwr3()  {
    println!("modulo lwr003.rs!");

    // Immutable bindings
    let x: i32 = 1;

    // Integer/float suffixes
    let y: i32 = 13i32;
    let f: f64 = 1.3f64;

println!("{} {} {}", x,y,f);
println!("imprimindo os valores de x, y, f");
    // Type inference
    // Most of the time, the Rust compiler can infer what type a variable is, so
    // you don’t have to write an explicit type annotation.
    // Throughout this tutorial, types are explicitly annotated in many places,
    // but only for demonstrative purposes. Type inference can handle this for
    // you most of the time.
    let implicit_x = 1;
    let implicit_f = 1.3;

    // Arithmetic
    let sum = x + y + 13;

    // Mutable variable
    let mut mutable = 1;
    mutable = 4;
    mutable += 2;

    // Strings //

    // String literals
    let x: &str = "JP 14/09/2020! rusrt in action";

    // Printing
    println!("{} {}", x, f); // JP 14/09/2020! rusrt in action 1.3 
    
    // A `String` – a heap-allocated string
    let s: String = "heap-allocated string".to_string();

    // A string slice – an immutable view into another string
    // The string buffer can be statically allocated like in a string literal
    // or contained in another object (in this case, `s`)

    let s_slice: &str = &s;

    println!("{} ", s); 
    println!("string slice {} ", s_slice); 

    println!("A string slice – an immutable view into another string");
    }