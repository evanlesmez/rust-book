use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess!");
        let mut guess = String::new(); // mut == "mutable. Not mutex. Literally able to change"

        io::stdin()
            .read_line(&mut guess) // Need &mut to update the variable
            .expect("Failed to read line"); // Result type has expect method. Causes program to crash
                                            // and show error

        let guess: u16 = match guess.trim().parse() {
            // result type is enum. match statement specifies how to look for enum case
            // handle
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
