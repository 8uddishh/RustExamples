use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Red")];
    let mut points: Vec<u32> = Vec::new();
    points.push(10);
    points.push(20);
    points.push(30);

    let mut scores: HashMap<String, u32> = teams.into_iter().zip(points.into_iter()).collect();

    scores.insert(String::from("Green"), 10);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(90);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";
    let mut word_count_map: HashMap<String, u32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count_map.entry(String::from(word)).or_insert(0);
        *count += 1;
    }

    for (key, value) in &word_count_map {
        println!("{}: {}", key, value);
    }
}
