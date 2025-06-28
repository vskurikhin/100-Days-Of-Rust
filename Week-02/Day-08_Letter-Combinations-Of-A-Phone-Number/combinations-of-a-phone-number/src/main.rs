use std::io;
use combinations_of_a_phone_number::{all_combinations, KEYWORDS};

fn main() {
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read line.");

        let mut m: Vec<&[char]> = Vec::new();
        for c in guess.chars() {
            match KEYWORDS.get(&*String::from(c)) {
                Some(keywords) => m.push(keywords),
                None => {}
            }
        }
        let result = all_combinations(&m);
        for comb in result {
            println!("{}", comb);
        }
    }
}
