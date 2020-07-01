use crate::schema::chapters;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::Insertable;
use std::fmt::{Display, Formatter, Result};

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
}

impl Display for Chapter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Chapter {}, read on {} at {}",
            self.number,
            self.read_on.date().format("%m/%d/%Y"),
            self.read_on.time().format("%R")
        )
    }
}
