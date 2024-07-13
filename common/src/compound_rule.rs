use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::{Advancing, AdvancingResult};

mod advancing;

pub struct NamedCompoundRule {
    pub name: String,
    pub rule: CompoundRule
}

impl NamedCompoundRule {
    pub fn new(name: String, rule: CompoundRule) -> Self {
        Self { name, rule }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompoundRule {
    pub iterations: i32,
    pub every: u32,
    pub seq: CompoundRuleSeq,
    pub values: Vec<String>,
    // backup used for reset
    og_iterations: i32,
    og_every: u32,
    og_seq: CompoundRuleSeq,
    og_values: Vec<String>
}

impl CompoundRule {
    pub fn new(iterations: i32, every: u32, seq: CompoundRuleSeq, values: Vec<String>) -> Self {
        Self {
            iterations, every,
            seq: seq.clone(),
            values: values.clone(),
            og_iterations: iterations,
            og_every: every,
            og_seq: seq,
            og_values: values
        }
    }
}

impl Advancing<(&String, NaiveDate), ()> for CompoundRule {
    fn raw_advance(&mut self, data: &(&String, NaiveDate)) -> AdvancingResult<()> {
        advancing::rule_advance(self, data)
    }
    fn reset(&mut self) {
        advancing::rule_reset(self)
    }
    fn needs_reset(&self) -> bool {
        advancing::rule_needs_reset(self.iterations)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CompoundRuleSeq {
    Any, All, Order, Streak(u32, Vec<NaiveDate>)
}