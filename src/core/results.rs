use std::error::Error;
use std::marker::PhantomData;
use std::str::FromStr;
use regex::Regex;

mod internals;

pub struct Result<'a> {
    name: String,
    completed: bool,
    repeat: bool,
    kind: Kind,
    selector: Selector,
    vals: Vec<String>,
    original_vals: Vec<String>,
    _pd: PhantomData<&'a str>
}

impl<'a> Result<'a> {
    pub fn process(&mut self) -> Vec<String> {
        match self.kind {
            Kind::Msg =>
        }
    }
    pub fn name(&self) -> &str {
        &self.name
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
        let args = capt
            .name("args")
            .unwrap()
            .as_str()
            .split_whitespace()
            .collect();
        Ok(Result { //todo verify vals by kind (msg at least 1 -> 2, reward at least 2 -> 4
            name,
            completed: false,
            repeat,
            kind,
            selector,
            vals: Clone::clone(&args),
            original_vals: args,
            _pd: PhantomData
        })
    }
}

enum Kind {
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