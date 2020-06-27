use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    user_id: u32,
    username: String,
    anime_stats: AnimeStats,
    manga_stats: MangaStats,
    favorites: Favorites,
}

#[derive(Debug, Deserialize)]
pub struct AnimeStats {
    days_watched: f64,
    mean_score: f64,
    watching: u32,
    completed: u32,
    on_hold: u32,
    dropped: u32,
    plan_to_watch: u32,
    total_entries: u32,
    rewatched: u32,
    episodes_watched: u32,
}

#[derive(Debug, Deserialize)]
pub struct MangaStats {
    days_read: f64,
    mean_score: f64,
    reading: u32,
    completed: u32,
    on_hold: u32,
    dropped: u32,
    plan_to_read: u32,
    total_entries: u32,
    reread: u32,
    chapters_read: u32,
    volumes_read: u32,
}

#[derive(Debug, Deserialize)]
pub struct Favorites {
    anime: Vec<FavoriteAnime>,
    manga: Vec<FavoriteManga>,
}

#[derive(Debug, Deserialize)]
pub struct FavoriteAnime {
    mal_id: u32,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct FavoriteManga {
    mal_id: u32,
    name: String,
}
