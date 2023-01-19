use std::io::stdin;

pub fn get_char_input() -> char {
    let mut input = String::new();
    while input == "" {
        stdin().read_line(&mut input).expect("Failed to read input");
    }
    input.chars().next().unwrap()
}
