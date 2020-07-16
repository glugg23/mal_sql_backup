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

use mal_sql_backup::{
    get_anime_episodes, get_anime_list, get_manga_chapters, get_manga_list, get_user_stats,
};
use mal_sql_backup::session::set_session_cookie;

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

    let connection = get_db_connection().unwrap_or_else(|e| {
        error!("{}", e);
        exit(1);
    });

    run_pending_migrations(&connection).unwrap_or_else(|e| {
        error!("{}", e);
        exit(1);
    });

    let client = Client::builder().cookie_store(true).build().unwrap();

    set_session_cookie(&client, username, password).unwrap_or_else(|e| {
        error!("{}", e);
        exit(1);
    });

    let user = get_user_stats(username, &client).unwrap_or_else(|e| {
        error!("{}", e);
        exit(1);
    });

    user.save(&connection).unwrap_or_else(|e| {
        error!("{}", e);
        exit(1);
    });

    let anime_list = get_anime_list(&user, &client).unwrap_or_else(|e| {
        error!("{}", e);
        exit(1);
    });

    for a in anime_list.iter() {
        a.save(&connection).unwrap_or_else(|e| {
            error!("{}", e);
            exit(1);
        });

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
        }
    }

    let manga_list = get_manga_list(&user, &client).unwrap_or_else(|e| {
        error!("{}", e);
        exit(1);
    });

    for m in manga_list.iter() {
        m.save(&connection).unwrap_or_else(|e| {
            error!("{}", e);
            exit(1);
        });

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
        }
    }
}
