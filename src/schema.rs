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
    anime_details (mal_id) {
        mal_id -> Integer,
        priority -> Integer,
        storage -> Nullable<Integer>,
        times_rewatched -> Integer,
        rewatch_value -> Nullable<Integer>,
        comments -> Nullable<Text>,
    }
}

table! {
    anime_storage (id) {
        id -> Integer,
        description -> Text,
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
    favourite_anime (user_id, mal_id) {
        user_id -> Integer,
        mal_id -> Integer,
        rank -> Integer,
    }
}

table! {
    favourite_manga (user_id, mal_id) {
        user_id -> Integer,
        mal_id -> Integer,
        rank -> Integer,
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

table! {
    manga_details (mal_id) {
        mal_id -> Integer,
        priority -> Integer,
        storage -> Integer,
        times_reread -> Integer,
        reread_value -> Nullable<Integer>,
        comments -> Nullable<Text>,
    }
}

table! {
    manga_storage (id) {
        id -> Integer,
        description -> Text,
    }
}

table! {
    priorities (id) {
        id -> Integer,
        description -> Text,
    }
}

table! {
    repeat_value (id) {
        id -> Integer,
        description -> Text,
    }
}

table! {
    statuses (id) {
        id -> Integer,
        anime_status -> Text,
        manga_status -> Text,
    }
}

table! {
    users (user_id) {
        user_id -> Integer,
        username -> Text,
        joined -> Text,
        days_watched -> Double,
        anime_mean_score -> Double,
        anime_watching -> Integer,
        anime_completed -> Integer,
        anime_on_hold -> Integer,
        anime_dropped -> Integer,
        anime_plan_to_watch -> Integer,
        anime_total_entries -> Integer,
        anime_rewatched -> Integer,
        episodes_watched -> Integer,
        days_read -> Double,
        manga_mean_score -> Double,
        manga_reading -> Integer,
        manga_completed -> Integer,
        manga_on_hold -> Integer,
        manga_dropped -> Integer,
        manga_plan_to_read -> Integer,
        manga_total_entries -> Integer,
        manga_reread -> Integer,
        chapters_read -> Integer,
        volumes_read -> Integer,
    }
}

joinable!(anime -> statuses (watching_status));
joinable!(anime_details -> anime (mal_id));
joinable!(anime_details -> anime_storage (storage));
joinable!(anime_details -> priorities (priority));
joinable!(anime_details -> repeat_value (rewatch_value));
joinable!(chapters -> manga (manga_id));
joinable!(episodes -> anime (anime_id));
joinable!(favourite_anime -> anime (mal_id));
joinable!(favourite_anime -> users (user_id));
joinable!(favourite_manga -> manga (mal_id));
joinable!(favourite_manga -> users (user_id));
joinable!(manga -> statuses (reading_status));
joinable!(manga_details -> manga (mal_id));
joinable!(manga_details -> manga_storage (storage));
joinable!(manga_details -> priorities (priority));
joinable!(manga_details -> repeat_value (reread_value));

allow_tables_to_appear_in_same_query!(
    anime,
    anime_details,
    anime_storage,
    chapters,
    episodes,
    favourite_anime,
    favourite_manga,
    manga,
    manga_details,
    manga_storage,
    priorities,
    repeat_value,
    statuses,
    users,
);
