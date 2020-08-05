CREATE TABLE manga_details
(
    mal_id       INTEGER NOT NULL PRIMARY KEY,
    priority     INTEGER NOT NULL,
    storage      INTEGER NOT NULL,
    times_reread INTEGER NOT NULL,
    reread_value INTEGER,
    comments     VARCHAR,
    FOREIGN KEY (mal_id) REFERENCES manga (mal_id),
    FOREIGN KEY (priority) REFERENCES priorities (id),
    FOREIGN KEY (storage) REFERENCES manga_storage (id),
    FOREIGN KEY (reread_value) REFERENCES repeat_value (id)
);

CREATE TABLE manga_storage
(
    id          INTEGER NOT NULL PRIMARY KEY,
    description VARCHAR NOT NULL
);

INSERT INTO manga_storage
VALUES (1, 'Hard Drive'),
       (2, 'DVD / CD'),
       (3, 'None'),
       (4, 'Retail Manga'),
       (5, 'Magazine'),
       (6, 'External HD'),
       (7, 'NAS'),
       (8, 'Blu-ray');
