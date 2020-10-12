fn main() {
    let str1 = String::from("Learning Rust");

    let (s, length) = calculate_length(str1);
    println!("Regular move '{}' length {}", s, length);

    let str2 = String::from("Learning Rust Ref");
    let length1 = calculate_ref_length(& str2);
    println!("Reference move '{}' length {}", str2, length1);

    let mut str3 = String::from("hello");
    append_word(&mut str3);
    println!("Mutable refernces {}", str3);

    let mut str4 = String::from("sbaliah");
    let r1 = &str4;
    let r1_length = r1.len();
    let r2 = &str4;

    println!("{} and {}, {}", r1, r2, r1_length);
    let r3 = &mut str4;
    r3.push_str(" is killing it");
    println!("post mutation {}", r3);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_ref_length(s: & String) -> usize {
    s.len()
}

fn append_word(s: &mut String) {
    s.push_str(", world");
}