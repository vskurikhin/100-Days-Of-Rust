use std::io;
use pair_of_socks::{sock_pairs};

fn main() {
    println!("Please input caps letters!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Could not read. Make you an caps letter is used.");
    println!("Sock Pairs: {}", sock_pairs(&guess));
}
