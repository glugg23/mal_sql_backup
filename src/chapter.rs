use std::fmt::{self, Display, Formatter};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::result::Error;
use diesel::{Insertable, RunQueryDsl, SqliteConnection};

use crate::schema::chapters;

#[derive(Debug, Insertable)]
pub struct Chapter {
    manga_id: i32,
    number: i32,
    read_on: NaiveDateTime,
}

impl Chapter {
    pub fn new(manga_id: i32, value: &str) -> Self {
        let value: Vec<&str> = value.split(" ").collect();

        let mut number = value[1].to_owned();
        number.pop();
        let number: i32 = number.parse().unwrap();

        let date = NaiveDate::parse_from_str(value[4], "%m/%d/%Y").unwrap();
        let time = NaiveTime::parse_from_str(value[6], "%R").unwrap();
        let read_on = NaiveDateTime::new(date, time);

        Chapter {
            manga_id,
            number,
            read_on,
        }
    }

    pub fn save(&self, connection: &SqliteConnection) -> Result<(), Error> {
        diesel::insert_into(chapters::table)
            .values(self)
            .execute(connection)?;

        Ok(())
    }
}

impl Display for Chapter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Chapter {}, read on {} at {}",
            self.number,
            self.read_on.date().format("%m/%d/%Y"),
            self.read_on.time().format("%R")
        )
    }
}
