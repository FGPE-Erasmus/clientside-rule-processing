use std::error::Error;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use chrono::{NaiveDate, NaiveTime};
use chrono::Datelike;
use regex::Regex;

use crate::core::rule::RULE_ELEMENT_REGEX;

mod internals;

const NUMERIC_ELEMENT_REGEX: &str = r"(?<sign>[<>=]*)(?<left>\d+)(?:..(?<right>\d+))?";
const DATE_ELEMENT_REGEX: &str =
    r"(?<sign>[<>=]*)(?<left>\d{4}.\d{2}.\d{2})(?:..(?<right>\d{4}.\d{2}.\d{2}))?";
const TIME_ELEMENT_REGEX: &str =
    r"(?<sign>[<>=]*)(?<left>\d{2}:\d{2})(?:..(?<right>\d{2}:\d{2}))?";

#[derive(Debug, Default)]
pub(super) struct Element<T> {
    seq: Seq,
    vals: Vec<(Border, T, Option<T>)>,
    original_vals: Vec<(Border, T, Option<T>)>
}

impl Element<u32> {
    pub(super) fn include_debug_data(&mut self, hits: u32) {
        match self.seq {
            Seq::All | Seq::Order => {
                if self.vals.is_empty() {
                    for e in 1..=hits {
                        self.vals.push((Border::Exact, e, None));
                        self.original_vals.push((Border::Exact, e, None));
                    }
                }
            }
            _ => ()
        }
    }
    pub(super) fn fired(&mut self, data: &u32) -> Option<usize> {
        internals::universal_fired(self, data)
    }
    pub(super) fn completed(&mut self, _data: &u32, data_pos: &Option<&usize>) -> bool {
        internals::universal_completed(self, data_pos)
    }
}

impl Element<NaiveDate> {
    pub(super) fn fired(&mut self, data: &NaiveDate) -> Option<usize> {
        internals::date_fired(self, data)
    }
    pub(super) fn completed(&mut self, data: &NaiveDate, data_pos: &Option<&usize>) -> bool {
        internals::date_completed(self, data, data_pos)
    }
}

impl Element<NaiveTime> {
    pub(super) fn fired(&mut self, data: &NaiveTime) -> Option<usize> {
        internals::universal_fired(self, data)
    }
    pub(super) fn completed(&mut self, _data: &NaiveTime, data_pos: &Option<&usize>) -> bool {
        internals::universal_completed(self, data_pos)
    }
}

impl<T> Element<T>
    where T: Debug,
          Element<T>: Default + FromStr,
          <Element<T> as FromStr>::Err: Debug + Display {
    pub(super) fn new_from(optional_str: Option<&&str>, rule_name: &str) -> Self {
        if optional_str.is_some() {
            let res = optional_str.unwrap().parse();
            if res.is_ok() {
                res.unwrap()
            } else {
                let err = res.unwrap_err();
                eprintln!("could not create element in rule ({rule_name}), returning default, reason: {err}");
                Default::default()
            }
        } else {
            Default::default()
        }
    }
}

impl<T: Clone> Element<T> {
    pub(super) fn reset(&mut self) {
        self.vals = Clone::clone(&self.original_vals);
    }
}

impl FromStr for Element<u32> {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_fn = |s: &str| s.parse().map_err(|e| Box::from(e));
        let mut element = parse_element(s, NUMERIC_ELEMENT_REGEX, parse_fn)?;
        filter_values(0, &mut element);
        Ok(element)
    }
}

impl FromStr for Element<NaiveDate> {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_fn = |s: &str| {
            NaiveDate::parse_from_str(s, "%Y.%m.%d").map_err(|e| Box::from(e))
        };
        let mut element = parse_element(s, DATE_ELEMENT_REGEX, parse_fn)?;
        if let Seq::Streak(streak) = element.seq {
            element.vals.clear();
            for _ in 0..streak {
                element.vals.push((Border::Exact, NaiveDate::default(), None));
            }
            element.original_vals = Clone::clone(&element.vals);
        }
        if let Seq::Selected(_) = element.seq {
            element.vals.push((Border::Exact, NaiveDate::default(), None));
            element.original_vals.push((Border::Exact, NaiveDate::default(), None));
        }
        Ok(element)
    }
}

impl FromStr for Element<NaiveTime> {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_fn = |s: &str| {
            NaiveTime::parse_from_str(s, "%H:%M").map_err(|e| Box::from(e))
        };
        Ok(parse_element(s, TIME_ELEMENT_REGEX, parse_fn)?)
    }
}

