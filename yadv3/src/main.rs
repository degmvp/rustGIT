fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    /////////////////part2////////////
    let a = [101, 120, 130, 240, 350];

    for element in a {
        println!("the value is: {element}");
    }
    ////////////////////part3///////////
    for number in (1..10).rev() {
        println!("{number} ");
    }
   ////////////////////////////////////
   let mut s = String::from("Degmar ");

    s.push_str(" Barbosa!"); // push_str() appends a literal to a String

    println!("{}", s);
    //////////// ownership /////////////////////
    let s1 = String::from("Rust-Course");
    let s2 = s1;
    println!("valor de s2: {}",s2);

    //////////// ownership ////////////////////
    let x1 = 1945;
    let x2 = x1;
    println!("valor de x2: {}",x2);
    println!("valor de x1: {}",x1);
    //////////////// clone ///////////////////
    let y1 = String::from("Rust Clone()");
    let y2 = y1.clone();

    println!("y1 = {}, y2 = {}", y1, y2);

    /////////////////shadow////////////////////////
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}")
    /////////////////////////////////////////
}