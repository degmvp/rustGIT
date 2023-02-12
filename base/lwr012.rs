// JP,2020/10/16 formato yyyy-mm-dd                                                                   -->
// Documentação de código RUST                                                                     -->
////////////////////////////////////////
//Pattern matching- using tupla!
//valor posicional no vetor                                                                                                     -->
////////////////////////////////////////
pub fn lwr12() {
    println!("Pattern matching 2020/10/07");
    println!("----------------------------------------");
    println!("Ignoring Parts of a Value with a Nested _");
    println!("----------------------------------------");
    
             let nv = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024);
    
        
            match nv {
    
                (_, _, _, _, _, _, _, v8, v9, v10) => {
    
                    println!("valor posicional no vetor ou tupla: {}, {}, {}", v8,v9,v10)
                }
            }
    
        println!("----------------------------------------");
        println!("Matching Literals ");
        println!("----------------------------------------");
    
        let x = 3;
      
            match x {
                1 => println!("valor de x {}", x),
                2 => println!("valor de x {}", x),
                3 => println!("valor de x {}", x),
                _ => println!("valor de x {}", x),
            }
    
    }
    