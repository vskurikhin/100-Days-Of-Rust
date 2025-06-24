use std::io;
use is_johnny_making_progress;
use is_johnny_making_progress::{parse_line_to_result_vec, progress_days};

fn main() {
    println!("Please input number separated by spaces!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Could not read. Make you an integer is used.");
    match parse_line_to_result_vec(&guess) {
        Ok(vec) => println!("Progress Days: {}", progress_days(vec)),
        Err(e) => println!("Error converting string:, {}", e),
    }
}
