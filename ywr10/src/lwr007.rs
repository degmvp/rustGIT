// JP,2020/09/30 formato yyyy-mm-dd                                                                   -->
// Documentação de código RUST                                                                     -->
////////////////////////////////////////
// função lw7()                                                                                                       -->
////////////////////////////////////////   
//Algoritmo boleano : Solução quando (a = b)");
//---------------------------------------------------------");
//Delta1 (a = b) solução binaria : 1 -  abs(sign(a - b))");
//---------------------------------------------------------");

pub fn lwr7(a: i32,b: i32) {
    if a == b {
        println!("a == b : {}", 0);
    }else
    if a > b {
        println!("a > b : {}", 1);
    }else    {
        println!("a < b : {}", -1);
    }    

}