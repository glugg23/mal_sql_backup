#[macro_use]
extern crate diesel;

use crate::anime::Anime;
use crate::chapter::Chapter;
use crate::episode::Episode;
use crate::manga::Manga;
use crate::request::Request;
use crate::user::User;
use reqwest::Error;
use scraper::{Html, Selector};
use serde::Deserialize;

pub mod anime;
pub mod chapter;
pub mod detail;
pub mod episode;
pub mod manga;
pub mod models;
pub mod request;
pub mod schema;
pub mod session;
pub mod user;

pub fn get_user_stats(username: &str, request: &Request) -> Result<User, Error> {
    let res = request.user_stats(username)?;
    Ok(res.json()?)
}

pub fn get_anime_list(user: &User, request: &Request) -> Result<Vec<Anime>, Error> {
    #[derive(Deserialize)]
    struct AnimeListResponse {
        anime: Vec<Anime>,
    }

    let pages = (user.anime_stats.total_entries / 300) + 1;
    let mut anime_list = Vec::new();

    for i in 1..=pages {
        let res = request.anime_list(&user.username, i)?;

        let mut res: AnimeListResponse = res.json()?;
        anime_list.append(&mut res.anime);
    }

    Ok(anime_list)
}

pub fn get_manga_list(user: &User, request: &Request) -> Result<Vec<Manga>, Error> {
    #[derive(Deserialize)]
    struct MangaListResponse {
        manga: Vec<Manga>,
    }

    let pages = (user.manga_stats.total_entries / 300) + 1;
    let mut manga_list = Vec::new();

    for i in 1..=pages {
        let res = request.manga_list(&user.username, i)?;

        let mut res: MangaListResponse = res.json()?;
        manga_list.append(&mut res.manga);
    }

    Ok(manga_list)
}

pub fn get_anime_episodes(anime_id: i32, request: &Request) -> Result<Vec<Episode>, Error> {
    let res = request.anime_episodes(anime_id)?;

    let html = Html::parse_document(res.text()?.as_str());
    let selector = Selector::parse(".spaceit_pad").unwrap();

    Ok(html
        .select(&selector)
        .map(|e| Episode::new(anime_id, e.text().next().unwrap()))
        .collect())
}

pub fn get_manga_chapters(manga_id: i32, request: &Request) -> Result<Vec<Chapter>, Error> {
    let res = request.manga_chapters(manga_id)?;

    let html = Html::parse_document(res.text()?.as_str());
    let selector = Selector::parse(".spaceit_pad").unwrap();

    Ok(html
        .select(&selector)
        .map(|e| Chapter::new(manga_id, e.text().next().unwrap()))
        .collect())
}

#[derive(PartialEq)]
pub enum Skip {
    None,
    All,
    Planned,
    Anime,
    Manga,
}

impl From<&str> for Skip {
    fn from(string: &str) -> Self {
        let string = string.to_lowercase();

        match string.as_str() {
            "none" => Skip::None,
            "all" => Skip::All,
            "planned" => Skip::Planned,
            "anime" => Skip::Anime,
            "manga" => Skip::Manga,
            _ => panic!(format!("Invalid conversion from '{}' to Skip enum", string)),
        }
    }
}

#[derive(PartialEq)]
pub enum Detail {
    None,
    All,
    Anime,
    Manga,
}

impl From<&str> for Detail {
    fn from(string: &str) -> Self {
        let string = string.to_lowercase();

        match string.as_str() {
            "none" => Detail::None,
            "all" => Detail::All,
            "anime" => Detail::Anime,
            "manga" => Detail::Manga,
            _ => panic!(format!(
                "Invalid conversion from '{}' to Detail enum",
                string
            )),
        }
    }
}
