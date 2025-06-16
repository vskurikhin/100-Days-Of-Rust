use std::io;
use anyhow::Result;
const DAYS_IN_YEAR: u32 = 365;

fn main() {
    loop {
        println!("Please input your age in years!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read age. Make you an integer is used.");

        // Конвертировать предположение в возраст в днях
        match convert_years_to_days(guess.trim()) {
            Ok(age) =>  println!("Silly rabbit you are {} days old not {} !", age, guess.trim_end()),
            Err(e) => println!("Error converting string:, {}", e),
        }
    }
}

//  Простая функция для преобразования года в дни
fn convert_years_to_days(year: &str) -> Result<String> {
    // Преобразует строку в u32
    let age_in_days: u32 = year.parse()?;

    // если ошибок нет, умножает его на константу дней в году и преобразует обратно в строку и возвращает её
    Ok((age_in_days * DAYS_IN_YEAR).to_string())
}