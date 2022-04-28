pub fn construct_uri(uri_parts: URIstuff, secrets: APIstuff, electricity: Option<bool>) -> Result<String, ()> {
    let uri = format!(
        "{base}{SectOne}{mpxn}{SectTwo}{serial}{SectThree}",
        base = uri_parts.base_uri,
        SectOne = uri_parts.section_one,
        mpxn = {
            if electricity.unwrap_or(true) { secrets.e_mpan_number }
            else { secrets.g_mprn_number }
        },
        SectTwo = uri_parts.section_two,
        serial = {
            if electricity.unwrap_or(true) { secrets.e_serial_number }
            else { secrets.g_serial_number }
        },
        SectThree = uri_parts.section_three
    );

    Ok(uri)
}

#[derive(Debug)]
pub struct APIstuff {
    api_key: String,
    e_mpan_number: String,
    e_serial_number: String,
    g_mprn_number: String,
    g_serial_number: String,
}

impl APIstuff {
    pub fn new() -> Self {
        Self {
            // figure out way to add slashes after these, cant do same as URIstuff
            api_key: dotenv::var("API_KEY").unwrap(),
            e_mpan_number: dotenv::var("E_MPAN_NUMBER").unwrap() + "/",
            e_serial_number: dotenv::var("E_SERIAL_NUMBER").unwrap() + "/",
            g_mprn_number: dotenv::var("G_MPRN_NUMBER").unwrap() + "/",
            g_serial_number: dotenv::var("G_SERIAL_NUMBER").unwrap() + "/",
        }
    }
}

#[derive(Debug)]
pub struct URIstuff {
    base_uri: String,
    section_one: String,
    section_two: String,
    section_three: String,
}

impl URIstuff {
    // Example uri: "https://api.octopus.energy/v1/electricity-meter-points/x_MPAN_NUMBER/meters/x_SERIAL_NUMBER/consumption/"
    /*               [          base_uri          ][      section_one      ][mpan_number][ s_two][ serial_number][ s_three  ]*/
    pub fn new(s_one: String, s_two: String, s_three: String) -> Self {
        Self {
            base_uri: String::from("https://api.octopus.energy/v1/"),
            // very hacky solution for concatenating fwd slashes, TODO: consider changing
            section_one: s_one + "/",
            section_two: s_two + "/",
            section_three: s_three + "/",
        }
    }
}
