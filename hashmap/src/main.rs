use std::collections::HashMap;

fn main() {
     let mut scores = HashMap::new();

     scores.insert(String::from("Blue"),10);
     scores.insert(String::from("Yellow"), 50);
    println!("Hello, world!");

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
 