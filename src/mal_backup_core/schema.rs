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
    chapters (id) {
        id -> Integer,
        manga_id -> Integer,
        number -> Integer,
        read_on -> Timestamp,
    }
}

table! {
    episodes (id) {
        id -> Integer,
        anime_id -> Integer,
        number -> Integer,
        watched_on -> Timestamp,
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

joinable!(chapters -> manga (manga_id));
joinable!(episodes -> anime (anime_id));

allow_tables_to_appear_in_same_query!(anime, chapters, episodes, manga,);
