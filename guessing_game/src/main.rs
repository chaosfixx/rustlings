use std::io;
use rand::Rng;

fn main(){
    println!("guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("the secret number is: {}", secret_number);
    println!("type your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("you guessed: {}", guess);

}