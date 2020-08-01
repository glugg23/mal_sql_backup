CREATE TABLE anime_details
(
    mal_id          INTEGER NOT NULL PRIMARY KEY,
    priority        INTEGER NOT NULL,
    storage         INTEGER,
    times_rewatched INTEGER NOT NULL,
    rewatch_value   INTEGER,
    comments        VARCHAR,
    FOREIGN KEY (mal_id) REFERENCES anime (mal_id),
    FOREIGN KEY (priority) REFERENCES priorities (id),
    FOREIGN KEY (storage) REFERENCES anime_storage (id),
    FOREIGN KEY (rewatch_value) REFERENCES repeat_value (id)
);

CREATE TABLE priorities
(
    id          INTEGER NOT NULL PRIMARY KEY,
    description VARCHAR NOT NULL
);

INSERT INTO priorities
VALUES (0, 'Low'),
       (1, 'Medium'),
       (2, 'High');

CREATE TABLE anime_storage
(
    id          INTEGER NOT NULL PRIMARY KEY,
    description VARCHAR NOT NULL
);

INSERT INTO anime_storage
VALUES (1, 'Hard Drive'),
       (2, 'DVD / CD'),
       (3, 'None'),
       (4, 'Retail DVD'),
       (5, 'VHS'),
       (6, 'External HD'),
       (7, 'NAS'),
       (8, 'Blu-ray');

CREATE TABLE repeat_value
(
    id          INTEGER NOT NULL PRIMARY KEY,
    description VARCHAR NOT NULL
);

INSERT INTO repeat_value
VALUES (1, 'Very Low'),
       (2, 'Low'),
       (3, 'Medium'),
       (4, 'High'),
       (5, 'Very High');
