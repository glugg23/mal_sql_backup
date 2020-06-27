use crate::episode::Episode;
use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use reqwest::Error;
use scraper::{Html, Selector};

pub mod episode;

const SELECTOR: &'static str = ".spaceit_pad";

pub fn get_anime_episodes(
    anime_id: u32,
    session: &str,
    client: &Client,
) -> Result<Vec<Episode>, Error> {
    let res = client
        .get(
            format!(
                "https://myanimelist.net/ajaxtb.php?detailedaid={}",
                anime_id
            )
            .as_str(),
        )
        .header(COOKIE, format!("MALSESSIONID={};is_logged_in=1", session))
        .send()?;

    let html = Html::parse_document(res.text()?.as_str());
    let selector = Selector::parse(SELECTOR).unwrap();

    Ok(html
        .select(&selector)
        .map(|e| Episode::new(anime_id, e.text().next().unwrap()))
        .collect())
}
