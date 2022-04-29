extern crate octo_web;
extern crate dotenv;

use crate::octo_web::api::api_request::*;
use crate::octo_web::api::api_utils::*;

fn main() -> Result<()> {
    dotenv::from_filename(".env")?; // dont know if needed, pls verify
    let goodies = APIstuff::default();

    // TODO: Concatenate the uri with all the relevant stuff
    let uri_components = URIstuff::new(
        String::from("electricity-meter-points"),
        String::from("meters"),
        String::from("consumption"),
    );

    let r = make_request(
        construct_uri(uri_components, &goodies, None),
        &goodies)
        .unwrap();

    println!("testing: {:#?}", r /*["detail"]*/);

    Ok(())
}
