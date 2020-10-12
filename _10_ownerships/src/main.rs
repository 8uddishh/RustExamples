fn main() {
    let s1 = String::from("hello");
    let mut s2 = s1;

    s2.push_str(", world");
    println!("s1 will throw error as only one owner can be there for a value");
    println!("value of s2 {}", s2);

    println!("Heap example - Clone");
    let c1 = String::from("hello");
    let mut c2 = c1.clone();
    c2.push_str(", world");
    println!("clone c1 = {}, c2 = {}", c1, c2);

    println!("Stack example");
    let x = 5;
    let mut y = x;
    y += 10;
    println!("x = {}, y = {}", x, y);
}
