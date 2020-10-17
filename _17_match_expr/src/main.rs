fn main() {
    let boolean = false;

    match boolean {
        false => println!("False"),
        true => println!("True"),
    }

    let number = 13;
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is prime"),
        13..=19 => println!("Teen"),
        _ => println!("Number not covered"),
    }

    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} => {}", boolean, binary);
}
