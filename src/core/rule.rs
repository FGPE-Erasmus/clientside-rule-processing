use chrono::{NaiveDate, NaiveTime};
use crate::core::rule::element::Element;

mod element;

struct Rule {
    // meta
    name: String,
    repeat: u32,
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
    fn reset(&mut self, new_repeat: u32) {
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
}