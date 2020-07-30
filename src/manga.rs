use diesel::result::Error;
use diesel::{Insertable, RunQueryDsl, SqliteConnection};
use serde::Deserialize;

use crate::schema::manga as MangaTable;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "MangaTable"]
pub struct Manga {
    pub mal_id: i32,
    pub title: String,
    #[serde(rename = "type")]
    manga_type: String,
    pub reading_status: i32,
    score: i32,
    read_chapters: i32,
    read_volumes: i32,
    total_chapters: i32,
    total_volumes: i32,
    publishing_status: i32,
    is_rereading: bool,
    tags: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    read_start_date: Option<String>,
    read_end_date: Option<String>,
    days: Option<i32>,
}

impl Manga {
    pub fn save(&self, connection: &SqliteConnection) -> Result<(), Error> {
        diesel::insert_into(MangaTable::table)
            .values(self)
            .execute(connection)?;

        Ok(())
    }
}
