use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use regex::Regex;

mod internals;

pub struct Result {
    name: String,
    completed: bool,
    repeat: bool,
    kind: Kind,
    selector: Selector,
    vals: Vec<String>,
    original_vals: Vec<String>
}

impl Result {
    pub fn process(&mut self) -> (Vec<String>, &Kind, bool) {
        let amount = match self.kind {
            Kind::Reward => 2,
            Kind::Offer => 3,
            _ => 1
        };
        let data = internals::process(amount, &self.selector, &mut self.vals);
        (data.0, &self.kind, data.1)
    }
    pub fn reset(&mut self) {
        self.completed = false;
        self.vals = Clone::clone(&self.original_vals);
    }
    pub fn remove(&mut self, vals: Vec<String>) -> bool {
        let to_remove = BTreeSet::from_iter(vals);
        self.vals.retain(|e| !to_remove.contains(e));
        self.vals.is_empty()
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn is_completed(&self) -> bool {
        self.completed
    }
    pub fn set_completed(&mut self) {
        self.completed = true;
    }
    pub fn repeat(&self) -> bool {
        self.repeat
    }
}

const RESULT_REGEX: &str = r"(?<name>[a-zA-Z0-9_]+)\s+->\s+(?<content>.+)";
const RESULT_ELEMENT_REGEX: &str = concat!(
    r"(?<repeat>repeat\s+)?(?<kind>msg|reward|offer|open|restart)(?:\s+(",
    r"?<selector>all|seq|random|random_once|choice))?(?:\s+(?<args>.+))"
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
        let capt = Regex::new(RESULT_ELEMENT_REGEX)?
            .captures(content)
            .ok_or("could not find result parts")?;
        let repeat = capt.name("repeat").is_some();
        let kind = capt.name("kind")
            .unwrap()
            .as_str()
            .parse()?;
        let selector = capt.name("selector");
        let selector = if selector.is_some() {
            selector.unwrap().as_str().parse()?
        } else {
            Selector::All
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
                    return Err(Box::from("wrong number of args"))
                }
            }
            Kind::Offer => {
                if args.len() % 3 != 0 {
                    return Err(Box::from("wrong number of args"))
                }
            }
            _ => ()
        }
        Ok(Result {
            name,
            completed: false,
            repeat,
            kind,
            selector,
            vals: Clone::clone(&args),
            original_vals: args
        })
    }
}

pub enum Kind {
    Msg, Reward, Offer, Open, Restart
}

impl FromStr for Kind {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "msg" => Ok(Kind::Msg),
            "reward" => Ok(Kind::Reward),
            "offer" => Ok(Kind::Offer),
            "open" => Ok(Kind::Open),
            "restart" => Ok(Kind::Restart),
            _ => Err("invalid kind")
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Kind::Msg => "Msg",
            Kind::Reward => "Reward",
            Kind::Offer => "Offer",
            Kind::Open => "Open",
            Kind::Restart => "Restart"
        };
        f.write_fmt(format_args!("Kind: {}", v))
    }
}

enum Selector {
    All, Seq, Random, RandomOnce, Choice
}

impl FromStr for Selector {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "all" => Ok(Selector::All),
            "seq" => Ok(Selector::Seq),
            "random" => Ok(Selector::Random),
            "random_once" => Ok(Selector::RandomOnce),
            "choice" => Ok(Selector::Choice),
            _ => Err("invalid selector")
        }
    }
}