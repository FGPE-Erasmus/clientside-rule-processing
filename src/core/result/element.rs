use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};

mod internals;

#[derive(Serialize, Deserialize)]
pub(super) struct Element {
    completed: bool,
    repeat: bool,
    kind: Kind,
    selector: Selector,
    vals: Vec<String>,
    original_vals: Vec<String>
}

impl Element {
    pub(super) fn new(repeat: bool, kind: Kind, selector: Selector, vals: Vec<String>) -> Self {
        Self {
            completed: false,
            repeat,
            kind,
            selector,
            vals: Clone::clone(&vals),
            original_vals: vals
        }
    }
    pub(super) fn process(&mut self) -> (Vec<String>, &Kind, bool) {
        let amount = match self.kind {
            Kind::Reward => 2,
            Kind::Offer => 3,
            _ => 1
        };
        let data = internals::process(amount, &self.selector, &mut self.vals);
        if data.1 {
            if self.repeat {
                self.completed = false;
                self.vals = Clone::clone(&self.original_vals);
            } else {
                self.completed = true;
            }
        }
        (data.0, &self.kind, data.1)
    }
    pub(super) fn remove(&mut self, to_remove: &BTreeSet<String>) {
        self.vals.retain(|e| !to_remove.contains(e));
        if self.vals.is_empty() {
            self.completed = true;
        }
    }
    pub(super) fn is_completed(&self) -> bool {
        self.completed
    }
}

#[derive(Serialize, Deserialize)]
pub enum Kind {
    Msg, Reward, Offer, Open, Restart
}

impl FromStr for Kind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
        f.write_fmt(format_args!("{}", v))
    }
}

#[derive(Default, Serialize, Deserialize)]
pub enum Selector {
    #[default]
    All, Seq, Random, RandomOnce, Choice
}

impl FromStr for Selector {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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