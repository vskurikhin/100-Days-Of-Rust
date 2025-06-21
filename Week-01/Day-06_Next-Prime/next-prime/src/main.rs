use rand::Rng;
use std::io;
// Библиотека io является частью стандартной библиотеки, известной как std
use std::cmp::Ordering;
use next_prime::next_prime;

fn main() {
    loop {
        println!("Please input your guess."); // это макрос, который выводит строку на экран

        let mut guess = String::new(); // создаём переменную для хранения пользовательского ввода

        match io::stdin().read_line(&mut guess) {
            // перечисления и сопоставление с образцом
            // выражение match состоит из веток (arms)
            Ok(n) => {
                println!("{n} bytes read");
                println!("You entered::{guess}");
            }
            Err(error) => panic!("Failed to read line error: {error}"),
        }

        // затенение позволяет нам повторно использовать имя переменной guess
        // метод trim на экземпляре String удалит любые пробельные символы в начале и конце строки
        // метод trim убирает \n или \r\n
        // метод parse строк преобразует строку в другой тип
        // здесь мы используем его для преобразования строки в число
        let guess: u128 = match guess.trim().parse() {
            // перечисления и сопоставление с образцом
            // выражение match состоит из веток (arms)
            Ok(num) => num,
            Err(error) => { 
                println!("Failed to read line error: {error}");
                continue
            },
        };

        println!("You guessed: {guess}");
        println!("Next prime number: {}", next_prime(guess));
    }
}
