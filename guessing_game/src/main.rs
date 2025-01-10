use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101); // or gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the number");


        let guess:i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_e) => {eprintln!("Please type a number! {}", _e);continue;}
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {println!("{}","You win! 🎉".green());break;},
        }
    }
    
    
}
