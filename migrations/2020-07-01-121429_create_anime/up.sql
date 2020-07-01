CREATE TABLE anime
(
    mal_id           INTEGER NOT NULL PRIMARY KEY,
    title            VARCHAR NOT NULL,
    anime_type       VARCHAR NOT NULL,
    watching_status  INTEGER NOT NULL,
    score            INTEGER NOT NULL,
    watched_episodes INTEGER NOT NULL,
    total_episodes   INTEGER NOT NULL,
    airing_status    INTEGER NOT NULL,
    is_rewatching    BOOLEAN NOT NULL,
    tags             VARCHAR NULLABLE,
    start_date       VARCHAR NULLABLE,
    end_date         VARCHAR NULLABLE,
    watch_start_date VARCHAR NULLABLE,
    watch_end_date   VARCHAR NULLABLE,
    days             INTEGER NULLABLE
);
