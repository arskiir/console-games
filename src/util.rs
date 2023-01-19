use std::io::stdin;

pub fn get_char_input() -> char {
    let mut input = String::new();
    while input == "" {
        stdin().read_line(&mut input).expect("Failed to read input");
    }
    // println!();
    input.chars().next().unwrap()
}

pub fn get_unique_vec<T>(iter: impl Iterator<Item = T>) -> Vec<T>
where
    T: std::cmp::PartialEq,
{
    let mut unique_vec: Vec<T> = Vec::new();
    for item in iter {
        if !unique_vec.contains(&item) {
            unique_vec.push(item);
        }
    }
    unique_vec
}
