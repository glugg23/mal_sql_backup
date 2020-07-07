# MAL SQL Backup

## Usage

```bash
mal_sql_backup -p <password> -u <username> [skip-planned]
```

Username and password are required in order to fetch episode and chapter records.

If you want to skip attempting to fetch chapter and episode information for planned entries, 
use the `skip-planned` flag, this will speed up the backup.

## Description

This is a program to backup your MAL lists to a SQLite database, as a way to persist your data, 
and to allow ad-hoc queries.

This is still heavily in-progress, has only been tested with my own personal lists, 
and databases produced with any build are not guaranteed to work with any future builds until a 
`1.0.0` version is released.
