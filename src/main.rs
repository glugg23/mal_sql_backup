use clap::{App, Arg};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use std::convert::TryFrom;

fn main() {
    let args = App::new("MAL SQL Backup")
        .version("0.1.0")
        .author("Max Leonhardt <m.leonhardt424@gmail.com>")
        .arg(
            Arg::with_name("session")
                .short("s")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let session = args.value_of("session").unwrap();

    let client = Client::builder().cookie_store(true).build().unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(
        COOKIE,
        HeaderValue::try_from(format!("MALSESSIONID={};is_logged_in=1", session)).unwrap(),
    );

    let res = client
        .get("https://myanimelist.net/ajaxtb.php?detailedaid=1")
        .headers(headers)
        .send()
        .unwrap();

    println!("{}", res.text().unwrap());
}
