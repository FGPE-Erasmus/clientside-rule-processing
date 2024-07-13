use serde::{Deserialize, Serialize};
use crate::{Advancing, AdvancingResult};

mod advancing;

#[derive(Debug, Eq, PartialEq)]
pub struct NamedRuleResult {
    pub name: String,
    pub res: RuleResult
}

impl NamedRuleResult {
    pub fn new(name: String, res: RuleResult) -> Self {
        Self { name, res }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuleResult {
    pub values: Vec<RuleResultValue>
}

impl RuleResult {
    pub fn new(values: Vec<RuleResultValue>) -> Self {
        Self { values }
    }
}

impl Advancing<(), Vec<(RuleResultKind, Vec<String>)>> for RuleResult {
    fn raw_advance(&mut self, _: &()) -> AdvancingResult<Vec<(RuleResultKind, Vec<String>)>> {
        advancing::result_advance(&mut self.values)
    }
    fn reset(&mut self) {
        advancing::result_reset()
    }
    fn needs_reset(&self) -> bool {
        advancing::result_needs_reset()
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuleResultValue {
    pub iterations: i32,
    pub kind: RuleResultKind,
    pub seq: RuleResultSeq,
    pub values: Vec<String>,
    // backup used for reset
    og_values: Vec<String>
}

impl RuleResultValue {
    pub fn new(iterations: i32, kind: RuleResultKind, seq: RuleResultSeq, values: Vec<String>) -> Self {
        Self { iterations, kind, seq, values: values.clone(), og_values: values }
    }
}

impl Advancing<(), (RuleResultKind, Vec<String>)> for RuleResultValue {
    fn raw_advance(&mut self, _: &()) -> AdvancingResult<(RuleResultKind, Vec<String>)> {
        advancing::value_advance(self)
    }
    fn reset(&mut self) {
        advancing::value_reset(self)
    }
    fn needs_reset(&self) -> bool {
        advancing::value_needs_reset(self.iterations)
    }
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum RuleResultSeq {
    All, Order, Random, RandomOnce, Choice
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum RuleResultKind {
    Message, Offer, Open, Restart, Reward
}