#![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Directions {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

impl Directions {
    fn match_direction(&self) -> Keys {
        match *self {
            Directions::Up(_) => Keys::UpKey(String::from("Pressed w")),
            Directions::Down(_) => Keys::DownKey(String::from("Pressed w")),
            Directions::Left(_) => Keys::LeftKey(String::from("Pressed w")),
            Directions::Right(_) => Keys::RightKey(String::from("Pressed w")),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

fn main() {
    let u = Directions::Up(Point { x: 0, y: 1 });
    let k = u.match_direction();

    println!("{}", k.destruct());
}
