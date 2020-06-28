use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Anime {
    pub mal_id: u32,
    title: String,
    #[serde(rename = "type")]
    anime_type: String,
    watching_status: u8,
    score: u8,
    watched_episodes: u32,
    total_episodes: u32,
    airing_status: u8,
    is_rewatching: bool,
    tags: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    watch_start_date: Option<String>,
    watch_end_date: Option<String>,
    days: Option<u32>,
}
