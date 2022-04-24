use error_chain::error_chain;
use std::io::Read;
use serde_json::{Value};

extern crate dotenv;

use dotenv::dotenv;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        JsonLib(serde_json::Error);
        DotEnv(dotenv::Error);
    }
}

struct APIstuff {
    api_key: String,
    e_mpan_number: String,
    e_serial_number: String,
    g_mprn_number: String,
    g_serial_number: String,
}

impl APIstuff {
    fn new() -> Self {
        Self {
            api_key: dotenv::var("API_KEY").unwrap(),
            e_mpan_number: dotenv::var("E_MPAN_NUMBER").unwrap(),
            e_serial_number: dotenv::var("E_SERIAL_NUMBER").unwrap(),
            g_mprn_number: dotenv::var("G_MPRN_NUMBER").unwrap(),
            g_serial_number: dotenv::var("G_SERIAL_NUMBER").unwrap(),
        }
    }
}


fn web_request(uri: &str) -> Result<Value>{
    // TODO: send api_key with request to login (basic auth username)
    let mut req_resp = reqwest::blocking::get(uri)?;
    let mut data = String::new();
    req_resp.read_to_string(&mut data)?;
    let raw_json: Value = serde_json::from_str(&data)?;

    Ok(raw_json)
}

fn main() -> Result<()>{
    dotenv::from_filename(".env")?; // dont know if needed, pls verify
    let _goodies = APIstuff::new();

    // TODO: Concatenate the uri with all the meter info
    let r = web_request("https://api.octopus.energy/v1/electricity-meter-points/REPLACE__MPANNUMBER__REPLACE/meters/REPLACE__SERIALNUMBER__REPLACE/consumption/").unwrap();

    println!("testing: {}", r/*["detail"]*/);

    Ok(())
}
