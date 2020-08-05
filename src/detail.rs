use diesel::{Insertable, RunQueryDsl, SqliteConnection};
use reqwest::blocking::Client;
use scraper::{Html, Selector};

use crate::schema::{anime_details, manga_details};

#[derive(Insertable)]
#[table_name = "anime_details"]
pub struct AnimeDetail {
    mal_id: i32,
    priority: i32,
    storage: Option<i32>,
    times_rewatched: i32,
    rewatch_value: Option<i32>,
    comments: Option<String>,
}

impl AnimeDetail {
    pub fn get(client: &Client, mal_id: i32) -> Result<Self, reqwest::Error> {
        let res = client
            .get(
                format!(
                    "https://myanimelist.net/ownlist/anime/{}/edit?hideLayout",
                    mal_id
                )
                .as_str(),
            )
            .send()?;

        let html = Html::parse_document(res.text()?.as_str());

        let selector = Selector::parse("#add_anime_priority option[selected=selected]").unwrap();
        let priority: i32 = html
            .select(&selector)
            .next()
            .unwrap()
            .value()
            .attr("value")
            .unwrap()
            .parse()
            .unwrap();

        let selector =
            Selector::parse("#add_anime_storage_type option[selected=selected]").unwrap();
        let storage: Option<i32> = html
            .select(&selector)
            .next()
            .map(|e| e.value().attr("value").unwrap().parse().unwrap());

        let selector = Selector::parse("#add_anime_num_watched_times").unwrap();
        let times_rewatched: i32 = html
            .select(&selector)
            .next()
            .unwrap()
            .value()
            .attr("value")
            .unwrap()
            .parse()
            .unwrap();

        let selector =
            Selector::parse("#add_anime_rewatch_value option[selected=selected]").unwrap();
        let rewatch_value: Option<i32> = html
            .select(&selector)
            .next()
            .map(|e| e.value().attr("value").unwrap().parse().unwrap());

        let selector = Selector::parse("#add_anime_comments").unwrap();
        let comments: String = html.select(&selector).next().unwrap().text().collect();
        let comments = if comments.is_empty() {
            None
        } else {
            Some(comments)
        };

        Ok(AnimeDetail {
            mal_id,
            priority,
            storage,
            times_rewatched,
            rewatch_value,
            comments,
        })
    }

    pub fn save(&self, connection: &SqliteConnection) -> Result<(), diesel::result::Error> {
        diesel::insert_into(anime_details::table)
            .values(self)
            .execute(connection)?;

        Ok(())
    }
}

#[derive(Insertable)]
#[table_name = "manga_details"]
pub struct MangaDetail {
    mal_id: i32,
    priority: i32,
    storage: i32,
    times_reread: i32,
    reread_value: Option<i32>,
    comments: Option<String>,
}

impl MangaDetail {
    pub fn get(client: &Client, mal_id: i32) -> Result<Self, reqwest::Error> {
        let res = client
            .get(
                format!(
                    "https://myanimelist.net/ownlist/manga/{}/edit?hideLayout",
                    mal_id
                )
                .as_str(),
            )
            .send()?;

        let html = Html::parse_document(res.text()?.as_str());

        let selector = Selector::parse("#add_manga_priority option[selected=selected]").unwrap();
        let priority: i32 = html
            .select(&selector)
            .next()
            .unwrap()
            .value()
            .attr("value")
            .unwrap()
            .parse()
            .unwrap();

        let selector =
            Selector::parse("#add_manga_storage_type option[selected=selected]").unwrap();
        let storage: i32 = html
            .select(&selector)
            .next()
            .map(|e| e.value().attr("value").unwrap().parse().unwrap())
            .unwrap_or(3); //Malformed HTML in input, default value should be ID 3 for 'None'

        let selector = Selector::parse("#add_manga_num_read_times").unwrap();
        let times_reread: i32 = html
            .select(&selector)
            .next()
            .unwrap()
            .value()
            .attr("value")
            .unwrap()
            .parse()
            .unwrap();

        let selector =
            Selector::parse("#add_manga_reread_value option[selected=selected]").unwrap();
        let reread_value: Option<i32> = html
            .select(&selector)
            .next()
            .map(|e| e.value().attr("value").unwrap().parse().unwrap());

        let selector = Selector::parse("#add_manga_comments").unwrap();
        let comments: String = html.select(&selector).next().unwrap().text().collect();
        let comments = if comments.is_empty() {
            None
        } else {
            Some(comments)
        };

        Ok(MangaDetail {
            mal_id,
            priority,
            storage,
            times_reread,
            reread_value,
            comments,
        })
    }

    pub fn save(&self, connection: &SqliteConnection) -> Result<(), diesel::result::Error> {
        diesel::insert_into(manga_details::table)
            .values(self)
            .execute(connection)?;

        Ok(())
    }
}
