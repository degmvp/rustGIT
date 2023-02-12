    let x = 5;
    let z = x + 13;
    
let y = {
    let x = 2;
    x + 2
};

println!("X is {} and Y is {}", x,y);

let w = {
    let x = 12;
    x + 2
};

let total = { 
x + y + w + z
};
println!("X is {} and W is {}", x,w);

println!("X is {} and z is {}", x,z);

println!("X,Y,W,Z,total {}/{}/{}/{}/{}", x,y,w,z,total);

println!("number is {}", number(1));
println!("este numero é 2 elevado a 64");
fn number(n: i128) -> i128 {
    n + 18446744073709551615
}
}