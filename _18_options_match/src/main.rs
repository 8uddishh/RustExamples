use std::io::stdin;

fn main() {
    let mut role_id = String::new();
    println!("Enter the role id ->");
    stdin()
        .read_line(&mut role_id)
        .expect("Failed to read line");
    let role_id: u32 = role_id.trim().parse().expect("Please type a number");

    match get_roles(&role_id) {
        Some(rolename) => println!("{} belongs to {}", role_id, rolename),
        None => println!("No role definition found for {}", role_id),
    }
}

fn get_roles(&role_id: &u32) -> Option<String> {
    match role_id {
        1..=3 => Some("Executive role".to_string()),
        4..=18 => Some("Technician role".to_string()),
        19..=24 => Some("System role".to_string()),
        _ => None,
    }
}
