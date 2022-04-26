pub  fn construct_uri(uri_parts: URIstuff, secrets: APIstuff) -> Result<String, ()> {
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

#[derive(Debug)]
pub  struct APIstuff {
    api_key: String,
    e_mpan_number: String,
    e_serial_number: String,
    g_mprn_number: String,
    g_serial_number: String,
}

impl APIstuff {
    pub  fn new() -> Self {
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
pub  struct URIstuff {
    base_uri: String,
    section_one: String,
    section_two: String,
    section_three: String,
}

impl URIstuff {
    // Example uri: "https://api.octopus.energy/v1/electricity-meter-points/x_MPAN_NUMBER/meters/x_SERIAL_NUMBER/consumption/"
    /*               [          base_uri          ][      section_one      ][mpan_number][ s_two][ serial_number][ s_three  ]*/
    pub  fn new(s_one: String, s_two: String, s_three: String) -> Self {
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
