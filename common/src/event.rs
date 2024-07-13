use std::collections::HashMap;
use chrono::{NaiveDate, NaiveTime};

pub struct Event {
    pub parts: HashMap<String, EventPartValue>
}

impl Event {
    pub fn new(parts: HashMap<String, EventPartValue>) -> Self {
        Self { parts }
    }
}

pub enum EventPartValue {
    DateBased(NaiveDate), NumberBased(u32), TimeBased(NaiveTime)
}