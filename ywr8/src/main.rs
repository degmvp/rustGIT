mod front_of_house {
    pub mod hosting {
        pub fn greet() {
            println!("Welcome to rust module!");
        }
    }

   pub  mod serving {
        pub fn take_order() {
            println!("Using nested modules!");
        }
    }
}

fn main() {
    front_of_house::hosting::greet();
    front_of_house::serving::take_order();


    let mut x = 5; // x owns the value 5
    let y = &x; // y borrows the value 5 from x

    println!("The value of x is: {}", x); // Prints 5
    println!("The value of y is: {}", y); // Prints 5

    x = 10; // x takes ownership of the value 5 and drops y

    println!("The value of x is: {}", x); // Prints 10
    //println!("The value of y is: {}", y); // This will cause an error because y is no longer valid

}
