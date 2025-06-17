use anyhow::{anyhow, Result};
use std::io;

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
