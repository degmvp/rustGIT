// JP,2020/09/22 formato yyyy-mm-dd                                   
// Documentação de código RUST     
                                
////////////////////////////////////////                                       
//fixed-size array   
//dynamic array (vector)
//slice for vectors  
//use {:?} to print debug-style    
//tuple is a fixed-size set of values  
//Indexing  loop for                      
//////////////////////////////////////// 

pub fn lwr6() {
    // A fixed-size array
        let itens: [i32; 4] = [1, 2, 3, 4];
        println!("fixed-size array {:?}",itens);
        // A dynamic array (vector)
        let mut vector: Vec<i32> = vec![1, 2, 3, 4];
        vector.push(5);
        println!("dynamic array {:?}",vector);
        
        // A slice – an immutable view into a vector or array
        // This is much like a string slice, but for vectors
        let slice: &[i32] = &vector;
    
        // Use `{:?}` to print something debug-style
        println!("a slice for vectors {:?} {:?}", vector, slice); // [1, 2, 3, 4, 5] [1, 2, 3, 4, 5]
    
        // Tuples //
    
        // A tuple is a fixed-size set of values of possibly different types
        let x: (i32, &str, f64) = (1, "hello", 3.4);
        println!("valores em uma tupla x {:?}", x);
        
        let (a, b, c) = x;
        println!("tupla x {} {} {}", a, b, c);
        
        // Indexing
        println!("indexando valores de x.0 {}", x.0);
        println!("indexando valores de x.1 {}", x.1);
        println!("indexando valores de x.2 {}", x.2);
        
        // Fixed-size array (type signature is superfluous)
        let xs: [i32; 5] = [1, 2, 3, 4, 5];
    
         // Indexing starts at 0
        println!("first element of the array: {}", xs[0]);
        println!("second element of the array: {}", xs[1]);
    
        // `len` returns the size of the array
        println!("array size: {}", xs.len());
        
        
        fn sum_odd_numbers(up_to: u32) -> u32 {
            let mut acc = 0;
            for i in 0..up_to {
                // Notice that the return type of this match expression must be u32
                // because of the type of the "addition" variable.
                let addition: u32 = match i%2 == 1 {
                    // The "i" variable is of type u32, which is perfectly fine.
                    true => i,
                    // On the other hand, the "continue" expression does not return
                    // u32, but it is still fine, because it never returns and therefore
                    // does not violate the type requirements of the match expression.
                    false => continue,
                };
                acc += addition;
            }
            acc
        }
        println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
        
        
        
    }