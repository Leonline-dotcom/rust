
use std::{cmp::Ordering, io};
use rand::Rng;
use colored::*;
fn main() {
    println!("I'm thinking of a number, guess what it is!");
    let secret_number = rand::thread_rng().gen_range(1,100);
    println!("the secret number is {}", secret_number);


    loop{
        println!("Please enter your guess:");
        let mut guess = String::new();
    
        // this is how you get inputs
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
    
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => { 
                    println!("{}","Type a number or I'll eat your pants".red());
                    continue;
                },
            };


        println!("you guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".blue()),
            Ordering::Equal => {println!("{}","You got it!".green()); 
                                         break},
            Ordering::Greater => println!("{}","Too big".blue()),
        }
    }
    

}
