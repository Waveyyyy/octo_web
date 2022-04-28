pub  mod api;

extern crate dotenv;

use api::api_utils;
use api::api_request;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env")?; // dont know if needed, pls verify
    let goodies = api_utils::APIstuff::default();

    // TODO: Concatenate the uri with all the relevant stuff
    let uri_components = api_utils::URIstuff::new(
        String::from("electricity-meter-points"),
        String::from("meters"),
        String::from("consumption"),
    );

    let r = api_request::make_request(api_utils::construct_uri(uri_components, goodies, None))
        .unwrap();

    println!("testing: {}", r /*["detail"]*/);

    Ok(())
}
