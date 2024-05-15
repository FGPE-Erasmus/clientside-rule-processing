use std::error::Error;
use std::str::FromStr;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

mod internals;

#[derive(Serialize, Deserialize)]
pub(super) struct Element {
    seq: Seq,
    streak_data: Option<Vec<NaiveDate>>,
    vals: Vec<String>,
    original_vals: Vec<String>
}

impl Element {
    pub(super) fn new(seq: Seq, vals: Vec<String>) -> Self {
        let streak_data  = if let Seq::Streak(streak) = seq {
            let mut s_d = vec!();
            for _ in 0..streak {
                s_d.push(NaiveDate::default());
            }
            Some(s_d)
        } else {
            None
        };
        Self {
            seq,
            streak_data,
            vals: Clone::clone(&vals),
            original_vals: vals
        }
    }
    pub(super) fn reset(&mut self) {
        if let Seq::Streak(streak) = self.seq {
            let d = self.streak_data.as_mut().unwrap();
            d.clear();
            for _ in 0..streak {
                d.push(NaiveDate::default());
            }
        }
        self.vals = Clone::clone(&self.original_vals);
    }
    pub(super) fn fired(&mut self, data: &String, date_data: &NaiveDate) -> Option<usize> {
        internals::fired(self, data, date_data)
    }
    pub(super) fn completed(&mut self, data_pos: &Option<usize>, date_data: &NaiveDate) -> bool {
        internals::completed(self, date_data, data_pos)
    }
}

#[derive(Serialize, Deserialize)]
pub(super) enum Seq {
    Any, All, Order, Streak(usize)
}

impl FromStr for Seq {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.split("|");
        match data.next().ok_or("invalid seq")? {
            "any" => Ok(Seq::Any),
            "all" => Ok(Seq::All),
            "seq" => Ok(Seq::Order),
            "streak" => Ok(Seq::Streak(data.next().ok_or("invalid seq")?.parse()?)),
            _ => Err(Box::from("invalid seq value"))
        }
    }
}