// JP,2020/10/16 formato yyyy-mm-dd                                                                   -->
// Documentação de código RUST                                                                     -->
////////////////////////////////////////
//pattern match - samples
//Ignoring Parts of a Value with a Nested _ 
//Matching Literals
//Matching Named Variables 
//Multiple Patterns     
//Matching Ranges of Values with ..=    
//Adding a match guard to a pattern
//Matching values in a tupla  
//Extra Conditionals with Match Guards
//Using a match guard to test for equality                                                                          -->
////////////////////////////////////////
pub fn lwr11() {

    println!("----------------------------------------");
    println!("PATTERN MATCHING SAMPLES");
    println!("----------------------------------------");
    println!("----------------------------------------");
    println!("Ignoring Parts of a Value with a Nested _");
    println!("----------------------------------------");
         let numbers = (2, 4, 8, 16, 32);
    
        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {}, {}, {}", first, third, fifth)
            }
        }
    
    //-------------------//
    // Matching Literals    
    //-------------------//
    println!("----------------------------------------");
    println!("Matching Literals ");
    println!("----------------------------------------");

    let x = 5;
    
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("value zero"),
        }
    
    //-------------------------//
    //Matching Named Variables 
    //-------------------------//
    println!("----------------------------------------");
    println!("Matching Named Variables  ");
    println!("----------------------------------------"); 
        let x = Some(5);
        let y = 10;
    
        match x {
            Some(5) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }
    
        println!("at the end: x = {:?}, y = {:?}", x, y);
      
    //-------------------------//
    //Multiple Patterns 
    //-------------------------//
    println!("----------------------------------------");
    println!("Multiple Patterns   ");
    println!("----------------------------------------");  
        let x = 2;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    
    
    //----------------------------------//
    //Matching Ranges of Values with ..=   
    //----------------------------------//
    println!("----------------------------------------");
    println!("Matching Ranges of Values with ..=");
    println!("----------------------------------------");  
        let x = 3;
        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }
     
    
    //----------------------------------//
    //Matching Ranges of Values with ..=   
    //----------------------------------// 
    println!("----------------------------------------");
    println!("Matching Ranges of Values with ..= ");
    println!("----------------------------------------");   
        let x = 'c';
        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    
    
    //----------------------------------//
    // Adding a match guard to a pattern
    //---------------------------------//
    println!("----------------------------------------");   
    println!("Adding a match guard to a pattern");   
    println!("----------------------------------------");   
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {}", n),
            _ => println!("Default case, x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {}", x, y);
    
    
    //---------------------------//
    //Matching values in a tupla
    //---------------------------//
    println!("----------------------------------------");  
    println!("Matching values in a tupla");  
    println!("----------------------------------------");  
        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, second, ..) => {
                println!("Some numbers: {} {}", first,second)
            },
        }
        
    
    //------------------------------------//
    //Extra Conditionals with Match Guards
    //------------------------------------//
    println!("----------------------------------------");  
    println!("Extra Conditionals with Match Guards");  
    println!("----------------------------------------");  
        let num = Some(7);
        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("great than five {}", x),
            None => (),
        }
    
    //---------------------------------//   
    //Adding a match guard to a pattern
    //---------------------------------// 
    println!("----------------------------------------");  
    println!("Adding a match guard to a pattern");  
    println!("----------------------------------------");  
       let x = Some(50);
       let y = 10;
        match x {
            Some(50) => println!("value 50"),
            Some(n) if n == y => println!("Matched, n = {}", n),
            _ => println!("Default case, x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {}", x, y);
     
    let x = 4;
        let y = true;
    
    //---------------------------------------//
    //Using a match guard to test for equality 
    //---------------------------------------//
    println!("----------------------------------------");  
    println!("Using a match guard to test for equality");  
    println!("----------------------------------------");  
        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
     }
     
     
     
     