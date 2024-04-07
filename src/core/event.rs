use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use chrono::{NaiveDate, NaiveTime};
use regex::Regex;

pub struct Event {
    player: u32,
    action: u32,
    object: u32,
    location: u32,
    area: u32,
    date: NaiveDate,
    time: NaiveTime,
    result: u32
}

impl Event {
    pub fn player(&self) -> u32 {
        self.player
    }
    pub fn action(&self) -> u32 {
        self.action
    }
    pub fn object(&self) -> u32 {
        self.object
    }
    pub fn location(&self) -> u32 {
        self.location
    }
    pub fn area(&self) -> u32 {
        self.area
    }
    pub fn date(&self) -> NaiveDate {
        self.date
    }
    pub fn time(&self) -> NaiveTime {
        self.time
    }
    pub fn result(&self) -> u32 {
        self.result
    }
}

const EVENT_REGEX: &str = concat!(
    r"(?<player>\d+)\s(?<action>\d+)\s(?<object>\d+)\s(?<location>\d+)\s(?<are",
    r"a>\d+)\s(?<date>\d{4}.\d{2}.\d{2})\s(?<time>\d{2}:\d{2})\s(?<result>\d+)"
);

impl FromStr for Event {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let capt = Regex::new(EVENT_REGEX)?
            .captures(s)
            .ok_or("could not find main event parts")?;
        Ok(Event {
            player: capt.name("player").ok_or("no player in event")?.as_str().parse()?,
            action: capt.name("action").ok_or("no action in event")?.as_str().parse()?,
            object: capt.name("object").ok_or("no object in event")?.as_str().parse()?,
            location: capt.name("location").ok_or("no location in event")?.as_str().parse()?,
            area: capt.name("area").ok_or("no area in event")?.as_str().parse()?,
            date: NaiveDate::parse_from_str(capt.name("date").ok_or("no date in event")?.as_str(),"%Y.%m.%d")?,
            time: NaiveTime::parse_from_str(capt.name("time").ok_or("no time in event")?.as_str(), "%H:%M")?,
            result: capt.name("result").ok_or("no result in event")?.as_str().parse()?
        })
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Event: {} {} {} {} {} {} {} {}",
                                 self.player, self.action, self.object, self.location,
                                 self.area, self.date, self.time, self.result))
    }
}