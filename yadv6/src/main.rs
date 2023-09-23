fn main() {
    let my_string = String::from("Curso da linguagem rust complewto!");
    let slice = &my_string[0..24]; // Slice from index 0 to 24 (inclusive)
    println!("Slice: {}", slice);
////////////////////////////////////////////////////
{
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4]; // Slice from index 1 to 3 (inclusive)
    println!("Slice: {:?}", slice);

}
/////////////////////////////////////////////////////
{
    let my_string = String::from("Rust is fun!");
    let slice = &my_string[0..4];
    println!("Slice: {}", slice);
    println!("Length of the slice: {}", slice.len());
    println!("Is the slice empty? {}", slice.is_empty());
}
/////////////////////////////////////////////////////////

{
    let my_string = String::from("Rust Programming");
    let word = first_word(&my_string);
    println!("The first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] // Return the whole string if no space is found
}
}
