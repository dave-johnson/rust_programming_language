use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    println!("Guess the number {}", secret_number);
    println!("Enter your guess", );

    loop {
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("failed");
        
        println!("you guessed {}", guess);
        let guess: u32 =  match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "too low".red()),
            Ordering::Greater => println!("{}", "too big".red()),
            Ordering::Equal => {
                println!("{}", "nice job".green());
                break;
            },
        }
    }
}
