use std::process::exit;
use serde_json::Value;
use error_chain::error_chain;

extern crate tokio;

error_chain!{
    foreign_links{
        HttpRequest(reqwest::Error);
        JsonLib(serde_json::Error);
    }
}


#[tokio::main]
pub  async fn make_request(uri: String) -> Result<Value> {
    // TODO: send api_key with request to login (basic auth username)
    let req_resp = reqwest::get(uri).await.unwrap();
    match req_resp.status() {
        reqwest::StatusCode::OK => {
            let data = req_resp.text().await.unwrap();
            let raw_json: Value = serde_json::from_str(data.as_str()).unwrap();

            Ok(raw_json)
        }
        _ => {
            // Err() doesnt work here TODO: fix Err() in make_request
            dbg!(req_resp);
            exit(1)
        }
    }
}
