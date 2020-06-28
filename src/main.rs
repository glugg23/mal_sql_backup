use clap::{App, Arg};
use mal_backup_core::session::set_session_cookie;
use mal_backup_core::{
    get_anime_episodes, get_anime_list, get_manga_chapters, get_manga_list, get_user_stats,
};
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
        .arg(Arg::with_name("skip-planned"))
        .get_matches();

    let username = args.value_of("username").unwrap();
    let password = args.value_of("password").unwrap();
    let skip_planned = args.is_present("skip-planned");

    let client = Client::builder().cookie_store(true).build().unwrap();

    set_session_cookie(&client, username, password).expect("Failed to get session");

    let user = get_user_stats(username, &client).unwrap();

    let anime_list = get_anime_list(&user, &client).unwrap();

    for a in anime_list.iter() {
        println!("{:?}", a);

        if !(skip_planned && a.watching_status == 6) {
            let episodes = get_anime_episodes(a.mal_id, &client).unwrap();
            episodes.iter().for_each(|e| println!("{:?}", e));
        }
    }

    let manga_list = get_manga_list(&user, &client).unwrap();

    for m in manga_list.iter() {
        println!("{:?}", m);

        if !(skip_planned && m.reading_status == 6) {
            let chapters = get_manga_chapters(m.mal_id, &client).unwrap();
            chapters.iter().for_each(|c| println!("{:?}", c));
        }
    }
}
