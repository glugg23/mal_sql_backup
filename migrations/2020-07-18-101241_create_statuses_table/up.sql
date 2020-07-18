CREATE TABLE statuses
(
    id           INTEGER NOT NUlL PRIMARY KEY,
    anime_status VARCHAR NOT NULL,
    manga_status VARCHAR NOT NULL
);

INSERT INTO statuses
VALUES (1, 'Watching', 'Reading'),
       (2, 'Completed', 'Completed'),
       (3, 'On-Hold', 'On-Hold'),
       (4, 'Dropped', 'Dropped'),
       (6, 'Plan to Watch', 'Plan to Read');
