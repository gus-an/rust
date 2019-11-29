use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // create hash map using 'collect()' method, collecting the tuples
    // 'zip' creates tuples using vectors
    // HashMap<_, _> : it's possible to 'collect' into many different data structures,
    // and Rsut doesn't know which you want unless you specify
    // using '_' underscore, Rust can infer the types based on data in vectors
}
