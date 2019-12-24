use std::io;
use std::cmp::Ordering;
use rand::Rng;
mod fizzbuzz;

// struct Programs {
//     FizzBuzz,
//     GuessingGame,
// }

fn main() {

    println!("Please select a program to run");
    println!("Please select one of the following: ");
    println!("1 for Guessing Game");
    println!("2 for FizzBuzz");

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            1 => guessing_game(),
            2 => fizzbuzz::fizz_buzz(),
            _ => println!("Not an option, try again")
        }

    }
}



fn guessing_game() {
    println!("Welcome to the Guessing Game");

    // Generate random number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {

        println!("Please input your guess: ");

        let mut guess = String::new();


        // Read number from terminal
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess was to low"),
            Ordering::Greater => println!("Your guess was to high"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }
}
