CREATE TABLE manga
(
    mal_id            INTEGER NOT NULL PRIMARY KEY,
    title             VARCHAR NOT NULL,
    manga_type        VARCHAR NOT NULL,
    reading_status    INTEGER NOT NULL,
    score             INTEGER NOT NULL,
    read_chapters     INTEGER NOT NULL,
    read_volumes      INTEGER NOT NULL,
    total_chapters    INTEGER NOT NULL,
    total_volumes     INTEGER NOT NULL,
    publishing_status INTEGER NOT NULL,
    is_rereading      BOOLEAN NOT NULL,
    tags              VARCHAR NULLABLE,
    start_date        VARCHAR NULLABLE,
    end_date          VARCHAR NULLABLE,
    read_start_date   VARCHAR NULLABLE,
    read_end_date     VARCHAR NULLABLE,
    days              INTEGER NULLABLE
);
