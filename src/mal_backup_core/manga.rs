use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Manga {
    pub mal_id: u32,
    title: String,
    #[serde(rename = "type")]
    manga_type: String,
    pub reading_status: u8,
    score: u8,
    read_chapters: u32,
    read_volumes: u32,
    total_chapters: u32,
    total_volumes: u32,
    publishing_status: u8,
    is_rereading: bool,
    tags: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    read_start_date: Option<String>,
    read_end_date: Option<String>,
    days: Option<u32>,
}
