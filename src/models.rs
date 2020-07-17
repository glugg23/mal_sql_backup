use diesel::Insertable;

use crate::schema::{favourite_anime, favourite_manga, users};
use crate::user;

#[derive(Insertable)]
#[table_name = "users"]
pub struct User {
    user_id: i32,
    username: String,
    days_watched: f64,
    anime_mean_score: f64,
    anime_watching: i32,
    anime_completed: i32,
    anime_on_hold: i32,
    anime_dropped: i32,
    anime_plan_to_watch: i32,
    anime_total_entries: i32,
    anime_rewatched: i32,
    episodes_watched: i32,
    days_read: f64,
    manga_mean_score: f64,
    manga_reading: i32,
    manga_completed: i32,
    manga_on_hold: i32,
    manga_dropped: i32,
    manga_plan_to_read: i32,
    manga_total_entries: i32,
    manga_reread: i32,
    chapters_read: i32,
    volumes_read: i32,
}

impl From<&user::User> for User {
    fn from(user: &user::User) -> Self {
        User {
            user_id: user.user_id,
            username: user.username.clone(),
            days_watched: user.anime_stats.days_watched,
            anime_mean_score: user.anime_stats.mean_score,
            anime_watching: user.anime_stats.watching,
            anime_completed: user.anime_stats.completed,
            anime_on_hold: user.anime_stats.on_hold,
            anime_dropped: user.anime_stats.dropped,
            anime_plan_to_watch: user.anime_stats.plan_to_watch,
            anime_total_entries: user.anime_stats.total_entries,
            anime_rewatched: user.anime_stats.rewatched,
            episodes_watched: user.anime_stats.episodes_watched,
            days_read: user.manga_stats.days_read,
            manga_mean_score: user.manga_stats.mean_score,
            manga_reading: user.manga_stats.reading,
            manga_completed: user.manga_stats.completed,
            manga_on_hold: user.manga_stats.on_hold,
            manga_dropped: user.manga_stats.dropped,
            manga_plan_to_read: user.manga_stats.plan_to_read,
            manga_total_entries: user.manga_stats.total_entries,
            manga_reread: user.manga_stats.reread,
            chapters_read: user.manga_stats.chapters_read,
            volumes_read: user.manga_stats.volumes_read,
        }
    }
}

#[derive(Insertable)]
#[table_name = "favourite_anime"]
pub struct FavoriteAnime {
    mal_id: i32,
    user_id: i32,
    rank: i32,
}

impl FavoriteAnime {
    pub fn new(mal_id: i32, user_id: i32, rank: usize) -> Self {
        FavoriteAnime { mal_id, user_id, rank: rank as i32 }
    }
}

#[derive(Insertable)]
#[table_name = "favourite_manga"]
pub struct FavoriteManga {
    mal_id: i32,
    user_id: i32,
    rank: i32,
}

impl FavoriteManga {
    pub fn new(mal_id: i32, user_id: i32, rank: usize) -> Self {
        FavoriteManga { mal_id, user_id, rank: rank as i32 }
    }
}
