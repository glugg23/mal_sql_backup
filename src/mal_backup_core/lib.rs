use crate::chapter::Chapter;
use crate::episode::Episode;
use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use reqwest::Error;
use scraper::{Html, Selector};

pub mod chapter;
pub mod episode;

const URL: &'static str = "https://myanimelist.net/ajaxtb.php";
const SESSION_COOKIE: &'static str = "is_logged_in=1;MALSESSIONID=";
const SELECTOR: &'static str = ".spaceit_pad";

pub fn get_anime_episodes(
    anime_id: u32,
    session: &str,
    client: &Client,
) -> Result<Vec<Episode>, Error> {
    let res = client
        .get(format!("{}?detailedaid={}", URL, anime_id).as_str())
        .header(COOKIE, format!("{}{}", SESSION_COOKIE, session))
        .send()?;

    let html = Html::parse_document(res.text()?.as_str());
    let selector = Selector::parse(SELECTOR).unwrap();

    Ok(html
        .select(&selector)
        .map(|e| Episode::new(anime_id, e.text().next().unwrap()))
        .collect())
}

pub fn get_manga_chapters(
    manga_id: u32,
    session: &str,
    client: &Client,
) -> Result<Vec<Chapter>, Error> {
    let res = client
        .get(format!("{}?detailedmid={}", URL, manga_id).as_str())
        .header(COOKIE, format!("{}{}", SESSION_COOKIE, session))
        .send()?;

    let html = Html::parse_document(res.text()?.as_str());
    let selector = Selector::parse(SELECTOR).unwrap();

    Ok(html
        .select(&selector)
        .map(|e| Chapter::new(manga_id, e.text().next().unwrap()))
        .collect())
}
