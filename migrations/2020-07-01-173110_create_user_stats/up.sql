CREATE TABLE users
(
    user_id             INTEGER NOT NULL PRIMARY KEY,
    username            VARCHAR NOT NULL,
    days_watched        DOUBLE  NOT NULL,
    anime_mean_score    DOUBLE  NOT NULL,
    anime_watching      INTEGER NOT NULL,
    anime_completed     INTEGER NOT NULL,
    anime_on_hold       INTEGER NOT NULL,
    anime_dropped       INTEGER NOT NULL,
    anime_plan_to_watch INTEGER NOT NULL,
    anime_total_entries INTEGER NOT NULL,
    anime_rewatched     INTEGER NOT NULL,
    episodes_watched    INTEGER NOT NULL,
    days_read           DOUBLE  NOT NULL,
    manga_mean_score    DOUBLE  NOT NULL,
    manga_reading       INTEGER NOT NULL,
    manga_completed     INTEGER NOT NULL,
    manga_on_hold       INTEGER NOT NULL,
    manga_dropped       INTEGER NOT NULL,
    manga_plan_to_read  INTEGER NOT NULL,
    manga_total_entries INTEGER NOT NULL,
    manga_reread        INTEGER NOT NULL,
    chapters_read       INTEGER NOT NULL,
    volumes_read        INTEGER NOT NULL
);

CREATE TABLE favourite_anime
(
    user_id INTEGER NOT NULL,
    mal_id  INTEGER NOT NULL,
    rank    INTEGER NOT NULL,
    PRIMARY KEY (user_id, mal_id),
    FOREIGN KEY (user_id) REFERENCES users (user_id),
    FOREIGN KEY (mal_id) REFERENCES anime (mal_id)
) WITHOUT ROWID;

CREATE TABLE favourite_manga
(
    user_id INTEGER NOT NULL,
    mal_id  INTEGER NOT NULL,
    rank    INTEGER NOT NULL,
    PRIMARY KEY (user_id, mal_id),
    FOREIGN KEY (user_id) REFERENCES users (user_id),
    FOREIGN KEY (mal_id) REFERENCES manga (mal_id)
) WITHOUT ROWID;
