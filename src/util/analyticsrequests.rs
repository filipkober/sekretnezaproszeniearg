use std::collections::HashMap;

use json::object;

pub fn new_user(entropy: String, answers: Vec<String>) {
    let client = reqwest::blocking::Client::new();
    let mut map = HashMap::new();
    map.insert("entropy", entropy);
    map.insert("favColor", answers[0].clone());
    map.insert("favAnimal", answers[1].clone());
    map.insert("favCrisis", answers[2].clone());
    map.insert("name", answers[3].clone());
    map.insert("whatWouldYouBe", answers[4].clone());
    map.insert("deepestSecret", answers[5].clone());

    client.post("https://entr0py.nigdit.men/entropy/user")
    .json(&map)
    .send().unwrap();
}
pub fn update_level(entropy: String, time_taken: f32, hints_used: usize, completed: bool, level_index: usize) {
    let client = reqwest::blocking::Client::new();
    let json = json::stringify(object! {
        entropy: entropy,
        timeTaken: time_taken,
        hintsUsed: hints_used,
        completed: completed,
        level_index: level_index
    });
    client.put("https://entr0py.nigdit.men/entropy/level")
    .body(json)
    .header(reqwest::header::CONTENT_TYPE, "application/json")
    .send().unwrap();
}