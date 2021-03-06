#[macro_use]
extern crate log;

use std::env;
use std::fs::File;
use std::process::exit;

use clap::{App, Arg};
use diesel::{Connection, ConnectionError, SqliteConnection};
use diesel_migrations::run_pending_migrations;
use dotenv::dotenv;
use simplelog::*;

use mal_sql_backup::detail::{AnimeDetail, MangaDetail};
use mal_sql_backup::request::Request;
use mal_sql_backup::session::set_session_cookie;
use mal_sql_backup::{
    get_anime_episodes, get_anime_list, get_manga_chapters, get_manga_list, get_user_stats, Detail,
    Skip,
};

pub fn get_db_connection() -> Result<SqliteConnection, ConnectionError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
}

fn main() {
    let args = App::new("MAL SQL Backup")
        .version("0.1.0")
        .author("Max Leonhardt <m.leonhardt424@gmail.com>")
        .arg(
            Arg::with_name("username")
                .short("u")
                .long("--username")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .long("--password")
                .takes_value(true)
                .required_ifs(&[
                    ("skip", "none"),
                    ("skip", "planned"),
                    ("skip", "anime"),
                    ("skip", "manga"),
                    ("detail", "all"),
                    ("detail", "anime"),
                    ("detail", "manga"),
                ]),
        )
        .arg(
            Arg::with_name("skip")
                .short("S")
                .long("--skip")
                .takes_value(true)
                .possible_values(&["none", "all", "planned", "anime", "manga"])
                .default_value("all")
                .case_insensitive(true),
        )
        .arg(
            Arg::with_name("detail")
                .short("D")
                .long("--detail")
                .takes_value(true)
                .possible_values(&["none", "all", "anime", "manga"])
                .default_value("none")
                .case_insensitive(true),
        )
        .get_matches();

    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(
            LevelFilter::Warn,
            Config::default(),
            File::create("mal.log").unwrap(),
        ),
    ])
    .unwrap();

    let username = args.value_of("username").unwrap();
    let skip: Skip = args.value_of("skip").unwrap().into();
    let detail: Detail = args.value_of("detail").unwrap().into();
    let password = args.value_of("password");

    let connection = match get_db_connection() {
        Ok(c) => {
            info!("Connected to database");
            c
        }
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    };

    match run_pending_migrations(&connection) {
        Ok(_) => info!("Ran pending database migrations"),
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    };

    let request = Request::default();

    match password {
        Some(p) => {
            match set_session_cookie(&request, username, p) {
                Ok(_) => info!("Logged in as '{}'", username),
                Err(e) => {
                    error!("{}", e);
                    exit(1);
                }
            };
        }
        None => (),
    }

    let user = match get_user_stats(username, &request) {
        Ok(u) => {
            info!("Got user stats for '{}'", u.username);
            u
        }
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    };

    match user.save(&connection) {
        Ok(_) => info!("Saved user stats to database"),
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    };

    let anime_list = match get_anime_list(&user, &request) {
        Ok(a) => {
            info!("Got anime list for '{}'", user.username);
            a
        }
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    };

    for a in anime_list.iter() {
        match a.save(&connection) {
            Ok(_) => info!("Saved anime '{}' to database", a.title),
            Err(e) => {
                error!("{}", e);
                exit(1);
            }
        };

        if detail == Detail::All || detail == Detail::Anime {
            let details = AnimeDetail::get(&request, a.mal_id).unwrap_or_else(|e| {
                error!("{}", e);
                exit(1);
            });

            match details.save(&connection) {
                Ok(_) => info!("Saved detailed info for '{}' to database", a.title),
                Err(e) => {
                    error!("{}", e);
                    exit(1);
                }
            };
        }

        if skip == Skip::None
            || (skip == Skip::Planned && a.watching_status != 6)
            || (skip != Skip::Anime && skip != Skip::All)
        {
            let episodes = get_anime_episodes(a.mal_id, &request).unwrap_or_else(|e| {
                error!("{}", e);
                exit(1);
            });

            episodes.iter().for_each(|e| {
                e.save(&connection).unwrap_or_else(|e| {
                    error!("{}", e);
                    exit(1);
                });
            });

            info!(
                "Saved {} episodes for anime '{}' to database",
                episodes.len(),
                a.title
            );
        }
    }

    let manga_list = match get_manga_list(&user, &request) {
        Ok(m) => {
            info!("Got manga list for '{}'", user.username);
            m
        }
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    };

    for m in manga_list.iter() {
        match m.save(&connection) {
            Ok(_) => info!("Saved manga '{}' to database", m.title),
            Err(e) => {
                error!("{}", e);
                exit(1);
            }
        };

        if detail == Detail::All || detail == Detail::Manga {
            let details = MangaDetail::get(&request, m.mal_id).unwrap_or_else(|e| {
                error!("{}", e);
                exit(1);
            });

            match details.save(&connection) {
                Ok(_) => info!("Saved detailed info for '{}' to database", m.title),
                Err(e) => {
                    error!("{}", e);
                    exit(1);
                }
            };
        }

        if skip == Skip::None
            || (skip == Skip::Planned && m.reading_status != 6)
            || (skip != Skip::Manga && skip != Skip::All)
        {
            let chapters = get_manga_chapters(m.mal_id, &request).unwrap_or_else(|e| {
                error!("{}", e);
                exit(1);
            });

            chapters.iter().for_each(|c| {
                c.save(&connection).unwrap_or_else(|e| {
                    error!("{}", e);
                    exit(1);
                });
            });

            info!(
                "Saved {} chapters for manga '{}' to database",
                chapters.len(),
                m.title
            );
        }
    }
}
