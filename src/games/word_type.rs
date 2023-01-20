use std::{
    io::{stdin, stdout, Write},
    time::Duration,
};

use crate::{util::TimeCounter, Play};

pub struct WordType;

impl Play for WordType {
    fn name(&self) -> &'static str {
        "Word Type"
    }

    fn start(&mut self) {
        let mut timer = TimeCounter::new();

        const COUNT: usize = 10;
        let words: [&str; COUNT] = [
            eff_wordlist::large::random_word(),
            eff_wordlist::large::random_word(),
            eff_wordlist::large::random_word(),
            eff_wordlist::large::random_word(),
            eff_wordlist::large::random_word(),
            eff_wordlist::large::random_word(),
            eff_wordlist::large::random_word(),
            eff_wordlist::large::random_word(),
            eff_wordlist::large::random_word(),
            eff_wordlist::large::random_word(),
        ];

        println!("Type the following {COUNT} words then enter\n");
        for word in words {
            print!("{word} ");
        }
        println!();
        std::thread::sleep(Duration::from_secs(2));

        print!("\nType now: ");
        stdout().flush().expect("Failed to flush");

        let mut typed = String::new();
        timer.start();
        stdin().read_line(&mut typed).expect("Failed to read input");
        timer.stop();

        let correct_count: u8 = words
            .iter()
            .map(|word| if typed.contains(word) { 1 } else { 0 })
            .sum();

        let wpm = COUNT as f32 / (timer.duration().unwrap().as_secs_f32() / 60.0);

        println!(
            "\nYou typed correctly {} out of {} words\nWPM: {}\n",
            correct_count, COUNT, wpm
        );
    }
}
