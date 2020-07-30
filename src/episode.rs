use std::fmt::{self, Display, Formatter};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::result::Error;
use diesel::{Insertable, RunQueryDsl, SqliteConnection};

use crate::schema::episodes;

#[derive(Debug, Insertable)]
pub struct Episode {
    anime_id: i32,
    number: i32,
    watched_on: NaiveDateTime,
}

impl Episode {
    pub fn new(anime_id: i32, value: &str) -> Self {
        let value: Vec<&str> = value.split(" ").collect();

        let mut number = value[1].to_owned();
        number.pop();
        let number: i32 = number.parse().unwrap();

        let date = NaiveDate::parse_from_str(value[4], "%m/%d/%Y").unwrap();
        let time = NaiveTime::parse_from_str(value[6], "%R").unwrap();
        let watched_on = NaiveDateTime::new(date, time);

        Episode {
            anime_id,
            number,
            watched_on,
        }
    }

    pub fn save(&self, connection: &SqliteConnection) -> Result<(), Error> {
        diesel::insert_into(episodes::table)
            .values(self)
            .execute(connection)?;

        Ok(())
    }
}

impl Display for Episode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Ep {}, watched on {} at {}",
            self.number,
            self.watched_on.date().format("%m/%d/%Y"),
            self.watched_on.time().format("%R")
        )
    }
}
