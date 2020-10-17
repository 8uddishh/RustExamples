#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Texas,
    Other,
}

#[derive(Debug)]
enum Coin {
    Penny(u8),
    Nickel(u8),
    Dime(u8),
    Quarter(u8, UsState),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home {:?}!", home);
    println!("loopback {:?}!", loopback);

    let penny = Coin::Penny(1);
    let nickel = Coin::Nickel(5);
    let dime = Coin::Dime(10);
    let quarter_texas = Coin::Quarter(25, UsState::Texas);
    let quarter_alaska = Coin::Quarter(25, UsState::Alaska);

    println!("penny {:?}!", penny);
    println!("nickel {:?}!", nickel);
    println!("dime {:?}!", dime);
    println!("quarter texas {:?}!", quarter_texas);
    println!("quarter alaska {:?}!", quarter_alaska);
}
