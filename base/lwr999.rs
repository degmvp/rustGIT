use std::io;
use std::io::Write;
use std::str::FromStr;
use std::num::ParseIntError;
use crate::base::lwr001::lwr1;
use crate::base::lwr002::lwr2;
use crate::base::lwr003::lwr3;
use crate::base::lwr004::lwr4;
use crate::base::lwr005::lwr5;
use crate::base::lwr006::lwr6;
use crate::base::lwr008::lwr8;
use crate::base::lwr009::lwr9;
use crate::base::lwr011::lwr11;

fn read_input() -> Result<u32,ParseIntError> {
    print!("Numero da rotina: ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Error ao ler do teclado");
    let input = input.trim();
    let edad: u32 = u32::from_str(&input)?;
    Ok(edad)
}

pub fn lwrc99() {
    let edad;
    loop {
        if let Ok(e) = read_input(){
            edad = e;
            print!("edad e {} {}\n",edad,e);

            break;
        }else{
            println!("Digite un número, por favor");
        }
    }
    if edad == 1 {
        println!("Call lwr1()");
        lwr1();
    }else
   
    if edad == 2 {    
        println!("Call lwr2()");
        lwr2();
    }else
    
    if edad == 3 {
        println!("Call lwr3()");
        lwr3();
    }else
       
     if edad == 4 {    
        println!("Call lwr4()");
        lwr4();
    }else  

    if edad == 5 {    
        println!("Call lwr5()");
        lwr5();
    }else
 
    if edad == 6 {    
        println!("Call lwr6()");
        lwr6();
    }else

    if edad == 8 {    
        println!("Call lwr8()");
        lwr8();
    }else

    if edad == 9 {    
        println!("Call lwr09()");
        lwr9();
    }
    else{  
        println!("Call lwr11");
        lwr11();
    }
 }

  

