use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("IT'S CASINO TIME!");
    let secret_number = rand::thread_rng().gen_range(1..=50);
    loop {
        println!("Make your bet.");
        let mut bet = String::new();

        io::stdin()
            .read_line(&mut bet)
            .expect("The fuck was that bullshit?");
        let bet: u32 = match bet.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your bet: {bet}");

        match bet.cmp(&secret_number) {
            Ordering::Less => println!("You lose!"),
            Ordering::Greater => println!("You lose!"),
            Ordering::Equal => {
                 println!("You win!");
                 break;
            }
        }
    }
}
