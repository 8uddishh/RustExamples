extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("Guess a random number - ");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // loop {
    //     let mut to_print = String::new();

    //     println!("Please input your guess");
    //     stdin()
    //         .read_line(&mut to_print)
    //         .expect("Failed to read line");
    //     let to_print: u32 = to_print.trim().parse().expect("Please type a number");
    //     if to_print > secret_number {
    //         println!("Too big");
    //     } else if to_print < secret_number {
    //         println!("Too small");
    //     } else {
    //         println!("Your guess is correct");
    //         break;
    //     }
    // }

    loop {
        let mut guess = String::new();

        println!("Please input your guess");
        stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("Please type a number");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Your guess is correct");
                break;
            }
        }
    }
}
