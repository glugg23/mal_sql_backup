CREATE TABLE episodes
(
    id         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    anime_id   INTEGER NOT NULL,
    number     INTEGER NOT NULL,
    watched_on TIMESTAMP NOT NULL,
    FOREIGN KEY (anime_id) REFERENCES anime (mal_id)
);
