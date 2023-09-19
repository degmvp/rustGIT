use std::io; // Import the 'io' module from the 'std' crate.

fn main() {
    let mut s = String::new();
    println!("{}", "-".repeat(50));

    println!("Digite um texto");

    io::stdin()
        .read_line(&mut s) // Pass a mutable reference to the string.
        .expect("error reading console");

    println!("Você digitou: {}", s); // Use {} to print the value of 's'.

    println!("Quantidade de bytes {}", s.trim().len());
    println!("Quantidade de chars {}", s.trim().chars().count());
    println!("{}", "-".repeat(50));

    let banner = 
        "impressao da linha 1 
     impressao da linha 2 
     impressao da linha 3";
    println!("{banner}");
    println!("{}", "-".repeat(50));

}


