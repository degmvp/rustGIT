// JP,2020/10/16 formato yyyy-mm-dd                                                                   -->
// Documentação de código RUST                                                                     -->
////////////////////////////////////////
//desestruturar o valor de uma tupla"
//Shadowing variaveis                                                                                          -->
////////////////////////////////////////
pub fn lwr15() {

    println!("desestruturar o valor de uma tupla");
    
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("O valor de x:y:z {} {} {}", x,y,z);
    println!("o valor da tupla {:?}",tup);

    let s1 = String::from("texto");

    let tamanho = &s1;
    let tamanho = tamanho.len();

    println!("O tamanho de '{}' é {}.", s1, tamanho);
    
    
    println!("Shadowing variaveis com o mesmo nome e tipos diferentes");
    
    let letras = "abcdef";
    let alf    = &letras;
    let letras = letras.len();
    println!("O valor de letras : {} {}", alf,letras);
    
    println!("Shadowing variaveis com o mesmo nome valores diferentes");
    
    let x = 5;
    let x = x + 1;
    let x = x * 2;
   
    println!("O valor de x é: {}", x);
    
}
