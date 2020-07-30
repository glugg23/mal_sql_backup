#[macro_use]
extern crate log;

use std::env;
use std::fs::File;
use std::process::exit;

use clap::{App, Arg};
use diesel::{Connection, ConnectionError, SqliteConnection};
use diesel_migrations::run_pending_migrations;
use dotenv::dotenv;
use reqwest::blocking::Client;
use simplelog::*;

use mal_sql_backup::session::set_session_cookie;
use mal_sql_backup::{
    get_anime_episodes, get_anime_list, get_manga_chapters, get_manga_list, get_user_stats,
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
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .takes_value(true)
                .required(true),
        )
        .arg(Arg::with_name("skip-planned"))
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
    let password = args.value_of("password").unwrap();
    let skip_planned = args.is_present("skip-planned");

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

    let client = Client::builder().cookie_store(true).build().unwrap();

    match set_session_cookie(&client, username, password) {
        Ok(_) => info!("Logged in as '{}'", username),
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    };

    let user = match get_user_stats(username, &client) {
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

    let anime_list = match get_anime_list(&user, &client) {
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

        if !(skip_planned && a.watching_status == 6) {
            let episodes = get_anime_episodes(a.mal_id, &client).unwrap_or_else(|e| {
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

    let manga_list = match get_manga_list(&user, &client) {
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

        if !(skip_planned && m.reading_status == 6) {
            let chapters = get_manga_chapters(m.mal_id, &client).unwrap_or_else(|e| {
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
