use std::collections::HashMap;
use std::fmt::{Debug};

use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

use crate::{Advancing, AdvancingResult};
use crate::event::Event;

mod advancing;

#[derive(Eq, PartialEq)]
pub struct NamedSimpleRule {
    pub name: String,
    pub rule: SimpleRule
}

impl NamedSimpleRule {
    pub fn new(name: String, rule: SimpleRule) -> Self {
        Self { name, rule }
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub struct SimpleRule {
    pub iterations: i32,
    pub parts: HashMap<String, WrappedSimpleRulePart>
}

impl SimpleRule {
    pub fn new(iterations: i32, parts: HashMap<String, WrappedSimpleRulePart>) -> Self {
        Self { iterations, parts }
    }
}

impl Advancing<Event, ()> for SimpleRule {
    fn raw_advance(&mut self, data: &Event) -> AdvancingResult<()> {
        advancing::rule_advance(self, data)
    }
    fn reset(&mut self) {
        advancing::rule_reset(self)
    }
    fn needs_reset(&self) -> bool {
        advancing::rule_needs_reset(self.iterations)
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WrappedSimpleRulePart {
    Number(SimpleRulePart<u32>),
    Time(SimpleRulePart<NaiveTime>),
    Date(SimpleRulePart<NaiveDate>)
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SimpleRulePart<T> where T: Clone {
    pub seq: SimpleRuleSeq,
    pub values: Vec<SimpleRulePartValue<T>>,
    // backup used for reset
    pub og_values: Vec<SimpleRulePartValue<T>>
}

impl<T> SimpleRulePart<T> where T: Clone {
    pub fn new(seq: SimpleRuleSeq, values: Vec<SimpleRulePartValue<T>>) -> Self {
        Self { seq, values: values.clone(), og_values: values }
    }
    pub fn empty() -> Self {
        Self { seq: SimpleRuleSeq::Any, values: Vec::new(), og_values: Vec::new() }
    }
}

impl Advancing<u32, usize> for SimpleRulePart<u32> {
    fn raw_advance(&mut self, data: &u32) -> AdvancingResult<usize> {
        advancing::part_universal_advance(self, data)
    }
    fn reset(&mut self) {
        advancing::part_reset(self)
    }
    fn needs_reset(&self) -> bool {
        advancing::part_needs_reset()
    }
}

impl Advancing<NaiveTime, usize> for SimpleRulePart<NaiveTime> {
    fn raw_advance(&mut self, data: &NaiveTime) -> AdvancingResult<usize> {
        advancing::part_universal_advance(self, data)
    }
    fn reset(&mut self) {
        advancing::part_reset(self)
    }
    fn needs_reset(&self) -> bool {
        advancing::part_needs_reset()
    }
}

impl Advancing<NaiveDate, usize> for SimpleRulePart<NaiveDate> {
    fn raw_advance(&mut self, data: &NaiveDate) -> AdvancingResult<usize> {
        advancing::part_date_advance(self, data)
    }
    fn reset(&mut self) {
        advancing::part_reset(self)
    }
    fn needs_reset(&self) -> bool {
        advancing::part_needs_reset()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SimpleRulePartValue<T> where T: Clone {
    border: SimpleRuleBorder,
    left_val: T,
    right_val: Option<T>
}

impl<T> SimpleRulePartValue<T> where T: Clone {
    pub fn exact(val: T) -> Self {
        Self::new(SimpleRuleBorder::Exact, val, None)
    }
    pub fn new(border: SimpleRuleBorder, left_val: T, right_val: Option<T>) -> Self {
        Self { border, left_val, right_val }
    }
}

impl<T> SimpleRulePartValue<T> where T: PartialEq + Eq + PartialOrd + Ord + Clone {
    fn matches(&self, data: &T) -> bool {
        let l_v = &self.left_val;
        match self.border {
            SimpleRuleBorder::Exact => data.eq(l_v),
            SimpleRuleBorder::Less => data.lt(l_v),
            SimpleRuleBorder::LessEq => data.le(l_v),
            SimpleRuleBorder::Greater => data.gt(l_v),
            SimpleRuleBorder::GreaterEq => data.gt(l_v),
            SimpleRuleBorder::Between => data.ge(l_v) && data.le(&self.right_val.as_ref().unwrap())
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SimpleRuleSeq {
    Any, All, Order, Streak(u32), Selected(u8)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SimpleRuleBorder {
    Exact, Less, LessEq, Greater, GreaterEq, Between
}