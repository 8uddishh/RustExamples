use std::io::stdin;

fn main() {
    println!("what do you want to print?");

    let mut to_print = String::new();

    stdin()
        .read_line(&mut to_print)
        .expect("Faield to read line");

    println!("User input to print {}", to_print);
}
