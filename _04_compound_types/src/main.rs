fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, _y, _z) = tup;

    println!("The value y is {}", _y);
    println!("The value tup0 is {}", tup.0);

    let a = [1, 2, 3, 4, 5];

    for number in a.iter() {
        println!("The value {}", number);
    }
}