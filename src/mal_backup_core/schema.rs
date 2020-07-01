table! {
    anime (mal_id) {
        mal_id -> Integer,
        title -> Text,
        anime_type -> Text,
        watching_status -> Integer,
        score -> Integer,
        watched_episodes -> Integer,
        total_episodes -> Integer,
        airing_status -> Integer,
        is_rewatching -> Bool,
        tags -> Nullable<Text>,
        start_date -> Nullable<Text>,
        end_date -> Nullable<Text>,
        watch_start_date -> Nullable<Text>,
        watch_end_date -> Nullable<Text>,
        days -> Nullable<Integer>,
    }
}

table! {
    manga (mal_id) {
        mal_id -> Integer,
        title -> Text,
        manga_type -> Text,
        reading_status -> Integer,
        score -> Integer,
        read_chapters -> Integer,
        read_volumes -> Integer,
        total_chapters -> Integer,
        total_volumes -> Integer,
        publishing_status -> Integer,
        is_rereading -> Bool,
        tags -> Nullable<Text>,
        start_date -> Nullable<Text>,
        end_date -> Nullable<Text>,
        read_start_date -> Nullable<Text>,
        read_end_date -> Nullable<Text>,
        days -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(anime, manga,);
