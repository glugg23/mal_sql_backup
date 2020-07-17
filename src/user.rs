use diesel::{RunQueryDsl, SqliteConnection};
use diesel::result::Error;
use serde::Deserialize;

use crate::models::{self, FavoriteAnime, FavoriteManga};
use crate::schema::{favourite_anime, favourite_manga, users};

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub anime_stats: AnimeStats,
    pub manga_stats: MangaStats,
    pub favorites: Favorites,
}

impl User {
    pub fn save(&self, connection: &SqliteConnection) -> Result<(), Error> {
        let favourite_anime: Vec<_> = self
            .favorites
            .anime
            .iter()
            .enumerate()
            .map(|(i, a)| FavoriteAnime::new(a.mal_id, self.user_id, i + 1))
            .collect();

        let favourite_manga: Vec<_> = self
            .favorites
            .manga
            .iter()
            .enumerate()
            .map(|(i, m)| FavoriteManga::new(m.mal_id, self.user_id, i + 1))
            .collect();

        let user: models::User = self.into();

        diesel::insert_into(users::table)
            .values(user)
            .execute(connection)?;

        diesel::insert_into(favourite_anime::table)
            .values(favourite_anime)
            .execute(connection)?;

        diesel::insert_into(favourite_manga::table)
            .values(favourite_manga)
            .execute(connection)?;

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct AnimeStats {
    pub days_watched: f64,
    pub mean_score: f64,
    pub watching: i32,
    pub completed: i32,
    pub on_hold: i32,
    pub dropped: i32,
    pub plan_to_watch: i32,
    pub total_entries: i32,
    pub rewatched: i32,
    pub episodes_watched: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MangaStats {
    pub days_read: f64,
    pub mean_score: f64,
    pub reading: i32,
    pub completed: i32,
    pub on_hold: i32,
    pub dropped: i32,
    pub plan_to_read: i32,
    pub total_entries: i32,
    pub reread: i32,
    pub chapters_read: i32,
    pub volumes_read: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Favorites {
    pub anime: Vec<Favorite>,
    pub manga: Vec<Favorite>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Favorite {
    pub mal_id: i32,
    name: String,
}
