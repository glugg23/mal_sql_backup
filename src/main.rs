use clap::{App, Arg};
use mal_backup_core::session::set_session_cookie;
use mal_backup_core::{get_anime_episodes, get_manga_chapters};
use reqwest::blocking::Client;

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
        .get_matches();

    let username = args.value_of("username").unwrap();
    let password = args.value_of("password").unwrap();

    let client = Client::builder().cookie_store(true).build().unwrap();

    set_session_cookie(&client, username, password).expect("Failed to get session");

    let episodes = get_anime_episodes(1, &client).unwrap();

    for e in episodes {
        println!("{}", e);
    }

    let chapters = get_manga_chapters(1, &client).unwrap();

    for c in chapters {
        println!("{}", c);
    }
}
