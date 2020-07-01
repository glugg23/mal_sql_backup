use clap::{App, Arg};
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use dotenv::dotenv;
use mal_backup_core::models::{self, FavoriteAnime, FavoriteManga};
use mal_backup_core::schema;
use mal_backup_core::session::set_session_cookie;
use mal_backup_core::{
    get_anime_episodes, get_anime_list, get_manga_chapters, get_manga_list, get_user_stats,
};
use reqwest::blocking::Client;
use std::env;

fn get_db_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
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

    let username = args.value_of("username").unwrap();
    let password = args.value_of("password").unwrap();
    let skip_planned = args.is_present("skip-planned");

    let connection = get_db_connection();

    let client = Client::builder().cookie_store(true).build().unwrap();

    set_session_cookie(&client, username, password).expect("Failed to get session");

    let user = get_user_stats(username, &client).unwrap();

    let user_db: models::User = user.clone().into();
    let mut favourite_anime = Vec::new();
    let mut favourite_manga = Vec::new();

    for a in &user.favorites.anime {
        favourite_anime.push(FavoriteAnime {
            user_id: user.user_id,
            mal_id: a.mal_id,
        });
    }

    for m in &user.favorites.manga {
        favourite_manga.push(FavoriteManga {
            user_id: user.user_id,
            mal_id: m.mal_id,
        });
    }

    diesel::insert_into(schema::users::table)
        .values(user_db)
        .execute(&connection)
        .unwrap();

    diesel::insert_into(schema::favourite_anime::table)
        .values(favourite_anime)
        .execute(&connection)
        .unwrap();

    diesel::insert_into(schema::favourite_manga::table)
        .values(favourite_manga)
        .execute(&connection)
        .unwrap();

    let anime_list = get_anime_list(&user, &client).unwrap();

    for a in anime_list.iter() {
        diesel::insert_into(schema::anime::table)
            .values(a)
            .execute(&connection)
            .unwrap();

        if !(skip_planned && a.watching_status == 6) {
            let episodes = get_anime_episodes(a.mal_id, &client).unwrap();

            for e in episodes.iter() {
                diesel::insert_into(schema::episodes::table)
                    .values(e)
                    .execute(&connection)
                    .unwrap();
            }
        }
    }

    let manga_list = get_manga_list(&user, &client).unwrap();

    for m in manga_list.iter() {
        diesel::insert_into(schema::manga::table)
            .values(m)
            .execute(&connection)
            .unwrap();

        if !(skip_planned && m.reading_status == 6) {
            let chapters = get_manga_chapters(m.mal_id, &client).unwrap();

            for c in chapters.iter() {
                diesel::insert_into(schema::chapters::table)
                    .values(c)
                    .execute(&connection)
                    .unwrap();
            }
        }
    }
}
