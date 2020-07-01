CREATE TABLE chapters
(
    id       INTEGER   NOT NULL PRIMARY KEY AUTOINCREMENT,
    manga_id INTEGER   NOT NULL,
    number   INTEGER   NOT NULL,
    read_on  TIMESTAMP NOT NULL,
    FOREIGN KEY (manga_id) REFERENCES manga (mal_id)
);
