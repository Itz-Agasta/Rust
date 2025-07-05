use rand::Rng;
use std::cmp::Ordering;
use std::io;
use text_to_ascii_art::to_art;
fn main() {
    // Print ASCII art for "guessing game"
    match to_art("guessing game".to_string(), "block", 0, 1, 0) {
        Ok(art) => println!("{}", art),
        Err(e) => println!("ASCII art error: {}", e),
    }

    let num = rand::rng().random_range(1..=100);

    // User input
    loop {
        println!("choose a number:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to get the number");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("you guessed {guess}");

        // Cmp the num and result out
        match guess.cmp(&num) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            }
        }
    }
}
