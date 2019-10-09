extern crate reqwest;

use reqwest::header::AUTHORIZATION;
use serde_json::Value;


fn get_reviews_last_update(review_id: &String) -> Value {
    let base_path = String::from("https://review-board.natinst.com/api/review-requests/");
    let last_update_method = String::from("/last-update/");

    let request_url = base_path + review_id + &last_update_method;

    let client = reqwest::blocking::Client::new();
    let last_update_json: Value = client.get(&request_url)
        .header(AUTHORIZATION, "token a2fc903eeaeadf3cbc87cbbdc03ef2d02241217f")
        .send().unwrap()
        .json().unwrap();
    return last_update_json;
}

fn main() {
    // Get the information from the Review
    let review_id: String = "311878".to_string();
    let last_update_json: Value = get_reviews_last_update(&review_id);

    println!("timestamp => {:?}", last_update_json["last_update"]["timestamp"]);
    
}

