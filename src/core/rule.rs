use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use chrono::{NaiveDate, NaiveTime};
use regex::Regex;
use crate::Config;
use crate::core::event::Event;
use crate::core::rule::element::{Complete, Element, Fire};

mod element;

pub struct Rule {
    // meta
    name: String,
    repeat: i32,
    completed: bool,
    // elements
    player: Element<u32>,
    action: Element<u32>,
    object: Element<u32>,
    location: Element<u32>,
    area: Element<u32>,
    date: Element<NaiveDate>,
    time: Element<NaiveTime>,
    result: Element<u32>
}

impl Rule {
    fn new(name: String, repeat: i32, completed: bool,
               player: Element<u32>, action: Element<u32>, object: Element<u32>,
               location: Element<u32>, area: Element<u32>, date: Element<NaiveDate>,
               time: Element<NaiveTime>, result: Element<u32>) -> Self {
        Self { name, repeat, completed, player, action, object, location, area, date, time, result }
    }
    pub fn include_debug_data(&mut self, config: &Config) {
        self.object.include_debug_data(config.object_hits());
        self.location.include_debug_data(config.location_hits());
        self.area.include_debug_data(config.area_hits());
    }
    pub fn reset(&mut self, new_repeat: i32) {
        self.repeat = new_repeat;
        self.completed = false;

        self.player.reset();
        self.action.reset();
        self.object.reset();
        self.location.reset();
        self.area.reset();
        self.date.reset();
        self.time.reset();
        self.result.reset();
    }
    pub fn fired(&mut self, data: &Event) -> Option<Vec<usize>> {
        let player = self.player.fired(&data.player())?;
        let action = self.action.fired(&data.action())?;
        let area = self.area.fired(&data.area())?;
        let location = self.location.fired(&data.location())?;
        let object = self.object.fired(&data.object())?;
        let date = self.date.fired(&data.date())?;
        let time = self.time.fired(&data.time())?;
        let result = self.result.fired(&data.result())?;
        Some(vec!(player, action, area, location, object, date, time, result))
    }
    pub fn completed(&mut self, data: &Event, pos: Vec<usize>) -> bool {
        self.player.completed(&data.player(), &pos.get(0)) &&
            self.action.completed(&data.action(), &pos.get(1)) &&
            self.area.completed(&data.area(), &pos.get(2)) &&
            self.location.completed(&data.location(), &pos.get(3)) &&
            self.object.completed(&data.object(), &pos.get(4)) &&
            self.date.completed(&data.date(), &pos.get(5)) &&
            self.time.completed(&data.time(), &pos.get(6)) &&
            self.result.completed(&data.result(), &pos.get(7))
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn repeat(&self) -> i32 {
        self.repeat
    }
    pub fn is_completed(&self) -> bool {
        self.completed
    }
    pub fn set_completed(&mut self) {
        self.completed = true;
    }
}

const RULE_REGEX: &str = r"(?<name>[a-zA-Z0-9_]+):\s+(?<content>.+)";
const RULE_ELEMENT_REGEX: &str = concat!(
    r"(?<keyword>[a-z]+)\s+(?:(?<values>[A-Z0-9*<>=.:]+(?:,\s*[A-Z0-9*<>=.:]+)*)|",
    r"(?<func>[a-z]+)\((?<func_values>[A-Z0-9*<>=.:]+(?:,\s*[A-Z0-9*<>=.:]+)*)\))"
);

impl FromStr for Rule {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rule_capt = Regex::new(RULE_REGEX)?
            .captures(s)
            .ok_or("could not find rule parts (name or content)")?;
        let rule_name = rule_capt
            .name("name")
            .ok_or("could not find rule name")?
            .as_str();
        let rule_content = rule_capt
            .name("content")
            .ok_or("could not find rule content")?
            .as_str();
        let mut elements_data = HashMap::new();
        Regex::new(RULE_ELEMENT_REGEX)?
            .captures_iter(rule_content)
            .for_each(|c| {
                elements_data.insert(c.name("keyword")
                .unwrap()
                .as_str(), c.get(0).unwrap().as_str());
            });
        let mut repeat = -1;
        if elements_data.get("repeat").is_some() {
            let repeat_regex = Regex::new(RULE_ELEMENT_REGEX);
            if repeat_regex.is_ok() {
                let repeat_capt = repeat_regex.unwrap()
                    .captures(elements_data.get("repeat").unwrap());
                if repeat_capt.is_some() {
                    let vals = repeat_capt.unwrap().name("values");
                    if vals.is_some() {
                        let parsed = vals.unwrap().as_str().parse();
                        if parsed.is_ok() {
                            let mut v = parsed.unwrap();
                            if v > 0 {
                                v -= 1;
                            }
                            repeat = v;
                        }
                    }
                }
            }
        }
        Ok(Rule::new(
            rule_name.to_string(),
            repeat,
            false,
            Element::new_from(elements_data.get("player")),
            Element::new_from(elements_data.get("did")),
            Element::new_from(elements_data.get("with")),
            Element::new_from(elements_data.get("in")),
            Element::new_from(elements_data.get("of")),
            Element::new_from(elements_data.get("on")),
            Element::new_from(elements_data.get("at")),
            Element::new_from(elements_data.get("achieving")),
        ))
    }
}