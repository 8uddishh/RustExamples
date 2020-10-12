fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number divisible by 4");
    } else if number % 3 == 0 {
        println!("number divisible by 3");
    } else if number % 2 == 0 {
        println!("number divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let divisible_by_four = if number % 4 == 0 { "yes" } else { "no" };
    println!("divisible by 4 {}", divisible_by_four);
}
