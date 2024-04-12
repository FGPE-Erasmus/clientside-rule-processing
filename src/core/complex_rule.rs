use std::error::Error;
use std::str::FromStr;
use chrono::NaiveDate;
use regex::Regex;
use crate::core::complex_rule::element::Element;

mod element;

pub struct ComplexRule {
    // meta
    name: String,
    repeat: bool,
    completed: bool,
    every: u32,
    original_every: u32,
    // data
    data: Element
}

impl ComplexRule {
    pub fn reset(&mut self) {
        self.completed = false;
        self.every = self.original_every;
        self.data.reset();
    }
    pub fn fired(&mut self, data: &String, date_data: &NaiveDate) -> Option<usize> {
        self.data.fired(data, date_data)
    }
    pub fn completed(&mut self, data_pos: &Option<usize>, date_data: &NaiveDate) -> bool {
        if self.data.completed(data_pos, date_data) {
            self.data.reset();
            self.every -= 1;
        }
        self.every == 0
    }
    pub fn is_completed(&self) -> bool {
        self.completed
    }
    pub fn set_completed(&mut self) {
        self.completed = true;
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn repeat(&self) -> bool {
        self.repeat
    }
}

const RULE_REGEX: &str = concat!(
    r"(?<name>[a-zA-Z0-9_]+):(?:\s+(?<repeat>repeat))?(?:\s+every\((?<every>[0-9]+)\)",
    r")?\s+(?<keyword>any|all|seq|streak)(?:\((?<streak>[0-9]+)\))?\s+(?<content>.+)"
);
const ELEMENT_REGEX: &str = r"(?<name>[a-zA-Z0-9_]+)+";

impl FromStr for ComplexRule {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let capt = Regex::new(RULE_REGEX)?
            .captures(s)
            .ok_or("could not find required complex rule parts (name/seq type/values)")?;
        let name = capt.name("name")
            .ok_or("could not find rule name")?
            .as_str()
            .to_string();
        let repeat = capt.name("repeat").is_some();
        let every = capt.name("every");
        let every = if every.is_some() {
            every.unwrap().as_str().parse()?
        } else {
            1
        };
        let seq_name = capt.name("keyword").unwrap().as_str();
        let streak = capt.name("streak");
        let seq = if streak.is_some() {
            let mut data = String::from_str(seq_name).unwrap();
            data.push_str("|");
            data.push_str(streak.unwrap().as_str());
            data.parse()?
        } else {
            seq_name.parse()?
        };
        let content = capt.name("content")
            .ok_or("could not find rule content")?
            .as_str();
        let data = Regex::new(ELEMENT_REGEX)?
            .captures_iter(content)
            .map(|c| c.name("name").unwrap().as_str().to_owned())
            .collect();
        Ok(Self {
            name,
            repeat,
            completed: false,
            every,
            original_every: every,
            data: Element::new(seq, data)
        })
    }
}