fn filter_values<T: PartialEq + Eq>(illegal_val: T, element: &mut Element<T>) {
    let indices = element.vals
        .iter()
        .enumerate()
        .fold(vec!(), |mut v, e| {
            if e.1.1.eq(&illegal_val) {
                v.push(e.0);
            }
            v
        });
    for index in indices {
        element.vals.remove(index);
        element.original_vals.remove(index);
    }
}

fn val_matches<T>(rule_data: &(Border, T, Option<T>), event_data: &T) -> bool
    where T: PartialEq + Eq + PartialOrd + Ord {
    let left_data = &rule_data.1;
    let right_data = &rule_data.2;
    match rule_data.0 {
        Border::Exact => event_data.eq(left_data),
        Border::Less => event_data.lt(left_data),
        Border::LessEq => event_data.le(left_data),
        Border::Greater => event_data.gt(left_data),
        Border::GreaterEq => event_data.ge(left_data),
        Border::Between => event_data.ge(left_data) && event_data.le(right_data.as_ref().unwrap())
    }
}

fn parse_element<T: Debug + Clone>(s: &str, regex: &str, parse_fn: impl Fn(&str) -> Result<T, Box<dyn Error>>) -> Result<Element<T>, Box<dyn Error>> {
    let element_capt = Regex::new(RULE_ELEMENT_REGEX)?
        .captures(s)
        .ok_or("could not find element parts (keyword or values)")?;
    let vals = element_capt.name("values");
    let func = element_capt.name("func");
    let func_vals = element_capt.name("func_values");
    let values_data = if vals.is_some() {
        ("any", vals.ok_or("could not find values")?.as_str())
    } else {
        (func.ok_or("could not find func type")?.as_str(),
         func_vals.ok_or("could not find func values")?.as_str())
    };
    let mut seq_data = String::from(values_data.0);
    seq_data.push_str("|");
    seq_data.push_str(values_data.1);
    let seq = seq_data.parse()?;
    let mut final_values = vec!();
    for val in values_data.1.split(",") {
        let parsed_val = parse_complex_value(val, regex, &parse_fn);
        if parsed_val.is_ok() {
            final_values.push(parsed_val.unwrap());
        } else {
            eprintln!("{}", parsed_val.unwrap_err().to_string());
        }
    }
    Ok(Element {
        seq,
        vals: Clone::clone(&final_values),
        original_vals: final_values
    })
}

fn parse_complex_value<T>(val: &str, regex: &str, parse_fn: &impl Fn(&str) -> Result<T, Box<dyn Error>>) -> Result<(Border, T, Option<T>), Box<dyn Error>> {
    let capt = Regex::new(regex)?
        .captures(val)
        .ok_or("could not find complex value")?;
    let sign = capt.name("sign");
    let left = capt.name("left");
    let right = capt.name("right");
    let left_val = parse_fn(
        left.ok_or("could not find left value for complex value")?.as_str()
    )?;
    if right.is_some() {
        let right_val = parse_fn(
            right.ok_or("could not find right value for complex value")?.as_str()
        )?;
        Ok((Border::Between, left_val, Some(right_val)))
    } else {
        let border = if sign.is_some() {
            sign.unwrap().as_str().parse()?
        } else {
            Border::Exact
        };
        Ok((border, left_val, None))
    }
}
#[derive(Debug, Default)]
enum Seq {
    #[default]
    Any, All, Order, Streak(usize), Selected(Day)
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
            "every" => Ok(Seq::Selected(data.next().ok_or("invalid seq")?.parse()?)),
            _ => Err(Box::from("invalid seq value"))
        }
    }
}

#[derive(Debug)]
enum Day {
    Everyday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
}

impl FromStr for Day {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" | "DAY" => Ok(Day::Everyday),
            "1" | "MONDAY" => Ok(Day::Monday),
            "2" | "TUESDAY" => Ok(Day::Tuesday),
            "3" | "WEDNESDAY" => Ok(Day::Wednesday),
            "4" | "THURSDAY" => Ok(Day::Thursday),
            "5" | "FRIDAY" => Ok(Day::Friday),
            "6" | "SATURDAY" => Ok(Day::Saturday),
            "7" | "SUNDAY" => Ok(Day::Sunday),
            _ => Err("invalid Day value")
        }
    }
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

#[derive(Clone, Debug)]
enum Border {
    Exact, Less, LessEq, Greater, GreaterEq, Between
}

impl FromStr for Border {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Border::Less),
            "<=" => Ok(Border::LessEq),
            ">" => Ok(Border::Greater),
            ">=" => Ok(Border::GreaterEq),
            ".." => Ok(Border::Between),
            "" => Ok(Border::Exact),
            _ => Err("invalid Border value")
        }
    }
}