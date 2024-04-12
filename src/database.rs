use std::collections::HashMap;
use crate::core::complex_rule::ComplexRule;
use crate::core::event::Event;
use crate::core::rule::Rule;
use crate::core::results::Result;

pub struct Database<'a> {
    rules: Vec<Rule>,
    complex_rules: Vec<ComplexRule>,
    results: HashMap<&'a str, Result<'a>>
}

impl Database {
    pub fn new(rules: Vec<Rule>, complex_rules: Vec<ComplexRule>, results: Vec<Result>) -> Self {
        let mut res_map = HashMap::new();
        results
            .into_iter()
            .for_each(|r| {
            res_map.insert(r.name(), r);
        });
        Self {
            rules,
            complex_rules,
            results: res_map
        }
    }
    pub fn process(&mut self, event: &Event) -> ProcessingResult {
        let mut fired_rules = vec!();
        let mut completed_rules = vec!();
        self.process_rules(event, &mut fired_rules, &mut completed_rules);
        self.process_complex_rules(event, &mut fired_rules, &mut completed_rules);
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
    fn process_complex_rules(&mut self, event: &Event, fired_rules: &mut Vec<String>, completed_rules: &mut Vec<String>) {
        let mut completed_complex_rules = vec!();
        for rule in completed_rules.iter_mut() {
            for complex_rule in self.complex_rules.iter_mut() {
                if complex_rule.is_completed() {
                    continue;
                }
                let fired = complex_rule.fired(rule, &event.date());
                if fired.is_none() {
                    continue;
                }
                fired_rules.push(complex_rule.name().to_string());
                let completed = complex_rule.completed(&fired, &event.date());
                if completed {
                    completed_complex_rules.push(complex_rule.name().to_string());
                    if complex_rule.repeat() {
                        complex_rule.reset();
                    } else {
                        complex_rule.set_completed();
                    }
                }
            }
        }
        completed_rules.append(&mut completed_complex_rules);
    }
    fn process_results(&self, completed_rules: &Vec<String>) {
        for completed_rule in completed_rules {

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