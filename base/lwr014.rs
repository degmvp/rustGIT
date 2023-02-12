// JP,2020/10/16 formato yyyy-mm-dd                                                                   -->
// Documentação de código RUST                                                                     -->
////////////////////////////////////////
//atribuição multipla
//variavel mutavel
//Atribuição com inferencia    
//Constantes 
//Funções
//Chamada de função por referenciia                                                                                             -->
////////////////////////////////////////
pub fn  lwr14()  {
    
    let  (day,  month,  year)  =  (11, 04,1945);
    println!("Atribuição multipla");
    println!("-----------------------------");
    println!("Nascido em :  {}/{}/{}",day,month,year);
            println!("-----------------------------");
    
    let dia = 31;
    let mes = 1;
    let ano = 1983;
    println!("Atribuição com inferencia");
    println!("-----------------------------");
    println!("Nascido em :  {}/{}/{}",dia,mes,ano);
            println!("-----------------------------");
    
    
    println!("Declarando variáveis mutáveis");
    println!("-----------------------------");
    
    let mut _a = 20;
    _a = 22;
    println!("variavel mutavel {}",_a);
            println!("-----------------------------");
    
    println!("Constantes");
    println!("-----------------------------");
    
    const Y: i32 = 1945;
    let x = 75;
    let x = Y + x;
    
    println!("Const:{} mut x {}",Y,x);
            println!("-----------------------------");
            
    println!("Funções");
    println!("-----------------------------");         
            
    fn sum(a: i32, b: i32) -> i32 {
    a + b
    }
    let   x   =   3;
    let   y   =   5;
    println!(" soma {}   +   {}   =   {}",   x,   y,   sum(x,   y));
    
            println!("-----------------------------");         
            
    fn sub(a: i32, b: i32) -> i32 {
    a - b
    }
    let   x   =   3;
    let   y   =   5;
    println!(" subt {}   -   {}   =   {}",   x,   y,   sub(x,   y));
                println!("-----------------------------"); 
                
   fn mul(a: i32, b: i32) -> i32 {
    a * b
    }
    let   x   =   3;
    let   y   =   5;
    println!(" mul  {}   *   {}   =   {}",   x,   y,   mul(x,   y));
                println!("-----------------------------"); 
    
    fn div(a: i32, b: i32) -> i32 {
    a / b
    }
    let   x   =  35;
    let   y   =   5;
    println!(" div {}   /   {}   =   {}",   x,   y,   div(x,   y));
                println!("-----------------------------"); 
                
fn print_today() {
    let dia = 13;
    let mes = 10;
    let ano = 2020;
    println!("-----------------------------");
    println!("data de hoje :  {}/{}/{}",dia,mes,ano);
    println!("-----------------------------");
    }
        println!("A função print_today()  exibe data de hoje");
        println!("A função receive_func() recebe var function do tipo fn()");
        println!("let var_pointer: fn() = print_today;");
        println!("essa variavel recebe um ponteiro para print_today()");
        println!("receive_func(var_pointer) chama a func com o ponteiro");
fn receive_func(function: fn()) {
    function()
    }

let var_pointer: fn() = print_today;
receive_func(var_pointer);    
    
}    
    