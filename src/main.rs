use error_chain::error_chain;
use serde_json::Value;

extern crate dotenv;
extern crate tokio;

error_chain! {
    foreign_links {
        HttpRequest(reqwest::Error);
        JsonLib(serde_json::Error);
        DotEnv(dotenv::Error);
    }
}

#[derive(Debug)]
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
            // figure out way to add slashes after these, cant do same as URIstuff
            api_key: dotenv::var("API_KEY").unwrap(),
            e_mpan_number: dotenv::var("E_MPAN_NUMBER").unwrap(),
            e_serial_number: dotenv::var("E_SERIAL_NUMBER").unwrap(),
            g_mprn_number: dotenv::var("G_MPRN_NUMBER").unwrap(),
            g_serial_number: dotenv::var("G_SERIAL_NUMBER").unwrap(),
        }
    }
}

#[derive(Debug)]
struct URIstuff {
    base_uri: String,
    section_one: String,
    section_two: String,
    section_three: String,
}

impl URIstuff {
    // Example uri: "https://api.octopus.energy/v1/electricity-meter-points/x_MPAN_NUMBER/meters/x_SERIAL_NUMBER/consumption/"
    /*               [          base_uri          ][      section_one      ][mpan_number][ s_two][ serial_number][ s_three  ]*/
    fn new(s_one: String, s_two: String, s_three: String) -> Self {
        Self {
            base_uri: String::from("https://api.octopus.energy/v1/"),
            // very hacky solution for concatenating fwd slashes, TODO: consider changing
            section_one: {
                let mut s = s_one;
                s.push_str("/");
                s
            },
            section_two: {
                let mut s = s_two;
                s.push_str("/");
                s
            },
            section_three: {
                let mut s = s_three;
                s.push_str("/");
                s
            },
        }
    }
}

fn construct_uri(uri_parts: URIstuff, secrets: APIstuff) -> Result<String> {
    let uri = format!(
        "{}{}{}/{}{}/{}",
        uri_parts.base_uri,
        uri_parts.section_one,
        secrets.e_mpan_number,
        uri_parts.section_two,
        secrets.e_serial_number,
        uri_parts.section_three
    );

    Ok(uri)
}

#[tokio::main]
async fn make_request(uri: String) -> Result<Value> {
    // TODO: send api_key with request to login (basic auth username)
    let req_resp = reqwest::get(uri).await.unwrap();
    match req_resp.status() {
        reqwest::StatusCode::OK => {
            let data = req_resp.text().await?;
            let raw_json: Value = serde_json::from_str(data.as_str())?;

            Ok(raw_json)
        }
        _ => {
            // Err() doesnt work here TODO: fix Err() in make_request
            dbg!(req_resp);
            panic!()
        }
    }
}

fn main() -> Result<()> {
    dotenv::from_filename(".env")?; // dont know if needed, pls verify
    let goodies = APIstuff::new();

    // TODO: Concatenate the uri with all the relevant stuff
    let uri_components = URIstuff::new(
        String::from("electricity-meter-points"),
        String::from("meters"),
        String::from("consumption"),
    );

    let r = make_request(construct_uri(uri_components, goodies).unwrap());

    println!("testing: {:#?}", r /*["detail"]*/);

    Ok(())
}
