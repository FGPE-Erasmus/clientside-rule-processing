use std::collections::HashMap;
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub parts: HashMap<String, EventPartValue>
}

impl Event {
    pub fn new(parts: HashMap<String, EventPartValue>) -> Self {
        Self { parts }
    }
}

#[derive(Serialize, Deserialize)]
pub enum EventPartValue {
    DateBased(NaiveDate), NumberBased(u32), TimeBased(NaiveTime)
}