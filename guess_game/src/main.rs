use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your number:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Not a number!");
        let guess : u32 = guess.trim().parse().expect("Not a number! Enter again.");
        
        println!("Guessed number:{guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small.\n"),
            Ordering::Greater => println!("Too High.\n"),
            Ordering::Equal => {
                println!("You Win!\n");
                break;
            }
                
        }
    }
}
