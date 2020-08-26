use reqwest::blocking::{Client, Response};
use reqwest::Error;
use std::fmt::Write;

pub struct Request {
    pub client: Client,
    jikan_url: String,
    episodes_url: String,
    chapters_url: String,
    anime_detail_url: String,
    manga_detail_url: String,
}

impl Request {
    pub fn new(
        client: Client,
        jikan_url: &str,
        episodes_url: &str,
        chapters_url: &str,
        anime_detail_url: &str,
        manga_detail_url: &str,
    ) -> Self {
        Self {
            client,
            jikan_url: jikan_url.to_owned(),
            episodes_url: episodes_url.to_owned(),
            chapters_url: chapters_url.to_owned(),
            anime_detail_url: anime_detail_url.to_owned(),
            manga_detail_url: manga_detail_url.to_owned(),
        }
    }

    pub fn user_stats(&self, username: &str) -> Result<Response, Error> {
        let mut url = String::new();
        write!(url, "{}/user/{}", self.jikan_url, username).unwrap();

        self.client.get(url.as_str()).send()
    }

    pub fn anime_list(&self, username: &str, page: i32) -> Result<Response, Error> {
        let mut url = String::new();
        write!(
            url,
            "{}/user/{}/animelist/all/{}",
            self.jikan_url, username, page
        )
        .unwrap();

        self.client.get(url.as_str()).send()
    }

    pub fn anime_episodes(&self, anime_id: i32) -> Result<Response, Error> {
        let mut url = String::new();
        write!(url, "{}{}", self.episodes_url, anime_id).unwrap();

        self.client.get(url.as_str()).send()
    }

    pub fn anime_detail(&self, anime_id: i32) -> Result<Response, Error> {
        let url = self.anime_detail_url.replace("{}", &anime_id.to_string());

        self.client.get(url.as_str()).send()
    }

    pub fn manga_list(&self, username: &str, page: i32) -> Result<Response, Error> {
        let mut url = String::new();
        write!(
            url,
            "{}/user/{}/mangalist/all/{}",
            self.jikan_url, username, page
        )
        .unwrap();

        self.client.get(url.as_str()).send()
    }

    pub fn manga_chapters(&self, manga_id: i32) -> Result<Response, Error> {
        let mut url = String::new();
        write!(url, "{}{}", self.chapters_url, manga_id).unwrap();

        self.client.get(url.as_str()).send()
    }

    pub fn manga_detail(&self, manga_id: i32) -> Result<Response, Error> {
        let url = self.manga_detail_url.replace("{}", &manga_id.to_string());

        self.client.get(url.as_str()).send()
    }
}

impl Default for Request {
    fn default() -> Self {
        Self {
            client: Client::builder().cookie_store(true).build().unwrap(),
            jikan_url: "https://api.jikan.moe/v3".to_owned(),
            episodes_url: "https://myanimelist.net/ajaxtb.php?detailedaid=".to_owned(),
            chapters_url: "https://myanimelist.net/ajaxtb.php?detailedmid=".to_owned(),
            anime_detail_url: "https://myanimelist.net/ownlist/anime/{}/edit?hideLayout".to_owned(),
            manga_detail_url: "https://myanimelist.net/ownlist/manga/{}/edit?hideLayout".to_owned(),
        }
    }
}
