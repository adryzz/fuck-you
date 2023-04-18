use colored::Colorize;
use fake::Faker;
use fake::faker::internet::en::IPv4;
use fake::faker::internet::en::UserAgent;
use fake::Dummy;
use fake::Fake;
use rayon::prelude::*;
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use fake::faker::name::en::FirstName;
use fake::faker::name::en::LastName;
use fake::faker::lorem::en::Paragraph;
use fake::faker::address::en::StateAbbr;
use fake::faker::address::en::ZipCode;
use fake::faker::address::en::StreetName;
use fake::faker::internet::en::FreeEmail;
use fake::faker::phone_number::en::PhoneNumber;

const url: &str = "https://ago.mo.gov/file-a-complaint/transgender-center-concerns?sf_cntrl_id=ctl00$MainContent$C001";

#[derive(Debug, Dummy, Serialize, Deserialize)]
pub struct RequestData {
    #[serde(rename = "TextFieldController_4")]
    #[dummy(faker = "FirstName()")]
    first_name: String,

    #[serde(rename = "TextFieldController_5")]
    #[dummy(faker = "LastName()")]
    last_name: String,

    #[serde(rename = "TextFieldController_1")]
    #[dummy(faker = "StreetName()")]
    address: String,
    
    #[serde(rename = "TextFieldController_2")]
    city: String,
    #[serde(rename = "DropdownListFieldController")]
    #[dummy(faker = "StateAbbr()")]
    state_abbr: String,

    #[serde(rename = "TextFieldController_6")]
    #[dummy(faker = "ZipCode()")]
    post_code: String,

    #[serde(rename = "TextFieldController_0")]
    #[dummy(faker = "FreeEmail()")]
    email: String,

    #[serde(rename = "TextFieldController_3")]
    #[dummy(faker = "PhoneNumber()")]
    phone_number: String,

    #[serde(rename = "ParagraphTextFieldController")]
    #[dummy(faker = "Paragraph(1000..2000)")]
    paragraph: String
}

fn main() {
    println!("Setting up...");
    let client = Client::new();
    let bee_movie_script = include_str!("bee_movie_script.txt");
    (0..1_000_000).into_par_iter().for_each(|i| {
        let mut data: RequestData = Faker.fake();

        // remove this for random paragraph generation
        data.paragraph = bee_movie_script.to_string();
        let request = client.post(url).headers(generate_headers()).json::<RequestData>(&data);

        if let Ok(response) = request.send() {
            print!("{}", ".".green());
        } else {
            print!("{}", ".".red());
        }
    });
}

fn generate_headers() -> HeaderMap {
    let ip: String = IPv4().fake();
    let user_agent: String = UserAgent().fake();
    let mut headers = HeaderMap::new();
    headers.append("Content-Type", HeaderValue::from_static("application/json"));
    headers.append(
        "User-Agent",
        HeaderValue::from_str(user_agent.as_ref()).unwrap(),
    );
    headers.append(
        "X-Forwarded-For",
        HeaderValue::from_str(ip.as_ref()).unwrap(),
    );
    headers.append("Cookies", HeaderValue::from_static(""));
    return headers;
}
