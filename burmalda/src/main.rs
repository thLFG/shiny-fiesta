use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("IT'S CASINO TIME!");
    let secret_number = rand::thread_rng().gen_range(1..=50);
    loop {
        println!("Make your bet.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You lose!"),
            Ordering::Greater => println!("You lose!"),
            Ordering::Equal => {
                 println!("You win!");
                 break;
            }
        }
    }
}
