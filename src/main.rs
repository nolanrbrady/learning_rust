use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Welcome to the Guessing Game");

    // Generate random number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {

        println!("Please input your guess: ");

        let mut guess = String::new();


        // Read number from terminal
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");


        let guess: u32 = guess.trim().parse().expect("Expected a number");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess was to low"),
            Ordering::Greater => println!("Your guess was to high"),
            Ordering::Equal => println!("You win!"),
        }

    }
}

