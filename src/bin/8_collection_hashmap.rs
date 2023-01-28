use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let s = String::from("Blue");
    scores.entry(&s).or_insert(&50);
    println!("{:?}", scores);

    let score = scores.get(&String::from("Blue"));
    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
