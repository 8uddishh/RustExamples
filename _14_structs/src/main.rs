struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("sbaliah@rust.com"),
        username: String::from("sbaliah"),
        sign_in_count: 100,
        active: true,
    };

    user1.email = String::from("sbaliah@rust123.com");

    println!("username {}", user1.username);
    println!("email {}", user1.email);
    println!("sign_in_count {}", user1.sign_in_count);
    println!("active {}", user1.active);

    let user2 = build_user(String::from("sbaliah2"), String::from("sbaliah2@rust.com"));
    println!("username {}", user2.username);
    println!("email {}", user2.email);
    println!("sign_in_count {}", user2.sign_in_count);
    println!("active {}", user2.active);

    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    let Color(r, g, b) = black;

    println!("color r{} g{} b{}", r, g, b);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Area rect1 {}", rect1.area());
}

fn build_user(username: String, email: String) -> User {
    User {
        email,
        username,
        sign_in_count: 0,
        active: true,
    }
}
