extern crate rand;

use rand::Rng;
use std::io::stdin;

fn main() {
    println!("Guess a random number - ");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut to_print = String::new();

        println!("Please input your guess");
        stdin()
            .read_line(&mut to_print)
            .expect("Failed to read line");
        let to_print: u32 = to_print.trim().parse().expect("Please type a number");
        if to_print > secret_number {
            println!("Too big");
        } else if to_print < secret_number {
            println!("Too small");
        } else {
            println!("Your guess is correct");
            break;
        }
    }
}
