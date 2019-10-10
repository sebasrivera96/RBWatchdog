extern crate reqwest;
#[macro_use]
extern crate rouille;

use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use std::sync::Mutex;

// static BASE_PATH: String = "https://review-board.natinst.com/api/review-requests/";

fn get_reviews_last_update(client_info: &Client) -> Value {
    let base_path = String::from("https://review-board.natinst.com/api/review-requests/");
    let last_update_method = String::from("/last-update/");

    let request_url = base_path + &client_info.review_id + &last_update_method;

    let client = reqwest::blocking::Client::new();
    let last_update_json: Value = client.get(&request_url)
        .header(AUTHORIZATION, "token a2fc903eeaeadf3cbc87cbbdc03ef2d02241217f")
        .send().unwrap()
        .json().unwrap();
    return last_update_json;
}

struct Client {
    token: String,
    review_id: String,
    new_update_available: bool
}

fn main() {
    // Get the information from the Review
    // let review_id: String = "311878".to_string();
    // let last_update_json: Value = get_reviews_last_update(&review_id);
    // println!("timestamp => {:?}", last_update_json["last_update"]["timestamp"]);

    // HTTP Server begins & Create a New Client
    println!("Now listening on localhost:8000"); 
    let sinlge_client = Client{
        token: String::from(""),
        review_id: String::from(""),
        new_update_available: false
    };
    let request_client = Mutex::new(sinlge_client);

    rouille::start_server("localhost:8000", move |request| {
        router!(request,
            (GET) (/set_token/{new_token: String}) => {
                (*request_client.lock().unwrap()).token = new_token;
                println!("==> NEW TOKEN: {}", (*request_client.lock().unwrap()).token);
                rouille::Response::text("TOKEN UPDATED!")
            },
            (GET) (/add_review_id/{new_review_id: String}) => {
                (*request_client.lock().unwrap()).review_id = new_review_id;
                println!("==> NEW REVIEW: {}", (*request_client.lock().unwrap()).review_id);
                rouille::Response::text("NEW REVIEW UPDATED!")
            },
            (GET) (/get_updates/) => {
                let last_update_json: Value = get_reviews_last_update(&(*request_client.lock().unwrap()));
                println!("timestamp => {:?}", last_update_json["last_update"]["timestamp"]);
                rouille::Response::text("@TODO: Return a JSON with the info")
            },

            
            // The code block is called if none of the other blocks matches the request.
            // We return an empty response with a 404 status code.
            _ => rouille::Response::empty_404()
        )

    });

}

