use std::io::stdin;

pub fn get_char_input() -> Option<char> {
    let mut input = String::new();
    while input == "" {
        stdin().read_line(&mut input).expect("Failed to read input");
    }
    input.trim().chars().next()
}
