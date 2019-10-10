extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rouille;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use std::sync::Mutex;

// DEFAULT_TOKEN = a2fc903eeaeadf3cbc87cbbdc03ef2d02241217f
// static BASE_PATH: String = "https://review-board.natinst.com/api/review-requests/";

#[derive(Serialize)]
struct Client {
    token: String,
    review_id: String,
    status: String,
    timestamp: String,
    username: String,
    summary: String,
}

impl Client {
    fn get_reviews_last_update(&mut self) -> bool {
        // => Local Vars
        let base_path = String::from("https://review-board.natinst.com/api/review-requests/");
        let last_update_method = String::from("/last-update/");
        let request_url = base_path + &self.review_id + &last_update_method;

        // => Call Review Board API
        let client = reqwest::blocking::Client::new();
        let last_update_json: Value = client.get(&request_url)
            .header(AUTHORIZATION, "token ".to_owned() + &self.token)
            .send().unwrap()
            .json().unwrap();
        
        // => Update info, IF new info available
        if last_update_json["last_update"]["timestamp"].to_string() != self.timestamp {
            println!("==> Timestamp is new!");
            self.status = last_update_json["stat"].to_string();
            self.timestamp = last_update_json["last_update"]["timestamp"].to_string();
            self.username = last_update_json["last_update"]["user"]["username"].to_string();
            self.summary = last_update_json["last_update"]["summary"].to_string();
            return true;
        }

        // => No update available
        return false;
}
}

fn main() {
    // HTTP Server begins & Create a New Client
    println!("Now listening on localhost:8000"); 
    let single_client = Client{
        token: String::from(""),
        review_id: String::from(""),
        status: String::from(""),
        timestamp: String::from(""),
        username: String::from(""),
        summary: String::from(""),
    };
    let request_client = Mutex::new(single_client);

    rouille::start_server("0.0.0.0:8000", move |request| {
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
                let new_update_available: bool = (*request_client.lock().unwrap()).get_reviews_last_update();             
                if new_update_available {
                    return rouille::Response::json(&(*request_client.lock().unwrap()));
                }
                rouille::Response::text("NO UPDATE AVAILABLE.")
            },

            // The code block is called if none of the other blocks matches the request.
            // We return an empty response with a 404 status code.
            _ => rouille::Response::empty_404()
        )
    });
}