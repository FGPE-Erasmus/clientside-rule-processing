use chrono::{NaiveDate, NaiveTime};
use chrono::Datelike;

use crate::core::{Complete, Fire};

mod internals;

pub(super) struct Element<T> {
    seq: Seq,
    vals: Vec<(Border, T, Option<T>)>,
    original_vals: Vec<(Border, T, Option<T>)>
}

impl<T: Clone> Element<T> {
    pub(super) fn reset(&mut self) {
        self.vals = Clone::clone(&self.original_vals);
    }
}

impl Fire<u32> for Element<u32> {
    fn fired(&mut self, data: &u32) -> Option<usize> {
        internals::universal_fired(self, data)
    }
}

impl Fire<NaiveDate> for Element<NaiveDate> {
    fn fired(&mut self, data: &NaiveDate) -> Option<usize> {
        internals::date_fired(self, data)
    }
}

impl Fire<NaiveTime> for Element<NaiveTime> {
    fn fired(&mut self, data: &NaiveTime) -> Option<usize> {
        internals::universal_fired(self, data)
    }
}

impl Complete<u32> for Element<u32> {
    fn completed(&mut self, _data: &u32, data_pos: &Option<usize>) -> bool {
        internals::universal_completed(self, data_pos)
    }
}

impl Complete<NaiveDate> for Element<NaiveDate> {
    fn completed(&mut self, data: &NaiveDate, data_pos: &Option<usize>) -> bool {
        internals::date_completed(self, data, data_pos)
    }
}

impl Complete<NaiveTime> for Element<NaiveTime> {
    fn completed(&mut self, _data: &NaiveTime, data_pos: &Option<usize>) -> bool {
        internals::universal_completed(self, data_pos)
    }
}

enum Seq {
    Any, All, Order, Streak(usize), Selected(Day)
}

enum Day {
    Everyday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
}

impl Day {
    fn date_matches(&self, date: &NaiveDate) -> bool {
        let days_from_mon = date.weekday().num_days_from_monday();
        let day = match self {
            Day::Everyday => days_from_mon,
            Day::Monday => 0,
            Day::Tuesday => 1,
            Day::Wednesday => 2,
            Day::Thursday => 3,
            Day::Friday => 4,
            Day::Saturday => 5,
            Day::Sunday => 6
        };
        days_from_mon as i32 - day as i32 == 0
    }
}

#[derive(Clone)]
enum Border {
    Exact, Less, LessEq, Greater, GreaterEq, Between
}