// In main.rs
mod my_module {
    pub fn msg() {
        println!("mod1 - Basic Module Structure: from my_module!");
        println!("my_module - Called from main() from msg() !");
    }
}

fn main() {
    my_module::msg(); // Calling the function from the module
}