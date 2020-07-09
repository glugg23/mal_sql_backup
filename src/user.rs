use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub anime_stats: AnimeStats,
    pub manga_stats: MangaStats,
    pub favorites: Favorites,
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
