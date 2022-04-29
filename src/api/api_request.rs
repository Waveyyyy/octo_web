use std::process::exit;
use serde_json::Value;
use reqwest::Client;

extern crate tokio;
use crate::api::api_utils::*;


#[tokio::main]
pub async fn make_request(uri: String, secrets: &APIstuff) -> Result<Value> {

    // initialise a new client
    let  client = Client::new();
    let  password: Option<String> = None;

    // make request and await response
    let req_resp = client
        .get(uri)
        .basic_auth(&secrets.api_key, password)
        .send()
        .await
        .unwrap();

    // Some 'error' handling
    match req_resp.status() {
        reqwest::StatusCode::OK => {
            let data = req_resp.text()
                .await
                .unwrap();
            let raw_json: Value = serde_json::from_str(data.as_str())
                .unwrap();

            Ok(raw_json)
        }
        _ => {
            // Err() doesnt work here TODO: fix Err() in make_request
            dbg!(req_resp);
            exit(1)
        }
    }
}
