// JP,2020/10/16 formato yyyy-mm-dd                                                                   -->
// Documentação de código RUST                                                                     -->
////////////////////////////////////////
//fixed size array - using macro vec!
//loop  - for - while                                                                                                     -->
////////////////////////////////////////
pub fn lwr9() {
println!("A fixed size array");
let xs: [u32; 12] = [4001,4002,4003,4004,4005,4006,4007,4008,4009,4010,4011,4012];
println!("fixed size array {:?}", xs);
println!("Using macro vec! create a fixed size array");

let v=vec![5001,5002,5003,5004,5005,5006,5007,5008,5009,5010,5011,5012];
println!("fixed size array {:?}\n", v);

for i in xs.iter() {
    println!("Using for The value array xs {}",i);
    }
println!("---------------------------------------------------------");
let acc = -1;
if acc < 0 {
    println!("printing {} acc is negative\n",acc);
    }
    else if acc > 0 {
    println!("printing {} acc is positive\n",acc);
    }
    else {
    println!("printing {} acc is zero\n",acc);
    }
println!("---------------------------------------------------------");

let mut i=0;
while i < 12 {
    println!("Using while The value array xs {}",xs[i]);
    i=i+1;
}
println!("---------------------------------------------------------");
let mut i=0;
loop {
      println!("Using loop the value array xs: {}", xs[i]);
      i=i+1;
      if i == 12 {
          break;
      }
}
}