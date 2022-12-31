use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

pub fn experiment() {

    let mut rng = thread_rng();
    let secret_number = rng.gen_range(-100..100);

    println!("Guess the number!\n");
    

    let mut guessed_number: bool = false;
    while !guessed_number {
        
        println!("Please input your guess.");

        let mut guess_str: String = String::new();

        io::stdin()
            .read_line(&mut guess_str)
            .unwrap();

        let guess: i32 = guess_str.trim().parse().unwrap();
        
        println!("\n\nYou guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => guessed_number = true,
        }
        println!("\n----------------------------------------------------------------------------------\n\n");
    }
    println!("You win!");
}