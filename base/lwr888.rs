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
use crate::base::lwr012::lwr12;
use crate::base::lwr013::lwr13;
use crate::base::lwr014::lwr14;
use crate::base::lwr015::lwr15;
use crate::base::lwr016::lwr16;
use crate::base::lwr999::lwrc99;

fn read_input() -> Result<u32,ParseIntError> {
    print!("Numero da Lwr: ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Error ao ler do teclado");
    let input = input.trim();
    let edad: u32 = u32::from_str(&input)?;
    Ok(edad)
 }


pub fn lwrc88() {
    let edad;
    loop {
        if let Ok(e) = read_input(){
            edad = e;
            print!("edad e {} {}\n",edad,e);
            break;
        }else{
            println!("Lwr ? Digite un número!");            
        }
    }
    match edad {
        1 => lwr1(),
        2 => lwr2(),
        3 => lwr3(),
        4 => lwr4(),
        5 => lwr5(),   
        6 => lwr6(),
        8 => lwr8(),
        9 => lwr9(),
       10 => lwrc99(),
       11 => lwr11(),
       12 => lwr12(),
       13 => lwr13(),
       14 => lwr14(),
       15 => lwr15(),
       16 => lwr16(),
        _ => println!("Pattern match -No Call lwr!"),
    }
}   

  

