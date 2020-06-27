use crate::anime::Anime;
use crate::chapter::Chapter;
use crate::episode::Episode;
use crate::user::User;
use reqwest::blocking::Client;
use reqwest::Error;
use scraper::{Html, Selector};
use serde::Deserialize;

pub mod anime;
pub mod chapter;
pub mod episode;
pub mod session;
pub mod user;

const MAL_URL: &'static str = "https://myanimelist.net/ajaxtb.php";
const JIKAN_URL: &'static str = "https://api.jikan.moe/v3";
const SELECTOR: &'static str = ".spaceit_pad";

pub fn get_user_stats(username: &str, client: &Client) -> Result<User, Error> {
    let res = client
        .get(format!("{}/user/{}", JIKAN_URL, username).as_str())
        .send()?;

    Ok(res.json()?)
}

#[derive(Deserialize)]
struct AnimeListResponse {
    anime: Vec<Anime>,
}

pub fn get_anime_list(user: &User, client: &Client) -> Result<Vec<Anime>, Error> {
    let pages = (user.anime_stats.total_entries / 300) + 1;
    let mut anime_list = Vec::new();

    for i in 1..=pages {
        let res = client
            .get(format!("{}/user/{}/animelist/all/{}", JIKAN_URL, user.username, i).as_str())
            .send()?;

        let mut res: AnimeListResponse = res.json()?;
        anime_list.append(&mut res.anime);
    }

    Ok(anime_list)
}

pub fn get_anime_episodes(anime_id: u32, client: &Client) -> Result<Vec<Episode>, Error> {
    let res = client
        .get(format!("{}?detailedaid={}", MAL_URL, anime_id).as_str())
        .send()?;

    let html = Html::parse_document(res.text()?.as_str());
    let selector = Selector::parse(SELECTOR).unwrap();

    Ok(html
        .select(&selector)
        .map(|e| Episode::new(anime_id, e.text().next().unwrap()))
        .collect())
}

pub fn get_manga_chapters(manga_id: u32, client: &Client) -> Result<Vec<Chapter>, Error> {
    let res = client
        .get(format!("{}?detailedmid={}", MAL_URL, manga_id).as_str())
        .send()?;

    let html = Html::parse_document(res.text()?.as_str());
    let selector = Selector::parse(SELECTOR).unwrap();

    Ok(html
        .select(&selector)
        .map(|e| Chapter::new(manga_id, e.text().next().unwrap()))
        .collect())
}
