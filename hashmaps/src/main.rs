use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Team 1"), String::from("Team 2")];
    let scores = vec![1, 2];
    let mut scores: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();

    println!("{}", match scores.get(&String::from("Team 3")) {
        Some(score) => score.to_string(),
        None => String::from("nothing")
    });

    for (team, score) in &scores {
        println!("{} has {}", team, score);
    }

    println!("score for Orange is {}", scores.entry(&String::from("Orange")).or_insert(&3));
}
