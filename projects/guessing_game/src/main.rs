use std::io; // input / output library
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    loop {
    println!("Please input your guess.");

    let mut guess = String::new();  // variable

    io::stdin()
    .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
}