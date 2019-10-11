extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rouille;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use std::sync::Mutex;

// DEFAULT_TOKEN = a2fc903eeaeadf3cbc87cbbdc03ef2d02241217f
// static BASE_PATH: String = "https://review-board.natinst.com/api/review-requests/";

#[derive(Serialize, Debug)]
struct ReviewInfo {
    review_id: String,
    status: String,
    timestamp: String,
    username: String,
    summary: String,
    new_update_available: bool,
}

impl ReviewInfo {
    fn normal_constructor(review_id: String) -> ReviewInfo {
        ReviewInfo {
            review_id: review_id,
            status: String::from(""),
            timestamp: String::from(""),
            username: String::from(""),
            summary: String::from(""),
            new_update_available: false,
        }
    }

    fn update_review_info(&mut self, last_update: &Value) {
        self.status = last_update["stat"].to_string();
        self.timestamp = last_update["last_update"]["timestamp"].to_string();
        self.username = last_update["last_update"]["user"]["username"].to_string();
        self.summary = last_update["last_update"]["summary"].to_string();
        self.new_update_available = true;
        println!("==> New information available! {:#?}", self);
    }

    fn create_request_url(&self, base_path: &String, method: &String) -> String {
        let mut response = String::from("");
        response.push_str(base_path);
        response.push_str(&self.review_id);
        response.push_str(method);
        response
    }
}

#[derive(Serialize, Debug)]
struct Client {
    token: String,
    review_ids: Vec<ReviewInfo>,
}

impl Client {
    fn empty_client() -> Client {
        Client {
            token: String::from(""),
            review_ids: Vec::<ReviewInfo>::new(),
        }
    }

    fn add_new_review(&mut self, new_review_id: String) {
        let new_review_info = ReviewInfo::normal_constructor(new_review_id);
        self.review_ids.push(new_review_info);
    }

    fn get_reviews_last_update(&mut self) {
        // => Local Vars
        let base_path = String::from("https://review-board.natinst.com/api/review-requests/");
        let last_update_method = String::from("/last-update/");

        // => Iterate over the reviews
        for temp_review_id in &mut self.review_ids {
            let request_url = temp_review_id.create_request_url(&base_path, &last_update_method);
        
            // => Call Review Board API
            let client = reqwest::blocking::Client::new();
            let last_update_json: Value = client.get(&request_url)
                .header(AUTHORIZATION, "token ".to_owned() + &self.token)
                .send().unwrap()
                .json().unwrap();
        
            // => Update info, IF new info available
            if last_update_json["last_update"]["timestamp"].to_string() != temp_review_id.timestamp {
                temp_review_id.update_review_info(&last_update_json);
            }
            // => Clear the new_update_available member
            else {
                temp_review_id.new_update_available = false;
            }
        }
    }
}

fn main() {
    // HTTP Server begins & Create a New Client
    println!("Now listening on localhost:8000"); 
    let single_client = Client::empty_client();
    let request_client = Mutex::new(single_client);

    rouille::start_server("0.0.0.0:8000", move |request| {
        router!(request,
            (GET) (/set_token/{new_token: String}) => {
                (*request_client.lock().unwrap()).token = new_token;
                println!("==> NEW TOKEN: {}", (*request_client.lock().unwrap()).token);
                rouille::Response::text("TOKEN UPDATED!")
            },
            (GET) (/add_review_id/{new_review_id: String})  => {
                (*request_client.lock().unwrap()).add_new_review(new_review_id);
                println!("==> NEW REVIEW ID ADDED. NEW LENGTH: {}", (*request_client.lock().unwrap()).review_ids.len());
                rouille::Response::text("NEW REVIEW UPDATED!")
            },
            (GET) (/get_updates/) => {
                (*request_client.lock().unwrap()).get_reviews_last_update();             
                rouille::Response::json(&(*request_client.lock().unwrap()))
            },

            // The code block is called if none of the other blocks matches the request.
            // We return an empty response with a 404 status code.
            _ => rouille::Response::empty_404()
        )
    });
}