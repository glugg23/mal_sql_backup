use diesel::{Insertable, RunQueryDsl, SqliteConnection};
use diesel::result::Error;
use serde::Deserialize;

use crate::schema::anime as AnimeTable;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "AnimeTable"]
pub struct Anime {
    pub mal_id: i32,
    pub title: String,
    #[serde(rename = "type")]
    anime_type: String,
    pub watching_status: i32,
    score: i32,
    watched_episodes: i32,
    total_episodes: i32,
    airing_status: i32,
    is_rewatching: bool,
    tags: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    watch_start_date: Option<String>,
    watch_end_date: Option<String>,
    days: Option<i32>,
}

impl Anime {
    pub fn save(&self, connection: &SqliteConnection) -> Result<(), Error> {
        diesel::insert_into(AnimeTable::table)
            .values(self)
            .execute(connection)?;

        Ok(())
    }
}
