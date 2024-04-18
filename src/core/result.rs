use std::collections::BTreeSet;
use std::error::Error;
use std::str::FromStr;

use regex::Regex;

use crate::core::result::element::{Element, Kind};

pub mod element;

pub struct Result {
    name: String,
    elements: Vec<Element>
}

impl Result {
    pub fn process(&mut self) -> Vec<(Vec<String>, &Kind, bool)> {
        self.elements
            .iter_mut()
            .fold(vec!(), |mut v, e| {
                if !e.is_completed() {
                    v.push(e.process());
                }
                v
            })
    }
    pub fn remove(&mut self, vals: Vec<String>) {
        let to_remove = BTreeSet::from_iter(vals);
        self.elements
            .iter_mut()
            .for_each(|e| e.remove(&to_remove));
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn is_completed(&self) -> bool {
        self.elements
            .iter()
            .all(|e| e.is_completed())
    }
}

const RESULT_REGEX: &str = r"(?<name>[a-zA-Z0-9_]+)\s+->\s+(?<content>.+)";
const RESULT_ELEMENT_REGEX: &str = concat!(
    r"(?<repeat>repeat\s+)?(?<kind>msg|reward|offer|open|restart)(?:\s+(?<se",
    r"lector>all|seq|random|random_once|choice))?(?:\s+(?<args>[^;]+))(?:;)?"
);

impl FromStr for Result {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let capt = Regex::new(RESULT_REGEX)?
            .captures(s)
            .ok_or("could not find result parts (name or content)")?;
        let name = capt
            .name("name")
            .ok_or("could not find result name")?
            .as_str()
            .to_string();
        let content = capt
            .name("content")
            .ok_or("could not find result content")?
            .as_str();
        let elements = Regex::new(RESULT_ELEMENT_REGEX)?
            .captures_iter(content)
            .map(|capt| {
                let repeat = capt.name("repeat").is_some();
                let kind = capt.name("kind")
                    .unwrap()
                    .as_str()
                    .parse()?;
                let selector = capt.name("selector");
                let selector = if selector.is_some() {
                    selector.unwrap().as_str().parse()?
                } else {
                    Default::default()
                };
                let args: Vec<String> = capt
                    .name("args")
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                match kind {
                    Kind::Reward => {
                        if args.len() % 2 != 0 {
                            let err: Box<dyn Error> = Box::from("wrong number of args");
                            return Err(err)
                        }
                    }
                    Kind::Offer => {
                        if args.len() % 3 != 0 {
                            return Err(Box::from("wrong number of args"))
                        }
                    }
                    _ => ()
                };
                Ok(Element::new(repeat, kind, selector, args))
            })
            .inspect(|e| {
                if let Err(err) = e {
                    eprintln!("can't parse result: {err}")
                }
            })
            .filter_map(|e| {
                e.ok()
            })
            .collect();
        Ok(Result {
            name,
            elements
        })
    }
}