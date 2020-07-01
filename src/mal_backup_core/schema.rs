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
