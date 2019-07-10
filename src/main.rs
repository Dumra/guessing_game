use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess from 1 to 100 range!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Incorrect input. Please type a digit!");
                continue
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Nah! Too small! Try again!"),
            Ordering::Greater => println!("Nah! Too big! Try again!"),
            Ordering::Equal => {
                println!("You win!!!!");
                break;
            },
        }
    }
}
