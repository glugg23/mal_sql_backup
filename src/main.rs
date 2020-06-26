use clap::{App, Arg};
use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use scraper::{Html, Selector};

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

    let client = Client::builder().build().unwrap();

    let res = client
        .get("https://myanimelist.net/ajaxtb.php?detailedaid=1")
        .header(COOKIE, format!("MALSESSIONID={};is_logged_in=1", session))
        .send()
        .unwrap();

    let html = Html::parse_document(res.text().unwrap().as_str());
    let selector = Selector::parse(".spaceit_pad").unwrap();

    for elem in html.select(&selector) {
        println!("{}", elem.text().next().unwrap().trim_end());
    }
}
