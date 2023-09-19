// In main.rs
mod my_module; // Import the external module file

fn main() {
    println!("Calling the function from the external module file!");
    my_module::msg(); 
}
