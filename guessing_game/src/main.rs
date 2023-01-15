use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    loop {
        println!("Guess the number! (1..=100)");
        println!("Please enter your guess.");

        let secret_number = rand::thread_rng().gen_range(1..=100);
        let mut guesses = 0;

        loop {
            guesses += 1;
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read guess");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("You guessed {guess}");
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small"),
                Ordering::Greater => println!("Too big"),
                Ordering::Equal => {
                    println!("You win after {guesses} guesses");
                    break;
                }
            }
        }
        println!("Play again? Y/N");
        let mut entry = String::new();
        io::stdin().read_line(&mut entry).expect("Failed to read entry");
        match entry.trim().eq("Y") {
            true => continue,
            false => break,
        };
    }
}
