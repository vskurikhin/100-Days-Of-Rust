use anyhow::{anyhow, Result};
use std::io;
use regex::Regex;

const NEMO: &str = "Nemo";

fn main() {
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read line.");

        match find_nemo(guess.trim()) {
            Ok(index) => println!("I found Nemo at {}!", index),
            Err(_) => println!("I can't find Nemo :("),
        }
    }
}

fn find_nemo(guess: &str) -> Result<u32> {
    let parts = guess.split(" ");
    let collection = parts.collect::<Vec<&str>>();

    for i in 0..collection.len() {
        if collection[i] == NEMO {
            return Ok((i + 1) as u32);
        }
    }
    Err(anyhow!(NEMO))
}

pub fn find_nemo_regex (buffer: &str) {
    let nemo_regex = Regex::new(r"\s*Nemo\s+").expect("Wrong regex pattern.");
    match nemo_regex.find(buffer.trim()) {
        Some(value) => {
            let slice: &str = &buffer[..value.start()+1];
            let split = slice.split(' ');
            let place_of_nemo: usize = split.count();
            println!("Found Nemo at {place_of_nemo}!");
        },
        None => println!("I can't find Nemo :("),
    }
}
