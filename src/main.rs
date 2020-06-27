use clap::{App, Arg};
use mal_backup_core::{get_anime_episodes, get_manga_chapters};
use reqwest::blocking::Client;

fn main() {
    let args = App::new("MAL SQL Backup")
        .version("0.1.0")
        .author("Max Leonhardt <m.leonhardt424@gmail.com>")
        .arg(
            Arg::with_name("session")
                .short("s")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let session = args.value_of("session").unwrap();

    let client = Client::new();

    let episodes = get_anime_episodes(1, session, &client).unwrap();

    for e in episodes {
        println!("{}", e);
    }

    let chapters = get_manga_chapters(1, session, &client).unwrap();

    for c in chapters {
        println!("{}", c);
    }
}
