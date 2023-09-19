use std::fs;

fn main() {
    let results = fs::read_to_string("mytext.csv");

    let contents = match results {
        Ok(message) => message,
        Err(_) => todo!("***** Arquivo Inexistente *****"),
    };
        println!("{}", contents);
}