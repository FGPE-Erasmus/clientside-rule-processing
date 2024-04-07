use crate::core::event::Event;
use crate::core::rule::Rule;

pub struct Database {
    rules: Vec<Rule>
}

impl Database {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self {
            rules
        }
    }
    pub fn process(&mut self, event: &Event) -> ProcessingResult {
        let mut fired_rules = vec!();
        let mut completed_rules = vec!();
        self.process_rules(event, &mut fired_rules, &mut completed_rules);
        ProcessingResult {
            fired_rules,
            completed_rules
        }
    }
    fn process_rules(&mut self, event: &Event, fired_rules: &mut Vec<String>, completed_rules: &mut Vec<String>) {
        for rule in self.rules.iter_mut() {
            if rule.is_completed() {
                continue;
            }
            let fired = rule.fired(event);
            if fired.is_none() {
                continue;
            }
            fired_rules.push(rule.name().to_string());
            let completed = rule.completed(event, fired.unwrap());
            if completed {
                completed_rules.push(rule.name().to_string());
                let repeat = rule.repeat();
                if repeat == -1 {
                    rule.reset(-1)
                } else if repeat > 0 {
                    rule.reset(repeat - 1)
                } else {
                    rule.set_completed();
                }
            }
        }
    }
}

pub struct ProcessingResult {
    fired_rules: Vec<String>,
    completed_rules: Vec<String>
}

impl ProcessingResult {
    pub fn fired_rules(&self) -> &Vec<String> {
        &self.fired_rules
    }
    pub fn completed_rules(&self) -> &Vec<String> {
        &self.completed_rules
    }
}