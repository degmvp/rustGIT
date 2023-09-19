// In main.rs
mod outer_module {
    pub mod inner_module {
        pub fn msg() {
            println!("mod2 - Nested Modules: from inner_module!");
            println!("outer_module::inner_module::msg() from main() !");
        }
    }
}

fn main() {
    outer_module::inner_module::msg(); // Calling the function from the nested module
}

